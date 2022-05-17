mod overland;
mod config;
mod kindle_generator;

use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder, get};
use log::{debug, info};
use env_logger::Env;
use std::env;
use std::path::Path;
use serde::{Deserialize, Serialize};
use actix_web::web::Query;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let bind_v4_addr = env::var("BIND_ADDR").unwrap_or("127.0.0.1".to_string());
    let bind_port = u16::from_str_radix(&env::var("BIND_PORT").unwrap_or("8080".to_string()),10).unwrap_or(8080);
    info!("Starting tracking webserver on {}:{}", bind_v4_addr, bind_port);



    HttpServer::new(move || {
        App::new().service(recv_location)
            .service(kindle_image)
            .service(status)
           .service(recv_calendar)
    })
    .bind((bind_v4_addr, bind_port))?
    .run()
    .await
}


#[post("/location/{endpoint}")]
async fn recv_location(endpoint: web::Path<(String,)>, body: web::Bytes)-> impl Responder {
    let key = endpoint.into_inner().0.as_str().to_string();
    if config::is_authorized_key(&key)  {
        debug!("Posting of location to endpoint {}", &key);

        //Convert the overland info to an object, then get the last item in locations which should be the most recent
        let resp:overland::OverlandMessage = serde_json::from_str(std::str::from_utf8(&body).expect("Converting bytes for body  to string")).expect("Error parsing incoming JSON");
        let recent = resp.locations.last().expect("Error getting last location");
        debug!("{} {}",recent.geometry.coordinates[1],recent.geometry.coordinates[0]);
        //Save the location info
        let _location_info = config::location_info( recent.geometry.coordinates[1],recent.geometry.coordinates[0],&key).expect("Error loading config");
        HttpResponse::Ok().content_type("application/json").json(overland::OverlandResult {result: "ok".to_string()})
        
    }else{
        HttpResponse::Forbidden().finish()
    }
}


#[post("/location/{endpoint}/calendar/{secret}")]
async fn recv_calendar(path: web::Path<(String,String)>, body: web::Bytes)-> impl Responder {
    let (endpoint, url_secret) = path.into_inner();
    let key = endpoint.as_str().to_string();
    let secret = url_secret.as_str().to_string();
    //Limit the post size to 1 MB
    if body.len() < 1_000_000 &&config::is_authorized_key_and_calendar_secret(&key,&secret)  {
        //Save the calendar
        debug!("Updating calendar {}", &key);
        let calendar_info = std::str::from_utf8(&body).expect("Converting bytes for body  to string");
        config::save_calendar(&key,&calendar_info.to_string()).expect("Failed to save calendar info");

        //Let the uploader know it worked
        HttpResponse::Ok()
        
    }else{
        HttpResponse::Forbidden()
    }
}

#[get("/location/{endpoint}/kindle/{url_secret}")]
async fn kindle_image(path: web::Path<(String,String)>)-> impl Responder {
    let (endpoint, url_secret) = path.into_inner();
    let key = endpoint.as_str().to_string();
    let secret = url_secret.as_str().to_string();
    if config::is_authorized_key_and_secret(&key,&secret) {
        debug!("Request to Kindle endpoint {}", &key);
        //Get everything we need to load the image
        let key_conf = config::key_configuration(&key).expect("Failed to load config");
        let key_status = config::get_status(&key).expect("Failed to load status");
        let font_path = Path::new("config").join(&key).join(key_conf.font).into_os_string().into_string().expect("Failed to generate path");
       

        // Generate the image
        let image = kindle_generator::generate_status_image(key_conf.name, key_conf.title, key_status.text, font_path).expect("Failed to make image");
        HttpResponse::Ok().content_type("image/png").body(image)
    }else{
        HttpResponse::Forbidden().finish()
    }
}


#[get("/location/{endpoint}/json/{url_secret}")]
async fn status(path: web::Path<(String,String)>, params: Query<StatusParams>)-> impl Responder {
    let (endpoint, url_secret) = path.into_inner();
    let key = endpoint.as_str().to_string();
    let secret = url_secret.as_str().to_string();
    if config::is_authorized_key_and_secret(&key,&secret) {
        debug!("Request to JSON endpoint {}", &key);

        //Get everything we need to build the JSON
        let key_conf = config::key_configuration(&key).expect("Failed to load config");
        let key_status = config::get_status(&key).expect("Failed to load status");
        let mut status = key_status.text;
        let mut media_url = key_status.media_url;

        // check if we want to ignore calendar events
        if params.location_only != None && params.location_only.unwrap() == true {
            debug!("Location only JSON request; skipping calendar")
        }else{
            //If we have a calendar and a matching event, set the status to the meeting
            if let Ok(cal_status) = config::get_calendar_info(&key) {
                status = cal_status.text; 
                media_url = cal_status.media_url;
            }
        }
        
       
        //Send the JSON
        HttpResponse::Ok().content_type("application/json").json(AppJson {name: key_conf.name, title: key_conf.title, status: status ,media_url: media_url})
    }else{
        HttpResponse::Forbidden().finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppJson {
    name: String,
    title: String,
    status: String,
    media_url: String,

}

#[derive(Deserialize)]
struct StatusParams {
    location_only: Option<bool>,
}