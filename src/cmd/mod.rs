extern crate log;
extern crate hostname;

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use zenoh::config::{Config, ConnectConfig, ListenConfig, EndPoint, WhatAmI};
use std::env;
use std::fmt::Debug;
use crate::stdlib;

pub mod setloglevel;

pub mod zbus_get;
pub mod zbus_put;
pub mod zbus_subscribe;
pub mod zbus_version;


pub fn init() {
    log::debug!("Parsing CLI parameters");
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    stdlib::initlib(&cli);
    let mut config =  Config::default();

    if cli.disable_multicast_scout.clone() {
        match config.scouting.multicast.set_enabled(Some(false)) {
            Ok(_) => { log::debug!("Multicast discovery disabled")}
            Err(err) => {
                log::error!("Failure in disabling multicast discovery: {:?}", err);
                return;
            }
        }
    }
    match EndPoint::from_str(&cli.bus) {
        Ok(zconn) => {
            log::debug!("ZENOH bus set to: {:?}", &zconn);
            let _ = config.set_connect(ConnectConfig::new(vec![zconn]).unwrap());
        }
        Err(err) => {
            log::error!("Failure in parsing connect address: {:?}", err);
            return;
        }
    }
    match EndPoint::from_str(&cli.listen) {
        Ok(zlisten) => {
            log::debug!("ZENOH listen set to: {:?}", &zlisten);
            let _ = config.set_listen(ListenConfig::new(vec![zlisten]).unwrap());
        }
        Err(_) => {
            log::debug!("ZENOH listen set to default");
        }
    }
    if cli.set_connect_mode {
        log::debug!("ZENOH configured in CONNECT mode");
        let _ = config.set_mode(Some(WhatAmI::Client));
    } else {
        log::debug!("ZENOH configured in PEER mode");
        let _ = config.set_mode(Some(WhatAmI::Peer));
    }
    if config.validate() {
        log::debug!("ZENOH config is OK");
    } else {
        log::error!("ZENOH config not OK");
        return;
    }
    match &cli.command {
        Commands::Put(zput) => {
            log::debug!("Set single metric to the bus");
            zbus_put::run(&cli, &zput, config.clone());
        }
        Commands::Get(zget) => {
            log::debug!("Get single metric from the bus");
            zbus_get::run(&cli, &zget, config.clone());
        }
        Commands::Subscribe(_zsub) => {
            log::debug!("Subscribe to the metrics");
            zbus_subscribe::run(&cli);
        }
        Commands::Version(_version) => {
            log::debug!("Get the tool version");
            zbus_version::run(&cli);
        }
    }
}

#[derive(Parser, Clone)]
#[clap(name = "zbus")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "ZBUS telemetry CLI tool", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    pub debug: u8,

    #[clap(help="ZBUS telemetry protocol version", long, default_value_t = String::from("v1"))]
    pub protocol_version: String,

    #[clap(help="ZENOH bus address", long, default_value_t = String::from(env::var("ZBUS_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus: String,

    #[clap(help="ZENOH listen address", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub listen: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable multicast discovery of ZENOH bus")]
    pub disable_multicast_scout: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Configure CONNECT mode for ZENOH bus")]
    pub set_connect_mode: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Put(Put),
    Get(Get),
    Subscribe(Subscribe),
    Version(Version),
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum TelemetryType {
    Metric,
    Event,
    Trace,
    Log,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Put single telemetry value to the bus")]
pub struct Put {
    #[clap(help="Timestamp", long, default_value_t = String::from("now"))]
    pub timestamp: String,

    #[clap(help="Telemetry source", long, default_value_t = String::from(hostname::get().unwrap().into_string().unwrap()))]
    pub source: String,

    #[clap(long, value_enum, default_value_t = TelemetryType::Metric, help="Telemetry type")]
    pub telemetry_type: TelemetryType,

    #[clap(help="Telemetry key", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub key: String,

    #[clap(help="Telemetry value", long, default_value_t = String::from(""))]
    pub value: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Pass the value as-is without computation")]
    pub raw_value: bool,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get single telemetry value from the bus")]
pub struct Get {
    #[clap(help="Telemetry source", long, default_value_t = String::from(hostname::get().unwrap().into_string().unwrap()))]
    pub source: String,

    #[clap(long, value_enum, default_value_t = TelemetryType::Metric, help="Telemetry type")]
    pub telemetry_type: TelemetryType,

    #[clap(help="Telemetry key", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub key: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Subscribe to the telemetry on the bus")]
struct Subscribe {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the tool")]
struct Version {
    #[clap(last = true)]
    args: Vec<String>,
}
