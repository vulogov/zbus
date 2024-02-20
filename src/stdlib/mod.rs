extern crate log;

pub mod banner;
pub mod telemetry_key;
pub mod payload;
pub mod getfile;
pub mod fixedqueue;
pub mod traits;
pub mod hostname;

use crate::cmd::{Cli};


pub fn initlib(_c: &Cli) {
    log::trace!("Running STDLIB init");
}
