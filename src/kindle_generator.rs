
use image::{ ImageBuffer, GrayImage , Luma};
use imageproc::drawing::{draw_text_mut,text_size};
use image::imageops::{rotate90,overlay,rotate180};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use rusttype::{Font,Scale};

pub fn  generate_status_image (name: String, title: String, status: String, image_font_path: String) -> Result<Vec<u8>,String> {
    const WIDTH: u32= 758;
    const HEIGHT: u32 = 1024;
    let status_label = "Is currently:".to_string();
    //Kindle has an odd resolution
    let mut image: GrayImage  = ImageBuffer::from_fn(WIDTH,HEIGHT, |_x,_y| {
        Luma([255u8])
    });

    //Load in the font
    let mut f = File::open(image_font_path).expect("Error finding font file");
    let mut font_data = Vec::new();
    f.read_to_end(&mut font_data).expect("Error reading font file");
    let font = Font::try_from_vec(font_data).expect("Error loading font");
    // Draw the name at the top
    let font_size = 80.0;
    let mut font_scale = Scale {x: font_size, y: font_size,};


    //Add the name and title centered
    overlay(&mut image,&generate_90d_image_from_text(&name,&font,&font_scale)?,((10) as u32) as i64,((HEIGHT as i32-(text_size(font_scale,&font,&name).0))/2) as i64);
    overlay(&mut image,&generate_90d_image_from_text(&title,&font,&font_scale)?,((font_size-20.0) as u32 *2 ) as i64,((HEIGHT as i32-(text_size(font_scale,&font,&title).0))/2) as i64);

    // Add some text to say they are somewhere
    overlay(&mut image,&generate_90d_image_from_text(&status_label,&font,&font_scale)?,((WIDTH/2) as u32  ) as i64,HEIGHT as i64-(40+text_size(font_scale,&font,&status_label).0 as i64));
    // Add where they are, shrink the font size if it's too big
    if 80+text_size(font_scale,&font,&status).0 > HEIGHT as i32 {
        font_scale = Scale {x: font_size/1.5, y: font_size/1.5,};
    }
    overlay(&mut image,&generate_90d_image_from_text(&status,&font,&font_scale)?,(((WIDTH/2) as u32) +font_size as u32) as i64,HEIGHT as i64-(80+text_size(font_scale,&font,&status).0 as i64));

    //Convert to a jpg with 75% quality
    let mut jpg_cur = Cursor::new(Vec::new());
    image.write_to(&mut jpg_cur, image::ImageOutputFormat::Jpeg(75)).expect("Failed to convert to jpeg");
    Ok(jpg_cur.get_ref().to_vec())
}

pub fn generate_90d_image_from_text(text: &String, font: &Font, font_scale: &Scale)-> Result<ImageBuffer<Luma<u8>,Vec<u8>>,String> {

    //We want to show the kindle sideways
    let (w,h)= text_size(*font_scale,&font,&text);
    let mut image: GrayImage  = ImageBuffer::from_fn(w as u32,h as u32, |_x,_y| {
        Luma([255u8])
    });
    draw_text_mut(&mut image,
        Luma([0u8]),
        0,
        0,
        *font_scale,
        &font,
        &text);
    
        Ok(rotate180(&rotate90(&image)))

}