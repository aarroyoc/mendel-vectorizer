extern crate image;
extern crate imageproc;
extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate clap;

use imageproc::corners::Corner;

use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Button, Window, DrawingArea};

use clap::{Arg, App};

use std::cell::RefCell;
use std::rc::Rc;

mod corner;
mod genetic;

const CORNER_RADIUS: f64 = 5.0;

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

    let corners: Rc<RefCell<Vec<Corner>>> = Rc::new(RefCell::new(Vec::new()));


    if gtk::init().is_err() {
        panic!("Failed to initialize GTK");
    }
    let glade = include_str!("../assets/app.glade");
    let builder = gtk::Builder::new_from_string(glade);

    let window: Window = builder.get_object("window").unwrap();
    let drawing: DrawingArea = builder.get_object("drawingArea").unwrap();
    let clear: Button = builder.get_object("clear").unwrap();
    let fast9: Button = builder.get_object("fast9").unwrap();
    let go: Button = builder.get_object("go").unwrap();

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    /* Clear Button */

    let c = corners.clone();
    let d = drawing.clone();
    clear.connect_clicked(move |_|{
        let corners = c.clone();
        (*corners.borrow_mut()).clear();
        d.queue_draw();
    });

    /* FAST 9 Button */

    let c = corners.clone();
    let d = drawing.clone();
    let i = inputfile.clone();
    fast9.connect_clicked(move |_|{
        let corners = c.clone();
        let inputfile = i.clone();
        {
            let mut corners = corners.borrow_mut();
            let mut fast9 = corner::fast9(inputfile);
            corners.append(&mut fast9);
        }
        d.queue_draw();
    });

    /* Execute algorithm */
    let c = corners.clone();
    let d = drawing.clone();
    let i = inputfile.clone();
    go.connect_clicked(move |_| {
        let corners = c.clone();
        let corners = corners.borrow();
        let inputfile = i.clone();
        let lines = genetic::algorithm(inputfile,&*corners);
        for line in lines {
            //println!("Line: ");
        }
        d.queue_draw();
    });

    /* Drawing */

    let ifile = inputfile.clone();
    let c = corners.clone();
    drawing.connect_draw(move |_widget,cr|{
        let corners = c.clone();
        let corners = corners.borrow();

        let img = gdk_pixbuf::Pixbuf::new_from_file(ifile.clone()).unwrap();
        cr.set_source_pixbuf(&img,0.0,0.0);
        cr.paint();

        cr.set_source_rgb(1.0,0.0,0.0);
        for corner in corners.iter(){
            cr.arc(corner.x as f64,corner.y as f64,CORNER_RADIUS,0.0,std::f64::consts::PI*2.0);
            cr.fill();
        }

        Inhibit(false)
    });

    drawing.add_events(256);

    /* Canvas Click */
    let c = corners.clone();
    drawing.connect_button_press_event(move |widget,event|{
        let corners = c.clone();
        if event.get_event_type() == gdk::EventType::ButtonPress{
            let (x,y) = event.get_position();
            println!("Click: ({},{}) - {}",x,y,event.get_button());

            if event.get_button() == 1{
                corners.borrow_mut().push(Corner{
                    x: x as u32,
                    y: y as u32,
                    score: std::f32::INFINITY,
                });
            }else{
                let mut corners = corners.borrow_mut();
                let c: Vec<Corner> = corners.iter().filter(|corner|{
                    let xc = corner.x as f64;
                    let yc = corner.y as f64;
                    ((xc-x).powi(2) + (yc-y).powi(2)).sqrt() > CORNER_RADIUS
                })
                .cloned()
                .collect();
                *corners = c;
            }
        }
        widget.queue_draw();
        Inhibit(true)
    });

    gtk::main();
}

