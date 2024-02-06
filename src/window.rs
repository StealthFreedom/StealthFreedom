use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::application::StealthFreedom;
use crate::config::{APP_ID, PROFILE};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/StealthFreedom/StealthFreedom/ui/window.ui")]
    pub struct StealthFreedomWindow {
        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        pub settings: gio::Settings,
    }

    impl Default for StealthFreedomWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StealthFreedomWindow {
        const NAME: &'static str = "StealthFreedomWindow";
        type Type = super::StealthFreedomWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action_async("win.add-link", None, |widget, _, _| async move {
                widget.add_link().await;
            });
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StealthFreedomWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for StealthFreedomWindow {}
    impl WindowImpl for StealthFreedomWindow {
        // Save window state on delete event
        fn close_request(&self) -> glib::Propagation {
            if let Err(err) = self.obj().save_window_size() {
                tracing::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for StealthFreedomWindow {}
    impl AdwApplicationWindowImpl for StealthFreedomWindow {}
}

glib::wrapper! {
    pub struct StealthFreedomWindow(ObjectSubclass<imp::StealthFreedomWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl StealthFreedomWindow {
    pub fn new(app: &StealthFreedom) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    async fn add_link(&self) {
        match self.clipboard().read_text_future().await {
            Ok(Some(s)) => tracing::info!("Clipboard contents: {s}"),
            Ok(None) => {
                tracing::warn!("No text found in the clipboard");
            }
            Err(err) => tracing::error!("Failed to paste from clipboard: {err}"),
        }
    }
}
