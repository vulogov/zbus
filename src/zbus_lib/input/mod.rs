extern crate log;
use rhai::{Engine, EvalAltResult};
use rhai::plugin::*;
use fsio::{file};
use crate::stdlib::getfile;

pub mod bash;
pub mod command;
pub mod binfile;
pub mod textfile;
pub mod distributions;
pub mod url;
pub mod socket;
pub mod ssh;
pub mod watch;

#[export_module]
pub mod input_module {
    pub fn stdin() -> String {
        getfile::get_file_from_stdin()
    }
}

pub fn file_function(_context: NativeCallContext, u: &str) -> Result<String, Box<EvalAltResult>> {
    match file::read_text_file(u) {
        Ok(res) => Ok(res),
        Err(err) => {
            log::error!("Error reading {}", err);
            return Err(format!("Error in input::file: {:?}", err).into());
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::input init");

    let mut module = exported_module!(input_module);
    module.set_native_fn("socket", socket::get_from_socket);
    module.set_native_fn("url", url::get_from_url);
    module.set_native_fn("bash", bash::run_bash);
    module.set_native_fn("ssh", ssh::ssh_command);
    module.set_native_fn("command", command::os_command);
    module.set_native_fn("watch", watch::file_watch);
    module.set_native_fn("file", file_function);

    let mut dist_module = Module::new();
    dist_module.set_native_fn("normal", distributions::norm_distribution_gen);
    dist_module.set_native_fn("uniform", distributions::uniform_distribution_gen);
    dist_module.set_native_fn("binomial", distributions::binomial_distribution_gen);
    dist_module.set_native_fn("exp", distributions::exp_distribution_gen);
    dist_module.set_native_fn("lognormal", distributions::lognormal_distribution_gen);
    dist_module.set_native_fn("sawtooth", distributions::sawtooth_gen);
    dist_module.set_native_fn("periodic", distributions::periodic_gen);
    dist_module.set_native_fn("sinusoidal", distributions::sinusoidal_gen);
    dist_module.set_native_fn("square", distributions::square_gen);
    dist_module.set_native_fn("triangle", distributions::triangle_gen);
    module.set_sub_module("distribution", dist_module);

    let mut binfile_module = Module::new();
    binfile_module.set_native_fn("read", binfile::binfile_read);
    binfile_module.set_native_fn("zip", binfile::binfile_forward);
    module.set_sub_module("binfile", binfile_module);

    let mut textfile_module = Module::new();
    textfile_module.set_native_fn("forward", textfile::textfile_forward);
    textfile_module.set_native_fn("backward", textfile::textfile_backward);
    module.set_sub_module("textfile", textfile_module);

    engine.register_static_module("input", module.into());
}
