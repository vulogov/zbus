extern crate log;
use curl::easy::{Easy2, Handler, WriteError};
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn get_from_url(_context: NativeCallContext, socket_url: String) -> Result<Dynamic, Box<EvalAltResult>> {
    try_get_from_url(socket_url)
}


pub fn try_get_from_url(socket_url: String) -> Result<Dynamic, Box<EvalAltResult>> {
    struct Collector(Vec<u8>);

    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    let _ = easy.useragent("ZBSCRIPT");

    match easy.get(true) {
        Ok(_) => {
            match easy.url(&socket_url) {
                Ok(_) => {
                    match easy.perform() {
                        Ok(_) => {
                            let contents = easy.get_ref();
                            return Result::Ok(Dynamic::from(String::from_utf8_lossy(&contents.0).to_string()))
                        }
                        Err(err) => {
                            let msg = format!("Request from {} returns {}", &socket_url, err);
                            log::error!("{}", &msg);
                            return Err(msg.into());
                        }
                    }

                }
                Err(err) => {
                    let msg = format!("input::socket() error: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            let msg = format!("input::socket() error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}
