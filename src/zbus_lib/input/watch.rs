extern crate log;
use logwatcher::{LogWatcher,LogWatcherAction};
use rhai::{FnPtr, NativeCallContext, EvalAltResult};

pub fn file_watch(context: NativeCallContext, name: String, f: FnPtr) -> Result<(), Box<EvalAltResult>> {
    match LogWatcher::register(&name) {
        Ok(mut watcher) => {
            log::debug!("Registering file {} to watch", &name);
            watcher.watch(
                &mut move |line: String| {
                    log::trace!("Log: {}", line);
                    let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (line,));
                    match r {
                        Ok(_) => {},
                        Err(err) => log::error!("Watch cb returned: {}", err),
                    }
                    LogWatcherAction::None
                }
            );
            return Result::Ok(());
        }
        Err(err) => {
            log::error!("Error watch file: {}", err);
            return Result::Ok(());
        }
    }
}
