#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate serde_json;
extern crate serde;
extern crate regex;
extern crate gtk;
extern crate gdk_pixbuf;

pub mod error;
pub mod resources;
pub mod api;
pub mod icon;
