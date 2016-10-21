extern crate feedly_notifier;
extern crate gtk;

use gtk::prelude::*;

use std::env;

use feedly_notifier::api::Api;
use feedly_notifier::icon::Icon;

fn main() {
    let access_token = match env::var("FEEDLY_ACCESS_TOKEN") {
        Ok(value) => value,
        Err(_) => {
            println!("FEEDLY_ACCESS_TOKEN environment variable is missing.");
            std::process::exit(1);
        }
    };

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let api = Api::new(access_token.to_string());
    let icon = Icon::new(api);

    icon.update_icon();

    timeout_add_seconds(600, move || {
        icon.update_icon();
        gtk::Continue(true)
    });

    gtk::main();
}
