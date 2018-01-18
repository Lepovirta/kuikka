extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate pango;
extern crate chrono;

use gtk::prelude::*;
use gdk::prelude::*;

const STYLE_TEMPLATE: &'static str = "
.kuikka-root {
  margin: 20px;
  color: white;
}
.kuikka-time {
  font-size: 80px;
}
.kuikka-date {
  font-size: 30px;
}
";

type Coord = (i32, i32);
type DateTime = chrono::DateTime<chrono::Local>;

struct KuikkaTimeGui {
    container: gtk::Box,
    time: gtk::Label,
    date: gtk::Label,
}

impl KuikkaTimeGui {
    fn new() -> KuikkaTimeGui {
        // Time and date
        let time_lbl = gtk::Label::new(None);
        add_class(&time_lbl, "kuikka-time");
        let date_lbl = gtk::Label::new(None);
        add_class(&date_lbl, "kuikka-date");

        // Container
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
        add_class(&container, "kuikka-root");
        container.add(&time_lbl);
        container.add(&date_lbl);

        KuikkaTimeGui {
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

fn kuikka_window<W: IsA<gtk::Widget>>(
    coord: Coord,
    widget: &W
) -> gtk::Window {
    let (x_pos, y_pos) = coord;
    let w = gtk::Window::new(gtk::WindowType::Toplevel);
    w.set_title("kuikka");
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

fn run_gui() {
    // Init GTK
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        std::process::exit(1)
    }

    // Geometry
    let display = gdk::Display::get_default().unwrap();
    let monitor = display.get_primary_monitor().unwrap();
    let geometry = monitor.get_geometry();
    let coord = (geometry.width / 8, geometry.height / 8);

    // Style
    load_styles();

    // GUI
    let kuikka_time_gui = KuikkaTimeGui::new();
    let window = kuikka_window(coord, &kuikka_time_gui.container);
    window.show_all();

    // Ticker
    let tick = move || {
        kuikka_time_gui.update_datetime(chrono::Local::now());
        gtk::Continue(true)
    };
    tick();
    gtk::timeout_add_seconds(1, tick);

    // Kickoff
    gtk::main();
}

fn main() {
    run_gui();
}
