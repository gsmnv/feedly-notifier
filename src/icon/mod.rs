use gtk;
use gtk::prelude::*;
use gtk::{ StatusIcon, Menu, MenuItem };

use gdk_pixbuf::{ PixbufLoader, Pixbuf };

use std;
use std::fmt::Write;
use std::collections::HashMap;

use resources::unread_counts::UnreadCount;
use error::error_description;
use api::Api;

static NO_UPDATES_BYTES: &'static [u8] = include_bytes!("../../data/no-updates.png");
static SOME_UPDATES_BYTES: &'static [u8] = include_bytes!("../../data/some-updates.png");
static UPDATES_BYTES: &'static [u8] = include_bytes!("../../data/updates.png");

pub struct Icon {
    no_updates_image: Pixbuf,
    some_updates_image: Pixbuf,
    updates_image: Pixbuf,
    status_icon: StatusIcon,
    api: Api
}

impl Icon {
    pub fn new(api: Api) -> Self {
        let status_icon = StatusIcon::new();

        status_icon.connect_popup_menu(popup_menu);
        status_icon.set_visible(true);

        Icon {
            no_updates_image: load_pixbuf_from_bytes(NO_UPDATES_BYTES),
            some_updates_image: load_pixbuf_from_bytes(SOME_UPDATES_BYTES),
            updates_image: load_pixbuf_from_bytes(UPDATES_BYTES),
            status_icon: status_icon,
            api: api
        }
    }

    fn set_icon(&self, categories: &HashMap<String, UnreadCount>) {
        if let Some(all) = categories.get(&"All".to_string()) {
            if all.count == 0 {
                self.status_icon.set_from_pixbuf(Some(&self.no_updates_image));
            } else if all.count <= 50 {
                self.status_icon.set_from_pixbuf(Some(&self.some_updates_image));
            } else {
                self.status_icon.set_from_pixbuf(Some(&self.updates_image));
            }
        }
    }

    pub fn update_icon(&self) {
        match self.api.unread_counts() {
            Err(err) => {
                println!("{}", error_description(&err));
                std::process::exit(1);
            },
            Ok(unread_counts) => {
                let categories = unread_counts.categories();

                self.set_icon(&categories);
                set_tooltip(&categories, &self.status_icon);
            }
        }
    }
}

fn load_pixbuf_from_bytes(bytes: &[u8]) -> Pixbuf {
    let pixbuf_loader = PixbufLoader::new_with_type("png").unwrap();

    pixbuf_loader.loader_write(bytes).unwrap();
    pixbuf_loader.close().unwrap();
    pixbuf_loader.get_pixbuf().unwrap()
}

fn popup_menu(_: &StatusIcon, button: u32, time: u32) {
    let menu = Menu::new();
    let exit = MenuItem::new_with_label("Exit");

    menu.attach(&exit, 0, 1, 0, 1);

    exit.connect_activate(|_| {
        gtk::main_quit();
    });

    exit.show();

    menu.popup_easy(button, time);
}

fn set_tooltip(categories: &HashMap<String, UnreadCount>, status_icon: &StatusIcon) {
    let mut tooltip = String::new();

    for (name, category) in categories.iter() {
        writeln!(
            tooltip,
            "<span><b>{}</b>: {}</span>",
            name,
            category.count
        ).unwrap();
    }

    status_icon.set_tooltip_markup(Some(tooltip.as_str()));
}
