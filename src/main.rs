// SPDX-License-Identifier: GPL-2.0-only

mod application;
mod autostart;
mod config;
mod entry_row;
mod window;

use application::BootMateApplication;
use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};

use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    // Initialize gettext
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Failed to bind text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Failed to set text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Failed to set text domain");

    // Load resources
    gio::resources_register_include!("bootmate.gresource")
        .expect("Failed to register resources");

    // Create and run the application
    let app = BootMateApplication::new(APP_ID, &gio::ApplicationFlags::default());
    app.run()
}
