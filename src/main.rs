extern crate image;
extern crate imageproc;
extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate clap;

use image::GenericImageView;
use image::imageops::colorops::grayscale;
use imageproc::corners::corners_fast9;

use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Button, Window, DrawingArea};

use clap::{Arg, App};

fn main() {
    let matches = App::new("Mender Vectorizer")
        .version("1.0")
        .author("Adrián Arroyo Calle <adrian.arroyocalle@gmail.com>")
        .about("Vectorizes an image using genetic algorithms")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input image")
            .required(true)
            .index(1))
        .get_matches();
    let inputfile = matches.value_of("INPUT").unwrap().to_string();
    println!("Using input file: {}", inputfile);


    if gtk::init().is_err() {
        panic!("Failed to initialize GTK");
    }
    let glade = include_str!("../assets/app.glade");
    let builder = gtk::Builder::new_from_string(glade);

    let window: Window = builder.get_object("window").unwrap();
    let drawing: DrawingArea = builder.get_object("drawingArea").unwrap();
    let go: Button = builder.get_object("go").unwrap();

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    go.connect_clicked(|_| {
        println!("Clicked!");
    });

    let ifile = inputfile.clone();
    drawing.connect_draw(move |_widget,cr|{
        let img = gdk_pixbuf::Pixbuf::new_from_file(ifile.clone()).unwrap();
        cr.set_source_pixbuf(&img,0.0,0.0);
        cr.paint();
        Inhibit(false)
    });

    drawing.add_events(256);

    drawing.connect_button_press_event(|_,event|{
        if event.get_event_type() == gdk::EventType::ButtonPress{
            let (x,y) = event.get_position();
            println!("Click: ({},{})",x,y);
        }
        Inhibit(true)
    });

    gtk::main();

    /*let img = image::open("assets/cuadrado.png").unwrap();
    let (width, height) = img.dimensions();
    let img = grayscale(&img);
    let mut corners_img = image::GrayImage::new(width,height);
    let corners = corners_fast9(&img,0);
    
    for corner in corners {
        println!("Corner: ({},{})\tScore: {}",corner.x,corner.y,corner.score);
        let pixel = image::Luma([255 as u8]);
        corners_img.put_pixel(corner.x,corner.y,pixel);
    }
    corners_img.save("output.png").unwrap();*/
}

