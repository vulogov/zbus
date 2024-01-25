extern crate log;
use duct_sh;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn run_bash(_context: NativeCallContext, c: String) -> Result<Dynamic, Box<EvalAltResult>> {
    log::warn!("input::bash() executing external command: {:?}", &c);
    let cmd = c.clone();
    match duct_sh::sh_dangerous(cmd).read() {
        Ok(res) => {
            return Result::Ok(Dynamic::from(res));
        }
        Err(err) => {
            return Err(format!("input::bash() error: {}", err).into());
        }
    }
}
