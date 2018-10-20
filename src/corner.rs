extern crate image;

use image::imageops::colorops::grayscale;
use imageproc::corners::{corners_fast9, Corner};

pub fn fast9(img: String) -> Vec<Corner> {
    let img = image::open(img).unwrap();
    let img = grayscale(&img);
    corners_fast9(&img,50)
}