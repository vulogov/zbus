extern crate log;
use is_url::is_url;
use curl::easy::{Easy2, Handler, WriteError};
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn get_from_socket(_context: NativeCallContext, socket_path: String, socket_url: String) -> Result<Dynamic, Box<EvalAltResult>> {
    try_get_from_socket(socket_path, socket_url)
}


pub fn try_get_from_socket(socket_path: String, socket_url: String) -> Result<Dynamic, Box<EvalAltResult>> {
    struct Collector(Vec<u8>);

    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    if ! is_url(&socket_url) {
        return Err(format!("input::socket() url error: {}", &socket_url).into());
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    let _ = easy.useragent("ZBSCRIPT");

    match easy.get(true) {
        Ok(_) => {
            match easy.url(&socket_url) {
                Ok(_) => {
                    match easy.unix_socket(&socket_path) {
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
        Err(err) => {
            let msg = format!("input::socket() error: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}
