use log::SetLoggerError;
use simplelog::*;

use std::fs::OpenOptions;

const LOG_FILE: &str = "ddi3cli.log";

pub fn init() -> Result<(), SetLoggerError> {
    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
    // 打开日志文件以追加模式
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(LOG_FILE)
        .expect("can not open log file");

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, config.clone(), log_file),
    ])
}
