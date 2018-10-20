extern crate image;

use image::GenericImageView;
use image::imageops::colorops::grayscale;
use imageproc::corners::{corners_fast9, Corner};

pub fn fast9(img: String) -> Vec<Corner> {
    let img = image::open("assets/cuadrado.png").unwrap();
    let (width, height) = img.dimensions();
    let img = grayscale(&img);
    let mut corners_img = image::GrayImage::new(width,height);
    corners_fast9(&img,0)
}