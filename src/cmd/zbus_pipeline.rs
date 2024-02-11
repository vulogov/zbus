extern crate log;
use zenoh::config::{Config};

use crate::cmd;
use crate::cmd::{zbus_pipeline_generator, zbus_pipeline_processor, zbus_pipeline_sink, zbus_pipeline_aggregator, zbus_pipeline_fan};

pub fn run(c: &cmd::Cli, pipeline: &cmd::Pipeline, zc: Config)  {
    log::trace!("zbus_pipeline::run() reached");
    match &pipeline.command {
        cmd::PipelineCommands::Generator(generator) => {
            zbus_pipeline_generator::run(c, pipeline, generator, zc)
        }
        cmd::PipelineCommands::Processor(processor) => {
            zbus_pipeline_processor::run(c, pipeline, processor, zc)
        }
        cmd::PipelineCommands::Sink(sink) => {
            zbus_pipeline_sink::run(c, pipeline, sink, zc)
        }
        cmd::PipelineCommands::Aggregator(aggregator) => {
            zbus_pipeline_aggregator::run(c, pipeline, aggregator, zc)
        }
        cmd::PipelineCommands::Fan(fan) => {
            zbus_pipeline_fan::run(c, pipeline, fan, zc)
        }
    }
}
