#[macro_use]
extern crate abscissa_core;
extern crate hyper;
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod gcp;
pub mod packer;
pub mod prelude;
pub mod unpacker;

use crate::application::APPLICATION;

fn main() {
    abscissa_core::boot(&APPLICATION);
}
