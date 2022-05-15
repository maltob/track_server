mod overland;
mod config;
mod kindle_generator;

use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder, get};
use log::{debug, info};
use env_logger::Env;
use std::env;
use std::path::Path;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
   

    let bind_v4_addr = env::var("BIND_ADDR").unwrap_or("127.0.0.1".to_string());
    let bind_port = u16::from_str_radix(&env::var("BIND_PORT").unwrap_or("8080".to_string()),10).unwrap_or(8080);
    info!("Starting tracking webserver on {}:{}", bind_v4_addr, bind_port);



    HttpServer::new(move || {
        App::new().service(recv_location).service(kindle_image)
           
    })
    .bind((bind_v4_addr, bind_port))?
    .run()
    .await
}


#[post("/location/{endpoint}")]
async fn recv_location(endpoint: web::Path<(String,)>, body: web::Bytes)-> impl Responder {
    let k = endpoint.into_inner().0.as_str().to_string();
    if config::is_authorized_key(&k)  {
        debug!("{}",std::str::from_utf8(&body).unwrap());
        let resp:overland::OverlandMessage = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
        let recent = resp.locations.last().unwrap();

        //Save the location info
        let _location_info = config::location_info( recent.geometry.coordinates[0],recent.geometry.coordinates[1],&k).expect("Error loading config");
        HttpResponse::Ok().content_type("application/json").json(overland::OverlandResult {result: "ok".to_string()})
        
    }else{
        HttpResponse::Forbidden().finish()
    }
}

#[get("/location/{endpoint}/kindle/{url_secret}")]
async fn kindle_image(path: web::Path<(String,String)>)-> impl Responder {
    let (endpoint, url_secret) = path.into_inner();
    let key = endpoint.as_str().to_string();
    let secret = url_secret.as_str().to_string();
    if config::is_authorized_key_and_secret(&key,&secret) {

        //Get everything we need to load the image
        let key_conf = config::key_configuration(&key).expect("Failed to load config");
        let key_status = config::get_status(&key).expect("Failed to load status");
        let font_path = Path::new("config").join(&key).join(key_conf.font).into_os_string().into_string().expect("Failed to generate path");
       

        // Generate the image
        let image = kindle_generator::generate_status_image(key_conf.name, key_conf.title, key_status.text, font_path).expect("Failed to make image");
        HttpResponse::Ok().content_type("image/jpeg").body(image)
    }else{
        HttpResponse::Forbidden().finish()
    }
}


#[get("/location/{endpoint}/json/{url_secret}")]
async fn status(path: web::Path<(String,String)>)-> impl Responder {
    let (endpoint, url_secret) = path.into_inner();
    let key = endpoint.as_str().to_string();
    let secret = url_secret.as_str().to_string();
    if config::is_authorized_key_and_secret(&key,&secret) {
        //TODO
        HttpResponse::Ok().content_type("application/json").body("")
    }else{
        HttpResponse::Forbidden().finish()
    }
}

pub struct AppJson {
    name: String,

}