extern crate image;
extern crate imageproc;

use image::imageops::colorops::grayscale;
use imageproc::corners::Corner;

pub struct Spline{

}

pub fn algorithm(image: String, corners: &Vec<Corner>) -> Vec<Spline> {
    /* Abrir imagen */
    let image = image::open(image).unwrap(); /* O(1) */
    let image = grayscale(&image); /* O(n*m) n (ancho) m(alto) */

    /* Para cada punto ejecutamos el algoritmo genético */
    for corner in corners {
        
    }
    Vec::new()
}