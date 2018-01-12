extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gdk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let display = gdk::Display::get_default().unwrap();
    let monitor = display.get_primary_monitor().unwrap();
    let geometry = monitor.get_geometry();
    let win_width = geometry.width / 4;
    let win_height = geometry.height / 4;
    let x_pos = geometry.width - win_width - win_width / 2;
    let y_pos = win_height / 2;

    let window = gtk::Window::new(gtk::WindowType::Popup);
    window.set_title("osmo");
    window.move_(x_pos, y_pos);
    window.set_default_size(win_width, win_height);
    let label = gtk::Label::new("Hello world!");

    window.add(&label);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
