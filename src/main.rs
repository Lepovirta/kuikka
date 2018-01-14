extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate pango;
extern crate chrono;

use gtk::prelude::*;
use gdk::prelude::*;
use chrono::Local;

const STYLE_TEMPLATE: &'static str = "
.osmo-root {
  margin: 20px;
  color: white;
}
.osmo-time {
  font-size: 80px;
}
.osmo-date {
  font-size: 30px;
}
";

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Geometry
    let display = gdk::Display::get_default().unwrap();
    let monitor = display.get_primary_monitor().unwrap();
    let geometry = monitor.get_geometry();
    let x_pos = geometry.width / 8;
    let y_pos = geometry.height / 8;

    // Style
    let styles = gtk::CssProvider::new();
    styles.load_from_data(STYLE_TEMPLATE.as_bytes()).unwrap();
    let screen = gdk::Screen::get_default().unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &styles,
        gtk::STYLE_PROVIDER_PRIORITY_FALLBACK
    );

    // Window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("osmo");
    window.set_app_paintable(true);
    window.set_type_hint(gdk::WindowTypeHint::Dock);
    window.set_keep_above(true);
    window.move_(x_pos, y_pos);

    // Clock
    let time_lbl = gtk::Label::new(None);
    time_lbl.get_style_context().unwrap().add_class("osmo-time");

    // Date
    let date_lbl = gtk::Label::new(None);
    date_lbl.get_style_context().unwrap().add_class("osmo-date");

    // Container
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.get_style_context().unwrap().add_class("osmo-root");
    container.add(&time_lbl);
    container.add(&date_lbl);
    window.add(&container);

    // Ticker
    let tick = move || {
        let time = format!("{}", Local::now().format("%H:%M"));
        let date = format!("{}", Local::now().format("%Y-%m-%d"));
        time_lbl.set_text(&time);
        date_lbl.set_text(&date);
        gtk::Continue(true)
    };
    tick();
    gtk::timeout_add_seconds(1, tick);

    // Kickoff
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();

    gtk::main();
}
