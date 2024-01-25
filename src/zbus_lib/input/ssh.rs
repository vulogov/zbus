extern crate log;
use ssh::*;
use std::io::Read;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn ssh_command(_context: NativeCallContext, addr: String, cmd: String) -> Result<Dynamic, Box<EvalAltResult>> {
     match Session::new() {
         Ok(mut session) => {
             match session.set_host(&addr) {
                 Ok(_) => {
                     match session.parse_config(None) {
                         Ok(_) => {
                             match session.connect() {
                                 Ok(_) => {
                                     log::trace!("input::ssh session: {:?}",session.is_server_known());
                                     match session.userauth_publickey_auto(None) {
                                         Ok(_) => {
                                             match session.channel_new() {
                                                 Ok(mut s) => {
                                                     match s.open_session() {
                                                         Ok(_) => {
                                                             match s.request_exec(cmd.as_bytes()) {
                                                                 Ok(_) => {
                                                                     let _ = s.send_eof();
                                                                     let mut buf=Vec::new();
                                                                     match s.stdout().read_to_end(&mut buf) {
                                                                         Ok(_) => {
                                                                             match std::str::from_utf8(&buf) {
                                                                                 Ok(res) => {
                                                                                     return Result::Ok(Dynamic::from(res.to_string()));
                                                                                 }
                                                                                 Err(err) => {
                                                                                     let msg = format!("input::ssh() convert error: {}", err);
                                                                                     log::error!("{}", &msg);
                                                                                     return Err(msg.into());
                                                                                 }
                                                                             }
                                                                         }
                                                                         Err(err) => {
                                                                             let msg = format!("input::ssh() read error: {}", err);
                                                                             log::error!("{}", &msg);
                                                                             return Err(msg.into());
                                                                         }
                                                                     }
                                                                 }
                                                                 Err(err) => {
                                                                     let msg = format!("input::ssh() channel exec error: {}", err);
                                                                     log::error!("{}", &msg);
                                                                     return Err(msg.into());
                                                                 }
                                                             }
                                                         }
                                                         Err(err) => {
                                                             let msg = format!("input::ssh() channel session error: {}", err);
                                                             log::error!("{}", &msg);
                                                             return Err(msg.into());
                                                         }
                                                     }
                                                 }
                                                 Err(err) => {
                                                     let msg = format!("input::ssh() channel error: {}", err);
                                                     log::error!("{}", &msg);
                                                     return Err(msg.into());
                                                 }
                                             }
                                         }
                                         Err(err) => {
                                             let msg = format!("input::ssh() authentication error: {}", err);
                                             log::error!("{}", &msg);
                                             return Err(msg.into());
                                         }
                                     }
                                 }
                                 Err(err) => {
                                     let msg = format!("input::ssh() connect error: {}", err);
                                     log::error!("{}", &msg);
                                     return Err(msg.into());
                                 }
                             }
                         }
                         Err(err) => {
                             let msg = format!("input::ssh() parse config error: {}", err);
                             log::error!("{}", &msg);
                             return Err(msg.into());
                         }
                     }
                 }
                 Err(err) => {
                     let msg = format!("input::ssh() set host error: {}", err);
                     log::error!("{}", &msg);
                     return Err(msg.into());
                 }
             }
         }
         Err(_) => {
             let msg = format!("input::ssh() session error");
             log::error!("{}", &msg);
             return Err(msg.into());
         }
     }
}
