extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate time;

use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::enums::{FontSlant, FontWeight};
use cairo::Context;
use gdk::enums::modifier_type;

use std::sync::mpsc::sync_channel;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


fn main() {
    gtk::init().unwrap();
    let width = 500;
    let height = 500;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let drawing_area = Box::new(DrawingArea::new)();

    let mut data: Vec<String> = Vec::new();
    let mut data = Arc::new(Mutex::new(data));

    let for_drawing = data.clone();
    drawing_area.connect_draw(move |_, cr| {

        cr.select_font_face("Mono", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(14.0);

        let now = time::now();
        let mut row = 0;
        let mut column = 0;

        let data = for_drawing.lock().unwrap();
        for single_letter in data.iter() {
            put_char(cr, row, column, single_letter.to_string());
            column = column + 1;
        }

        println!("{}", time::now() - now);
        Inhibit(false)
    });

    window.set_default_size(width, height);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.add(&drawing_area);
    window.show_all();

    let mut writer = data.clone();
    let w = window.clone();
    window.connect_key_press_event(move |_, key| {
        let keyval = key.as_ref().keyval;
        let keystate = key.as_ref().state;


        let mut guy = writer.lock().unwrap();

        let key_pressed = keyval as u8 as char;
        let key_pressed = format!("{}", key_pressed);
        guy.push(key_pressed.clone());
        w.queue_draw_area(0,0,100000,10000);
        //println!("key pressed: {} / {:?}", keyval, keystate);

        //if keystate.intersects(modifier_type::ControlMask) {
        //    println!("You pressed Ctrl!");
        //}

        Inhibit(false)
    });



    gtk::main();
}

fn put_char(cairo_context: &Context, row: i64, column: i64, text: String) {
    let x = column as f64 * 7.7;
    let y = (row * 10) + 10;
    cairo_context.move_to(x, y as f64);
    cairo_context.show_text(text.as_str());
    cairo_context.set_source_rgb(1.0, 0.0, 0.0);
}
