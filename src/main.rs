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
#![allow(clippy::many_single_char_names,clippy::cast_lossless)]

use imageproc::corners::Corner;

use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Button, Window, DrawingArea};

use clap::{Arg, App};

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::sync::mpsc::channel;

mod corner;
mod genetic;
mod bezier;
mod export;

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
    let lines: Rc<RefCell<Vec<bezier::Bezier>>> = Rc::new(RefCell::new(Vec::new()));

    let (tx, rx) = channel();


    if gtk::init().is_err() {
        panic!("Failed to initialize GTK");
    }
    let glade = include_str!("../assets/app.glade");
    let builder = gtk::Builder::new_from_string(glade);

    let window: Window = builder.get_object("window").unwrap();
    let drawing: DrawingArea = builder.get_object("drawingArea").unwrap();
    let clear: Button = builder.get_object("clear").unwrap();
    let fast9: Button = builder.get_object("fast9").unwrap();
    let export: Button = builder.get_object("export").unwrap();
    let go: Button = builder.get_object("go").unwrap();
    let progress: gtk::ProgressBar = builder.get_object("progress").unwrap();

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    /* Clear Button */

    let c = corners.clone();
    let l = lines.clone();
    let d = drawing.clone();
    clear.connect_clicked(move |_|{
        let corners = c.clone();
        (*corners.borrow_mut()).clear();
        let lines = l.clone();
        lines.borrow_mut().clear();
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

    /* Export as SVG */
    let l = lines.clone();
    export.connect_clicked(move |_|{
        let lines = l.clone();
        let save_dialog = gtk::FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(gtk::WindowType::Popup)),
            gtk::FileChooserAction::Save,
        );

        // Add the cancel and save buttons to that dialog.
        save_dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
        save_dialog.add_button("Save", gtk::ResponseType::Ok.into());

        if save_dialog.run() == gtk::ResponseType::Ok.into() {
            if let Some(filename) = save_dialog.get_filename(){
                export::export(&lines.borrow(),filename);
            }
        }
        save_dialog.destroy();

    });

    /* Execute algorithm */
    let c = corners.clone();
    let i = inputfile.clone();
    let tx = tx.clone();
    go.connect_clicked(move |widget| {
        let corners = c.clone();
        let corners = corners.borrow();
        let inputfile = i.clone();
        widget.set_sensitive(false);
        let cpus = num_cpus::get();
        if cpus < corners.len(){
            let corners_per_thread = corners.len() / cpus;
            let remainder_corners = corners.len() % cpus;
            for i in 0..cpus{
                let tx = tx.clone();
                let copy_corners = corners.clone();
                let inp = inputfile.clone();
                thread::spawn(move || {
                    genetic::algorithm(inp,&copy_corners[i*corners_per_thread..=(i+1)*corners_per_thread],&tx);
                });
            }
            if remainder_corners > 1{
                let tx = tx.clone();
                let copy_corners = corners.clone();
                let inp = inputfile.clone();
                thread::spawn(move || {
                    genetic::algorithm(inp,&copy_corners[cpus*corners_per_thread..],&tx);
                });
            }
        }else{
            let tx = tx.clone();
            let copy_corners = corners.clone();
            thread::spawn(move || {
                genetic::algorithm(inputfile,&copy_corners,&tx);
            });
        }
    });

    /* Drawing */

    let ifile = inputfile.clone();
    let c = corners.clone();
    let l = lines.clone();
    drawing.connect_draw(move |_widget,cr|{
        let corners = c.clone();
        let corners = corners.borrow();
        let lines = l.clone();
        let lines = lines.borrow();

        let img = gdk_pixbuf::Pixbuf::new_from_file(ifile.clone()).unwrap();
        cr.set_source_pixbuf(&img,0.0,0.0);
        cr.paint();

        cr.set_source_rgb(1.0,0.0,0.0);
        for corner in corners.iter(){
            cr.arc(corner.x as f64,corner.y as f64,CORNER_RADIUS,0.0,std::f64::consts::PI*2.0);
            cr.fill();
        }

        for line in lines.iter(){
            draw_bezier(cr,&line);
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

            if event.get_button() == 1{
                corners.borrow_mut().push(Corner{
                    x: x as u32,
                    y: y as u32,
                    score: std::f32::INFINITY,
                });
            }else{
                /* Ya no tiene sentido borrar puntos ya que dependen del orden */
                /*let mut corners = corners.borrow_mut();
                let c: Vec<Corner> = corners.iter().filter(|corner|{
                    let xc = corner.x as f64;
                    let yc = corner.y as f64;
                    ((xc-x).powi(2) + (yc-y).powi(2)).sqrt() > CORNER_RADIUS
                })
                .cloned()
                .collect();
                *corners = c;*/
            }
        }
        widget.queue_draw();
        Inhibit(true)
    });

    /* Idle */
    let g = go.clone();
    let p = progress.clone();
    let d = drawing.clone();
    let l = lines.clone();
    let c = corners.clone();
    gtk::idle_add(move ||{
        let lines = l.clone();
        let corners = c.clone();
        if let Ok(line) = rx.try_recv(){
            lines.borrow_mut().push(line);
            p.set_fraction((lines.borrow().len() as f64)/(corners.borrow().len() as f64-1.0));
            if lines.borrow().len() == corners.borrow().len()-1 {
                g.set_sensitive(true)
            }
        };
        d.queue_draw();
        gtk::Continue(true)
    });

    gtk::main();
}

fn draw_bezier(cr: &cairo::Context, line: &bezier::Bezier){
    cr.set_source_rgb(0.0,0.0,1.0);
    cr.set_line_width(3.0);
    cr.move_to(line.start.x,line.start.y);
    cr.curve_to(line.control1.x,line.control1.y,
        line.control2.x,line.control2.y,
        line.end.x,line.end.y);
    cr.stroke();
}
