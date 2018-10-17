extern crate image;
extern crate imageproc;

use image::GenericImageView;
use image::imageops::colorops::grayscale;
use imageproc::corners::corners_fast9;

fn main() {
    let img = image::open("assets/avion2.png").unwrap();
    let (width, height) = img.dimensions();
    let img = grayscale(&img);
    let mut corners_img = image::GrayImage::new(width,height);
    let corners = corners_fast9(&img,50);
    
    for corner in corners {
        println!("Corner: ({},{})\tScore: {}",corner.x,corner.y,corner.score);
        let pixel = image::Luma([255 as u8]);
        corners_img.put_pixel(corner.x,corner.y,pixel);
    }
    corners_img.save("output.png").unwrap();
}
