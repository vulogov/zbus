extern crate log;
use rhai::{Engine};
use rhai::plugin::*;
use fsio::{file};
use crate::stdlib::getfile;

pub mod bash;
pub mod distributions;
pub mod url;
pub mod socket;
pub mod ssh;

#[export_module]
pub mod input_module {
    pub fn stdin() -> String {
        getfile::get_file_from_stdin()
    }
    pub fn file(u: &str) -> String {
        match file::read_text_file(u) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error reading {}", err);
                return "".to_string();
            }
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

    engine.register_static_module("input", module.into());
}
