extern crate log;
extern crate hostname;

use clap::{Args, Parser, Subcommand};
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
    match &cli.command {
        Commands::Put(_zput) => {
            log::debug!("Set single metric to the bus");
            zbus_put::run(&cli);
        }
        Commands::Get(_zget) => {
            log::debug!("Get single metric from the bus");
            zbus_get::run(&cli);
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

    #[clap(help="ZENOH bus address", long, default_value_t = String::from(env::var("ZBUS_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus: String,

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

#[derive(Args, Clone, Debug)]
#[clap(about="Put single telemetry value to the bus")]
struct Put {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get single telemetry value from the bus")]
struct Get {
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
