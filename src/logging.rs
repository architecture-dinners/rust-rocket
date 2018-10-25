use std::boxed::Box;
use log::LevelFilter;
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn get_logger() {
    let stderr = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .logger(Logger::builder().build("rust_logging_app", LevelFilter::Info))
        .build(Root::builder().appender("stderr").build(LevelFilter::Info))
        .expect("Built logger config");

    let handle = log4rs::init_config(config).unwrap();
}
