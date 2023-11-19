extern crate log;

use zenoh::prelude::sync::*;

pub fn zenoh_put(key: String, payload: String, session: &Session) {
    match session.put(&key, payload.clone()).encoding(KnownEncoding::AppJson).res() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Metadata submission for key {} failed: {:?}", &key, err);
        }
    }
}
