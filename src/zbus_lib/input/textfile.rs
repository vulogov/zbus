extern crate log;
use easy_reader::EasyReader;
use std::fs::File;
use rhai::{Dynamic, Array, FnPtr, NativeCallContext, EvalAltResult};

pub fn textfile_forward(context: NativeCallContext, fname: String, f: FnPtr) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut res = Array::new();
    match File::open(fname) {
        Ok(file) => {
            match EasyReader::new(file) {
                Ok(mut reader) => {
                    let _ = reader.build_index();
                    reader.bof();
                    loop {
                        match reader.next_line() {
                            Ok(Some(line)) => {
                                match f.call_within_context(&context, (line,)) {
                                    Ok(r) => res.push(r),
                                    Err(err) => {
                                        log::debug!("input::textfile::* function returned error: {}", err);
                                        continue;
                                    }
                                }
                            }
                            Ok(None) => break,
                            _ => break,
                        }
                    }
                }
                Err(err) => return Err(format!("input::textfile::* : {}", err).into()),
            }
        }
        Err(err) => return Err(format!("input::textfile::* : {}", err).into()),
    }
    return Result::Ok(Dynamic::from(res));
}

pub fn textfile_backward(context: NativeCallContext, fname: String, f: FnPtr) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut res = Array::new();
    match File::open(fname) {
        Ok(file) => {
            match EasyReader::new(file) {
                Ok(mut reader) => {
                    let _ = reader.build_index();
                    reader.eof();
                    loop {
                        match reader.prev_line() {
                            Ok(Some(line)) => {
                                match f.call_within_context(&context, (line,)) {
                                    Ok(r) => res.push(r),
                                    Err(err) => {
                                        log::debug!("input::textfile::* function returned error: {}", err);
                                        continue;
                                    }
                                }
                            }
                            Ok(None) => break,
                            _ => break,
                        }
                    }
                }
                Err(err) => return Err(format!("input::textfile::* : {}", err).into()),
            }
        }
        Err(err) => return Err(format!("input::textfile::* : {}", err).into()),
    }
    return Result::Ok(Dynamic::from(res));
}
