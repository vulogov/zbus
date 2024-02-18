extern crate log;
use rhai::{Engine, Dynamic, EvalAltResult};
use rhai::plugin::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use thread_manager::ThreadManager;

use crate::cmd;
use crate::zbus_lib;

lazy_static! {
    static ref THREADS: Mutex<ThreadManager<()>> = {
        let e: Mutex<ThreadManager<()>> = Mutex::new(ThreadManager::<()>::new(4));
        e
    };
}

pub fn terminale_all() {
    let t = THREADS.lock().unwrap();
    log::debug!("Terimating managed threads");
    t.terminate_all();
    drop(t);
}

#[export_module]
pub mod threads_module {

}

fn threads_active_threads(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match THREADS.lock() {
        Ok(t) => {
            let res = t.active_threads();
            drop(t);
            return Ok(Dynamic::from(res as i64));
        }
        Err(err) => return Err(format!("Error accessing Thread Manager: {:?}", err).into()),
    }
}

fn threads_busy_threads(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match THREADS.lock() {
        Ok(t) => {
            let res = t.busy_threads();
            drop(t);
            return Ok(Dynamic::from(res as i64));
        }
        Err(err) => return Err(format!("Error accessing Thread Manager: {:?}", err).into()),
    }
}

fn threads_waiting_threads(_context: NativeCallContext) -> Result<Dynamic, Box<EvalAltResult>> {
    match THREADS.lock() {
        Ok(t) => {
            let res = t.waiting_threads();
            drop(t);
            return Ok(Dynamic::from(res as i64));
        }
        Err(err) => return Err(format!("Error accessing Thread Manager: {:?}", err).into()),
    }
}

fn threads_spawn_thread(_context: NativeCallContext, c: String, label: String) -> Result<(), Box<EvalAltResult>> {
    threads_spawn_code(c, label)
}

pub fn threads_spawn_code(c: String, l: String) -> Result<(), Box<EvalAltResult>> {
    match THREADS.lock() {
        Ok(t) => {
            t.execute(move ||
            {
                zbus_lib::run_zbus_script_no_cli(c.clone(), l.clone());
            });
            drop(t);
            return Ok(());
        }
        Err(err) => return Err(format!("Error accessing Thread Manager: {:?}", err).into()),
    }
}

pub fn init(engine: &mut Engine, c: &cmd::Cli) {
    log::trace!("Running STDLIB::threads init");
    let mut t = THREADS.lock().unwrap();
    log::debug!("Thread engine has been configured with {} threads", &c.n);
    t.resize(c.n);
    drop(t);
    let mut module = exported_module!(threads_module);
    module.set_native_fn("active", threads_active_threads);
    module.set_native_fn("busy", threads_busy_threads);
    module.set_native_fn("waiting", threads_waiting_threads);
    module.set_native_fn("execute", threads_spawn_thread);
    engine.register_static_module("threads", module.into());
}

pub fn init_no_cli(engine: &mut Engine) {
    log::trace!("Running STDLIB::threads init");
    let mut module = exported_module!(threads_module);
    module.set_native_fn("active", threads_active_threads);
    module.set_native_fn("busy", threads_busy_threads);
    module.set_native_fn("waiting", threads_waiting_threads);
    module.set_native_fn("execute", threads_spawn_thread);
    engine.register_static_module("threads", module.into());
}
