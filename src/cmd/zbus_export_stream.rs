extern crate log;
use std::thread;
use std::sync::Arc;
use serde_json::{Deserializer, Value};
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use tiny_http::{Method};

fn run_events(content: String, c: cmd::Cli, stream_cmd: cmd::Stream, zc: Config) {
    let stream = Deserializer::from_str(&content).into_iter::<Value>();

    for value in stream {
        match value {
            Ok(jvalue) => {
                if stream_cmd.stdout {
                    println!("{}", serde_json::to_string_pretty(&jvalue).unwrap());
                }
            }
            Err(err) => log::error!("zabbix streamer catch an error during JSON processing: {:?}", err),
        }
    }
}

fn run_history(content: String, c: cmd::Cli, stream_cmd: cmd::Stream, zc: Config) {
    let stream = Deserializer::from_str(&content).into_iter::<Value>();

    for value in stream {
        match value {
            Ok(jvalue) => {
                if stream_cmd.stdout {
                    println!("{}", serde_json::to_string_pretty(&jvalue).unwrap());
                }
            }
            Err(err) => log::error!("zabbix streamer catch an error during JSON processing: {:?}", err),
        }
    }
}

pub fn run(c: &cmd::Cli, stream_cmd: &cmd::Stream, zc: Config)  {
    log::trace!("zbus_export_stream::run() reached");
    match tiny_http::Server::http(stream_cmd.listen.clone()) {
        Ok(server) => {
            let mut guards = Vec::with_capacity(stream_cmd.threads.into());
            let server = Arc::new(server);
            for _ in 0..stream_cmd.threads {
                let server = server.clone();
                let stream_cmd = stream_cmd.clone();
                let c = c.clone();
                let zc = zc.clone();
                let guard = thread::spawn(move || {
                    loop {
                        match server.recv() {
                            Ok(mut request) => {
                                if request.body_length() > Some(0) {
                                    let mut content = String::new();
                                    match request.as_reader().read_to_string(&mut content) {
                                        Ok(_) => {
                                            match request.method() {
                                                Method::Post => {
                                                    if stream_cmd.history {
                                                        run_history(content, c.clone(), stream_cmd.clone(), zc.clone());
                                                    } else if stream_cmd.events {
                                                        run_events(content, c.clone(), stream_cmd.clone(), zc.clone());
                                                    } else {
                                                        log::error!("Zabbix stream catcher is not recognizing how to process this data");
                                                    }
                                                }
                                                _ => {
                                                    let response = tiny_http::Response::empty(422);
                                                    let _ = request.respond(response);
                                                    continue;
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            log::error!("Error getting request body: {:?}", err);
                                            let response = tiny_http::Response::empty(422);
                                            let _ = request.respond(response);
                                            continue;
                                        }
                                    }
                                }
                                let response = tiny_http::Response::empty(200);
                                let _ = request.respond(response);
                            }
                            Err(err) => {
                                log::error!("Error receiving request: {:?}", err);
                            }
                        }
                    }
                });
                guards.push(guard);
            }
            for h in guards {
                match h.join() {
                    Ok(_) => {}
                    Err(err) => log::error!("Zabbix catcher error in joining the thread: {:?}", err),
                }
            }
        }
        Err(err) => {
            log::error!("Error creating catcher server: {:?}", err);
        }
    }
}
