extern crate log;
use std::env;
use std::io::Read;
use std::process::{Command};
use os_pipe;
use std::collections::HashMap;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn os_command(_context: NativeCallContext, c: String, a: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let filtered_env : HashMap<String, String> =
        env::vars().filter(|&(ref k, _)|
        k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
    ).collect();
    match os_pipe::pipe() {
        Ok((mut reader, writer)) => {
            let writer_clone = writer.try_clone().unwrap();
            let mut cmd = Command::new(c);
            cmd.env_clear();
            cmd.envs(&filtered_env);
            if ! a.is_empty() {
                cmd.arg(a);
            }
            cmd.stdout(writer);
            cmd.stderr(writer_clone);
            match cmd.spawn() {
                Ok(mut handle) => {
                    drop(cmd);
                    let mut output = String::new();
                    let _ = reader.read_to_string(&mut output);
                    match handle.wait() {
                        Ok(_) => Result::Ok(Dynamic::from(output)),
                        Err(err) => {
                            log::error!("Error waiting for process: {}", err);
                            return Result::Ok(Dynamic::from(output));
                        }
                    }
                }
                Err(err) => {
                    let msg = format!("Error executing command: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => {
            let msg = format!("Error creating pipe: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into());
        }
    }
}
