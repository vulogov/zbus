extern crate log;
pub mod banner;
use crate::cmd::{Cli};


pub fn initlib(_c: &Cli) {
    log::trace!("Running STDLIB init");
}
