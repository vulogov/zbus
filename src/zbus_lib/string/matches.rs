extern crate log;
use voca_rs::*;
use crate::zbus_lib::string::Text;

impl Text {
    pub fn matches(&mut self, t: String) -> bool {
        query::matches(&self.raw(), &t, 0)
    }
}
