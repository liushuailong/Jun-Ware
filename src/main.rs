extern crate getopts;
extern crate image;

use std::default::Default;
use std::io::{Read, BufWriter};
use std::fs::File;

pub mod html;
pub mod dom;
pub mod css;
pub mod style;
pub mod layout;
pub mod painting;
pub mod pdf;

fn main() {
    println!("Hello, world!");
}
