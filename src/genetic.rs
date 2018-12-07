/*
 *  This file is part of Mendel Vectorizer.
 *
 *  Mendel Vectorizer is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mendel Vectorizer is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mendel Vectorizer.  If not, see <https://www.gnu.org/licenses/>.
*/



use image::GenericImageView;
use image::GrayImage;
use image::imageops::colorops::grayscale;
use imageproc::corners::Corner;

use crate::bezier::{Point,Bezier};

use rand::prelude::*;
use rand::Rng;
use rand::distributions::Normal;

use std::sync::mpsc::Sender;

const GOOD_ONES: usize = 500;

pub fn algorithm(image: String, corners: &[Corner], tx: &Sender<Bezier>) {
    /* Abrir imagen */
    let image = image::open(image).unwrap(); /* O(1) */
    let image = grayscale(&image); /* O(n*m) n (ancho) m(alto) */

    /* Para cada punto ejecutamos el algoritmo genético con el siguiente punto */
    for i in 0..corners.len()-1 {
        let start_corner = corners[i];
        let start = Point { x: start_corner.x as f64, y: start_corner.y as f64};
        let end_corner = if corners.len() == i+1 {
            corners[0]
        }else{
            corners[i+1]
        };
        let end = Point { x: end_corner.x as f64, y: end_corner.y as f64};
        
        // INITIAL POPULATION
        let mut population = Vec::new();
        let mut rng = thread_rng();
        let distancia = start.distance(&end);
        for _ in 0..1000 {
            let xrand: f64 = rng.gen_range(-distancia,distancia);
            let yrand: f64 = rng.gen_range(-distancia,distancia);
            let mut control1 = start.middle(&end);
            control1.x += xrand;
            control1.y += yrand;
            let mut control2 = start.middle(&end);
            control2.x += xrand;
            control2.y += yrand;
            population.push(Bezier {start, end, control1, control2});
        }
        // SELECTION
        let mut population = natural_selection(&image,population);

        
        while evaluate(&image,&population[0]) < 80.0 {
            println!("BEST: {}",evaluate(&image,&population[0]));
            // CROSSOVER
            // Blend o Linear (Blend) https://engineering.purdue.edu/~sudhoff/ee630/Lecture04.pdf
            let mut i: usize = 0;
            let mut babies = Vec::new();
            while i < GOOD_ONES{
                // PROBABILIDAD CROSSOVER 100%, pero se mantienen los anteriores
                // 250 extra
                let line1 = &population[i];
                let line2 = &population[i+1];

                let min_x = line1.control1.x.min(line2.control1.x);
                let max_x = line1.control1.x.max(line2.control1.x);
                let min_y = line1.control1.y.min(line2.control1.y);
                let max_y = line1.control1.y.max(line2.control1.y);
                let control1 = Point{
                    x: rng.gen_range(min_x,max_x),
                    y: rng.gen_range(min_y,max_y),
                };

                let min_x = line1.control2.x.min(line2.control2.x);
                let max_x = line1.control2.x.max(line2.control2.x);
                let min_y = line1.control2.y.min(line2.control2.y);
                let max_y = line1.control2.y.max(line2.control2.y);
                let control2 = Point{
                    x: rng.gen_range(min_x,max_x),
                    y: rng.gen_range(min_y,max_y),
                };

                babies.push(Bezier{start,end,control1,control2});

                i += 2;
            }
            population.append(&mut babies);

            // MUTATION
            // TASA DE MUTACION DEL 25%
            population = population
            .into_iter()
            .map(|mut line|{
                if rng.gen::<f64>() < 0.10 {
                    let normal = Normal::new(0.0,distancia/2.0);
                    let mutation_where: u32 = rng.gen_range(1,5);
                    // Solo muta un gen, respecto a una Normal
                    match mutation_where {
                        1 => line.control1.x += rng.sample(normal),
                        2 => line.control1.y += rng.sample(normal),
                        3 => line.control2.x += rng.sample(normal),
                        4 => line.control2.y += rng.sample(normal),
                        _ => ()
                    }
                }
                line
            })
            .collect();

            // VOLVER A EVALUAR
            population = natural_selection(&image,population);
        }
        println!("Correct: {}",evaluate(&image,&population[0]));
        tx.send(population[0].clone()).unwrap();
    }
}

pub fn natural_selection(image: &GrayImage,mut population: Vec<Bezier>) -> Vec<Bezier>{
    population.sort_by(|a,b|{
        let a = evaluate(&image,&a);
        let b = evaluate(&image,&b);
        b.partial_cmp(&a).unwrap()
    });

    population.into_iter()
    .take(GOOD_ONES)
    .collect()
}


pub fn evaluate(image: &GrayImage, line: &Bezier) -> f64{
    let mut eval = 0.0;
    for point in line.iter() {
        let x = point.x as u32;
        let y = point.y as u32;
        if image.in_bounds(x,y){
            let pixel = image.get_pixel(x,y);
            if pixel.data[0] < 200{
                eval += 1.0;
            }else{
                eval -= 100.0;
            }
        }else{
            eval -= 100.0;
        }
    }
    eval
}
