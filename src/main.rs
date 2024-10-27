use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use gettext::gettext;
use gtk::glib;

const APP_ID: &str = "de.swsnr.gtk-rs-audit";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(gettext("My GNOME App"))
        .build();

    // Present window
    window.present();
}
