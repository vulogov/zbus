extern crate log;

use crate::stdlib::hostname;

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
pub mod zbus_export;
pub mod zbus_export_zabbix;
pub mod zbus_export_sla_zabbix;
pub mod zbus_export_events_zabbix;
pub mod zbus_export_prometheus;
pub mod zbus_export_stream;
pub mod zbus_version;
pub mod zbus_query;
pub mod zbus_pipeline;
pub mod zbus_pipeline_generator;
pub mod zbus_pipeline_feeder;
pub mod zbus_pipeline_processor;
pub mod zbus_pipeline_sink;
pub mod zbus_pipeline_aggregator;
pub mod zbus_pipeline_fan;
pub mod zbus_pipeline_vector;
pub mod zbus_pipeline_lib;
pub mod zbus_script;
pub mod zbus_query_raw;
pub mod zbus_query_metadata;
pub mod platform_api;
pub mod zabbix_api;
pub mod zabbix_lib;
pub mod prometheus_lib;
pub mod zenoh_lib;


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
        Commands::Subscribe(zsub) => {
            log::debug!("Subscribe to the metrics");
            zbus_subscribe::run(&cli, &zsub, config.clone());
        }
        Commands::Export(zexp) => {
            log::debug!("Export to ZBUS");
            zbus_export::run(&cli, &zexp, config.clone());
        }
        Commands::Api(api) => {
            log::debug!("Platform API");
            platform_api::run(&cli, &api, config.clone());
        }
        Commands::Query(query) => {
            log::debug!("Query ZBUS");
            zbus_query::run(&cli, &query, config.clone());
        }
        Commands::Script(script) => {
            log::debug!("Run script");
            zbus_script::run(&cli, &script, config.clone());
        }
        Commands::Pipeline(pipeline) => {
            log::debug!("Run programmatic telemetry pipeline");
            zbus_pipeline::run(&cli, &pipeline, config.clone());
        }
        Commands::Version(_) => {
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

    #[clap(help="ID of the observability platform", long, default_value_t = String::from("local"))]
    pub platform_name: String,

    #[clap(help="ZENOH bus address", long, default_value_t = String::from(env::var("ZBUS_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus: String,

    #[clap(help="ZENOH listen address", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub listen: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable multicast discovery of ZENOH bus")]
    pub disable_multicast_scout: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Configure CONNECT mode for ZENOH bus")]
    pub set_connect_mode: bool,

    #[clap(short, long, default_value_t = 32, help="Number of threads allocated to a thread manager")]
    pub n: usize,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Put(Put),
    Get(Get),
    Subscribe(Subscribe),
    Export(Export),
    Api(Api),
    Query(Query),
    Script(Script),
    Pipeline(Pipeline),
    Version(Version),
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum TelemetryType {
    Metric,
    Event,
    Trace,
    Log,
    Pipeline,
    Raw,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum TelemetrySources {
    Zabbix,
}

#[derive(Subcommand, Clone, Debug)]
enum ApiCommands {
    Login(Login),
    Metadata(Metadata),
}

#[derive(Subcommand, Clone, Debug)]
enum QueryCommands {
    QueryRaw(QueryRaw),
    QueryMetadata(QueryMetadata),
}

#[derive(Subcommand, Clone, Debug)]
enum ExportCommands {
    History(History),
    Stream(Stream),
    Sla(Sla),
    Events(Events),
    Prometheus(Prometheus),
}

#[derive(Subcommand, Clone, Debug)]
enum PipelineCommands {
    Feeder(PipelineFeeder),
    Generator(PipelineGenerator),
    Processor(PipelineProcessor),
    Sink(PipelineSink),
    Fan(PipelineFan),
    Aggregator(PipelineAggregator),
    Vector(PipelineVector),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Put single telemetry value to the bus")]
pub struct Put {
    #[clap(help="Timestamp", long, default_value_t = String::from("now"))]
    pub timestamp: String,

    #[clap(help="Telemetry source", long, default_value_t = String::from(hostname::get_hostname()))]
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
    #[clap(help="Telemetry source", long, default_value_t = String::from(hostname::get_hostname()))]
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
pub struct Subscribe {
    #[clap(long, value_enum, default_value_t = TelemetryType::Metric, help="Telemetry type")]
    pub telemetry_type: TelemetryType,

    #[clap(help="Telemetry source", long, default_value_t = String::from(hostname::get_hostname()))]
    pub source: String,

    #[clap(help="Telemetry key", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub key: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run programmatic telemetry pipeline")]
pub struct Pipeline {
    #[clap(flatten)]
    group: PipelineArgGroup,

    #[clap(subcommand)]
    command: PipelineCommands,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct PipelineArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Take pipeline script from STDIN")]
    pub stdin: bool,

    #[clap(short, long, help="URL pointing to the file with pipeline code")]
    url: Option<String>,

    #[clap(short, long, help="Filename of the file with pipeline code")]
    file: Option<String>,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Run internal processing")]
    pub internal: bool,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct PipelineVectorGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Send data to VECTOR stdin source")]
    pub stdout: bool,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Feed data to Zbus pipeline")]
pub struct PipelineFeeder {
    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Generate data to pipeline")]
pub struct PipelineGenerator {
    #[clap(help="Pipeline name", long)]
    pub pipeline: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Process data on pipeline")]
pub struct PipelineProcessor {
    #[clap(help="Output pipeline name", long)]
    pub pipeline: String,
    #[clap(help="Input pipeline name", long)]
    pub pipeline_in: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Pipeline sink")]
pub struct PipelineSink {
    #[clap(help="Pipeline name", long)]
    pub pipeline: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Pipeline fan")]
pub struct PipelineFan {
    #[clap(help="Output pipelines names", long)]
    pub pipeline: Vec<String>,
    #[clap(help="Input pipeline name", long)]
    pub pipeline_in: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Pipeline aggregator")]
pub struct PipelineAggregator {
    #[clap(help="Input pipelines names", long)]
    pub pipeline_in: Vec<String>,
    #[clap(help="Output pipeline name", long)]
    pub pipeline: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Pipeline to a VECTOR ")]
pub struct PipelineVector {
    #[clap(help="Input pipelines name", long)]
    pub pipeline_in: String,

    #[clap(flatten)]
    group: PipelineVectorGroup,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Export data to ZBUS")]
pub struct Export {
    #[clap(subcommand)]
    command: ExportCommands,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Platform API calls")]
pub struct Api {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Process calls in loop")]
    pub in_loop: bool,

    #[clap(long, default_value_t = 1, help="Interval between runs")]
    pub every: u16,

    #[clap(long, value_enum, default_value_t = TelemetrySources::Zabbix, help="Telemetry source")]
    pub source: TelemetrySources,

    #[clap(help="API endpoint", long, default_value_t = String::from("http://127.0.0.1:8080"))]
    pub endpoint: String,

    #[clap(subcommand)]
    command: ApiCommands,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the tool")]
struct Version {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Login to Api")]
struct Login {
    #[clap(help="Username", long, default_value_t = String::from("Admin"))]
    pub login: String,

    #[clap(help="Password", long, default_value_t = String::from("password"))]
    pub password: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="List of the hosts in configuration")]
struct Metadata {
    #[clap(help="Authentication token", long, default_value_t = String::from(""))]
    pub token: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Sync metadata to ZBUS")]
    pub sync_zbus: bool,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Export SLA data to ZBUS")]
pub struct Sla {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Process export files in loop")]
    pub in_loop: bool,

    #[clap(long, default_value_t = 1, help="Interval between runs")]
    pub every: u16,

    #[clap(long, value_enum, default_value_t = TelemetrySources::Zabbix, help="Telemetry source")]
    pub source: TelemetrySources,

    #[clap(help="Authentication token", long, default_value_t = String::from(""))]
    pub token: String,

    #[clap(help="API endpoint", long, default_value_t = String::from("http://127.0.0.1:8080"))]
    pub endpoint: String,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Export History data to ZBUS")]
pub struct History {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Process export files in loop")]
    pub in_loop: bool,

    #[clap(long, default_value_t = 1, help="Interval between runs")]
    pub every: u16,

    #[clap(long, value_enum, default_value_t = TelemetrySources::Zabbix, help="Telemetry source")]
    pub source: TelemetrySources,

    #[clap(help="Export files path", long, default_value_t = String::from(std::env::current_dir().unwrap().to_str().unwrap()))]
    pub path: String,

    #[clap(help="Export files search pattern", long, default_value_t = String::from(""))]
    pub search: String,

    #[clap(help="Export files extension", long, default_value_t = String::from("*"))]
    pub extension: String,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Export Events data to ZBUS")]
pub struct Events {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Process export files in loop")]
    pub in_loop: bool,

    #[clap(long, default_value_t = 1, help="Interval between runs")]
    pub every: u16,

    #[clap(long, value_enum, default_value_t = TelemetrySources::Zabbix, help="Telemetry source")]
    pub source: TelemetrySources,

    #[clap(help="Export files path", long, default_value_t = String::from(std::env::current_dir().unwrap().to_str().unwrap()))]
    pub path: String,

    #[clap(help="Export files search pattern", long, default_value_t = String::from(""))]
    pub search: String,

    #[clap(help="Export files extension", long, default_value_t = String::from("*"))]
    pub extension: String,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Export Events data to ZBUS")]
pub struct Prometheus {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Process export files in loop")]
    pub in_loop: bool,

    #[clap(long, default_value_t = 1, help="Interval between runs")]
    pub every: u16,

    #[clap(help="Prometheus Exporter URL", long, default_value_t = String::from("http://127.0.0.1:9100/metrics"))]
    pub exporter: String,

    #[clap(help="Telemetry source", long, default_value_t = String::from("prometheus"))]
    pub source: String,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Export telemetry data from Zabbix external streaming")]
pub struct Stream {

    #[clap(help="Listen address for the stream catcher", long, default_value_t = String::from("0.0.0.0:10055"))]
    pub listen: String,

    #[clap(help="Telemetry source", long, default_value_t = String::from("Zabbix"))]
    pub source: String,

    #[clap(long, default_value_t = 1, help="Number of catcher threads")]
    pub threads: u16,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Print data to STDIN")]
    pub stdout: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Send telemetry to the bus")]
    pub bus: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Zabbix history catcher")]
    pub history: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Zabbix events catcher")]
    pub events: bool,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Query telemetry data on ZBUS")]
pub struct Query {
    #[clap(subcommand)]
    command: QueryCommands,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Query data stored on ZBUS")]
pub struct QueryRaw {
    #[clap(help="ZBUS key", long, default_value_t = String::from("zbus/*"))]
    pub key: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Receive all matched elements")]
    pub all: bool,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Query metadata stored on ZBUS")]
pub struct QueryMetadata {
    #[clap(help="Host ID", long, default_value_t = String::from("zbus/*"))]
    pub hostid: String,

    #[clap(help="Item ID", long, default_value_t = String::from("zbus/*"))]
    pub itemid: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Convert key to ZENOH key format")]
    pub convert: bool,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Run ZBUS script. All arguments passed after -- will be passed to script")]
pub struct Script {
    #[clap(help="API endpoint", long, default_value_t = String::from("http://127.0.0.1:8080"))]
    pub endpoint: String,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Take script from STDIN")]
    pub stdin: bool,

    #[clap(help="File with Script", long, default_value_t = String::from(""))]
    pub file: String,

    #[clap(help="URI with Script", long, default_value_t = String::from(""))]
    pub uri: String,

    #[clap(help="Eval script snippet passed through command line", long, default_value_t = String::from(""))]
    pub eval: String,

    #[clap(last = true)]
    pub args: Vec<String>,
}
