extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate pango;
extern crate chrono;

use gtk::prelude::*;
use gdk::prelude::*;

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

type Coord = (i32, i32);
type DateTime = chrono::DateTime<chrono::Local>;

struct OsmoTimeGui {
    container: gtk::Box,
    time: gtk::Label,
    date: gtk::Label,
}

impl OsmoTimeGui {
    fn new() -> OsmoTimeGui {
        // Time and date
        let time_lbl = gtk::Label::new(None);
        add_class(&time_lbl, "osmo-time");
        let date_lbl = gtk::Label::new(None);
        add_class(&date_lbl, "osmo-date");

        // Container
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
        add_class(&container, "osmo-root");
        container.add(&time_lbl);
        container.add(&date_lbl);

        OsmoTimeGui {
            container: container,
            time: time_lbl,
            date: date_lbl,
        }
    }

    fn update_datetime(&self, dt: DateTime) {
        let time = format!("{}", dt.format("%H:%M"));
        let date = format!("{}", dt.format("%Y-%m-%d"));
        self.time.set_text(&time);
        self.date.set_text(&date);
    }
}

fn add_class<W: IsA<gtk::Widget> + gtk::WidgetExt>(
    widget: &W,
    class: &str
) {
    match widget.get_style_context() {
        Some(ctx) => ctx.add_class(class),
        None => eprintln!("Failed to get style context to add class {}", class),
    }
}

fn osmo_window<W: IsA<gtk::Widget>>(
    coord: Coord,
    widget: &W
) -> gtk::Window {
    let (x_pos, y_pos) = coord;
    let w = gtk::Window::new(gtk::WindowType::Toplevel);
    w.set_title("osmo");
    w.set_app_paintable(true);
    w.set_type_hint(gdk::WindowTypeHint::Dock);
    w.set_keep_above(true);
    w.move_(x_pos, y_pos);
    w.stick();
    w.add(widget);
    w.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    w
}

fn load_styles() {
    let styles = gtk::CssProvider::new();
    styles.load_from_data(STYLE_TEMPLATE.as_bytes()).unwrap();
    let screen = gdk::Screen::get_default().unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &styles,
        gtk::STYLE_PROVIDER_PRIORITY_FALLBACK
    );
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Geometry
    let display = gdk::Display::get_default().unwrap();
    let monitor = display.get_primary_monitor().unwrap();
    let geometry = monitor.get_geometry();
    let coord = (geometry.width / 8, geometry.height / 8);

    // Style
    load_styles();

    // GUI
    let osmo_time_gui = OsmoTimeGui::new();
    let window = osmo_window(coord, &osmo_time_gui.container);
    window.show_all();

    // Ticker
    let tick = move || {
        osmo_time_gui.update_datetime(chrono::Local::now());
        gtk::Continue(true)
    };
    tick();
    gtk::timeout_add_seconds(1, tick);

    // Kickoff
    gtk::main();
}
