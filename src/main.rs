extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate time;

use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::enums::{FontSlant, FontWeight};
use cairo::Context;
use gdk::enums::modifier_type;

fn main() {
    gtk::init().unwrap();
    let width = 500;
    let height = 500;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(|_, cr| {

        cr.select_font_face("Mono", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(14.0);

        let now = time::now();

        for column in 0..100 {
            for row in 0..100 {
                put_char(cr, row, column, "a".to_string());
            }
        }

        println!("{}", time::now() - now);
        Inhibit(false)
    });

    window.set_default_size(width, height);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        let keyval = key.as_ref().keyval;
        let keystate = key.as_ref().state;

        println!("key pressed: {} / {:?}", keyval, keystate);

        if keystate.intersects(modifier_type::ControlMask) {
            println!("You pressed Ctrl!");
        }

        Inhibit(false)
    });

    window.add(&drawing_area);
    window.show_all();

    gtk::main();
}

fn put_char(cairo_context: &Context, row: i64, column: i64, text: String) {
    let x = column as f64 * 7.7;
    let y = (row * 10) + 10;
    cairo_context.move_to(x, y as f64);
    cairo_context.show_text(text.as_str());
    cairo_context.set_source_rgb(1.0, 0.0, 0.0);
}
