extern crate image;
extern crate imageproc;

use image::GrayImage;
use image::imageops::colorops::grayscale;
use imageproc::corners::Corner;

use bezier::{Point,Bezier};


pub fn algorithm(image: String, corners: &Vec<Corner>) -> Vec<Bezier> {
    /* Abrir imagen */
    let image = image::open(image).unwrap(); /* O(1) */
    let image = grayscale(&image); /* O(n*m) n (ancho) m(alto) */
    let mut lines = Vec::new();

    /* Para cada punto ejecutamos el algoritmo genético */
    for start_corner in corners {
        let start = Point { x: start_corner.x as f64, y: start_corner.y as f64};
        for end_corner in corners {
            if start_corner.x == end_corner.x && start_corner.y == end_corner.y {
                continue;
            }
            let end = Point { x: end_corner.x as f64, y: end_corner.y as f64};
            // Try to join these two points
            let control1 = Point { x: (start.x+end.x)/2.0, y: (start.y+end.y)/2.0};
            let control2 = control1;
            let bezier = Bezier { start, end, control1, control2};
            lines.push(bezier);

        }

    }
    lines
}



pub fn evaluate(image: &GrayImage, line: Bezier) -> f64{
    let mut eval = 0.0;
    for point in line.iter() {
        let x = point.x as u32;
        let y = point.y as u32;
        let pixel = image.get_pixel(x,y);
        if pixel.data[0] > 10{
            eval += 1.0;
        }else{
            eval -= 1.0;
        }
    }
    eval
}