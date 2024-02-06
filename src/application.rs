use std::env;
use std::path::Path;
use std::process::Command;

use gettextrs::gettext;
use indoc::formatdoc;
use itertools::Itertools;
use os_release_rs::OsRelease;
use sysinfo::System;
use tracing::{debug, info};

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::StealthFreedomWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use std::cell::OnceCell;

    #[derive(Debug, Default)]
    pub struct StealthFreedom {
        pub window: OnceCell<WeakRef<StealthFreedomWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StealthFreedom {
        const NAME: &'static str = "StealthFreedom";
        type Type = super::StealthFreedom;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for StealthFreedom {}

    impl ApplicationImpl for StealthFreedom {
        fn activate(&self) {
            debug!("GtkApplication<StealthFreedom>::activate");
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = StealthFreedomWindow::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("GtkApplication<StealthFreedom>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for StealthFreedom {}
    impl AdwApplicationImpl for StealthFreedom {}
}

glib::wrapper! {
    pub struct StealthFreedom(ObjectSubclass<imp::StealthFreedom>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl StealthFreedom {
    fn main_window(&self) -> StealthFreedomWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn show_about_dialog(&self) {
        let dialog = adw::AboutWindow::builder()
            .transient_for(&self.main_window())
            .modal(true)
            // Main info:
            .application_icon(APP_ID)
            .application_name(gettext("Stealth Freedom"))
            // Legal:
            .license_type(gtk::License::MitX11)
            .copyright("Copyright © 2024 ewokthepreparer")
            // Contributors:
            .developer_name(gettext("ewokthepreparer"))
            .developers(vec!["ewokthepreparer"])
            .designers(vec!["ewokthepreparer"])
            // Translators: write your names here in this format (one name per line): "Name <email>".
            // They will be shown in the About Dialog.
            .translator_credits(gettext("translator-credits"))
            // Links:
            .website("https://github.com/StealthFreedom/StealthFreedom")
            .issue_url("https://github.com/StealthFreedom/StealthFreedom/issues")
            .support_url("https://github.com/StealthFreedom/StealthFreedom/discussions")
            // Release info:
            .version(VERSION)
            // Debug info:
            .debug_info(debug_info())
            .debug_info_filename("debug.txt")
            .build();

        dialog.present();
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("Stealth Freedom ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl Default for StealthFreedom {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property(
                "resource-base-path",
                "/io/github/StealthFreedom/StealthFreedom/",
            )
            .build()
    }
}

fn debug_info() -> String {
    let is_flatpak = Path::new("/.flatpak-info").is_file();
    let flatpak_runtime = if is_flatpak {
        OsRelease::new().map_or_else(
            |_| "\n- Flatpak Runtime: unknown".to_owned(),
            |os_release| format!("\n- Flatpak Runtime: {}", os_release.pretty_name),
        )
    } else {
        String::new()
    };
    let os_release = if is_flatpak {
        OsRelease::new_from("/run/host/os-release")
    } else {
        OsRelease::new()
    };
    let distribution = if let Ok(os_release) = os_release {
        if os_release.pretty_name.is_empty() {
            String::from("unknown")
        } else {
            os_release.pretty_name
        }
    } else {
        String::from("unknown")
    };

    let desktop_session = env::var("DESKTOP_SESSION").unwrap_or_else(|_| "unknown".to_owned());
    let display_server = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_owned());

    let kernel = Command::new("uname")
        .args(["-r", "-s"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let kernel = std::str::from_utf8(&kernel).unwrap_or("unknown");
    let kernel = kernel.strip_suffix('\n').unwrap_or(kernel);

    let arch = Command::new("uname")
        .args(["-m"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let arch = std::str::from_utf8(&arch).unwrap_or("unknown");
    let arch = arch.strip_suffix('\n').unwrap_or(arch);

    let mut sys = System::new();
    sys.refresh_cpu();
    sys.refresh_memory();
    let cpu_name =
        sys.cpus()
            .iter()
            .map(sysinfo::Cpu::brand)
            .unique()
            .fold(String::new(), |mut acc, e| {
                acc.push_str(e);
                acc.push_str(" / ");
                acc
            });
    let cpu_name = cpu_name.strip_suffix(" / ").unwrap_or(&cpu_name);
    let cpu_cores_count = sys
        .physical_core_count()
        .map_or(String::from("unknown"), |c| c.to_string());
    let cpu_threads_count = sys.cpus().len().to_string();
    let used_memory = sys.used_memory() / 1024 / 1024;
    let total_memory = sys.total_memory() / 1024 / 1024;
    let used_swap = sys.used_swap() / 1024 / 1024;
    let total_swap = sys.total_swap() / 1024 / 1024;

    let gtk_version = format!(
        "{}.{}.{}",
        gtk::major_version(),
        gtk::minor_version(),
        gtk::micro_version()
    );
    let adw_version = format!(
        "{}.{}.{}",
        adw::major_version(),
        adw::minor_version(),
        adw::micro_version()
    );

    formatdoc!(
        "- App: {APP_ID}
         - App version: {VERSION}

         - Runs in Flatpak: {is_flatpak}{flatpak_runtime}
         - Kernel version: {kernel}
         - Architecture: {arch}
         - Distribution: {distribution}
         - Desktop Session: {desktop_session}
         - Display Server: {display_server}

         - CPU: {cpu_name} {cpu_cores_count}C/{cpu_threads_count}T
         - Used memory: {used_memory} MiB / {total_memory} MiB
         - Used swap: {used_swap} MiB / {total_swap} MiB

         - GTK version: {gtk_version}
         - libadwaita version: {adw_version}"
    )
}
