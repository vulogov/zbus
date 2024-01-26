extern crate log;
extern crate binary_rw;

use std::fs::File;
use rhai::{Dynamic, Array, FnPtr, NativeCallContext, EvalAltResult};

use binary_rw::{FileStream, BinaryReader, Endian};

pub fn binfile_read(_context: NativeCallContext, fname: String, n: i64) -> Result<Dynamic, Box<EvalAltResult>> {
    match File::open(fname) {
        Ok(file) => {
            let mut res = Array::new();
            let mut fs = FileStream::new(file);
            let mut binary_file = BinaryReader::new(&mut fs, Endian::Big);
            loop {
                match binary_file.read_bytes(n as usize) {
                    Ok(buf) => {
                        res.push(Dynamic::from_blob(buf));
                    }
                    Err(_) => break,
                }
            }
            return Result::Ok(Dynamic::from(res));
        }
        Err(err) => return Err(format!("input::binfile::* : {}", err).into()),
    }
}

pub fn binfile_forward(context: NativeCallContext, fname: String, n: i64, f: FnPtr) -> Result<Dynamic, Box<EvalAltResult>> {
    match File::open(fname) {
        Ok(file) => {
            let mut res = Array::new();
            let mut fs = FileStream::new(file);
            let mut binary_file = BinaryReader::new(&mut fs, Endian::Big);
            loop {
                match binary_file.read_bytes(n as usize) {
                    Ok(buf) => {
                        match f.call_within_context(&context, (Dynamic::from_blob(buf),)) {
                            Ok(r) => res.push(r),
                            Err(err) => {
                                log::debug!("input::binfile::* function returned error: {}", err);
                                continue;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            return Result::Ok(Dynamic::from(res));
        }
        Err(err) => return Err(format!("input::binfile::* : {}", err).into()),
    }
}
