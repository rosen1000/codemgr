use chrono::offset;
use colored::Colorize;
use std::fmt::Debug;

#[cfg(debug_assertions)]
pub fn print<T>(data: T, log_type: LoggingLevel) -> ()
where
    T: Debug,
{
    match log_type {
        LoggingLevel::INFO => {
            println!(
                "{} {}: {:#?}",
                format!("{}", get_time()).truecolor(128, 128, 128),
                format!("INFO").blue(),
                &data
            )
        }
        LoggingLevel::WARN => {
            println!(
                "{} {}: {:?}",
                format!("{}", get_time()).truecolor(128, 128, 128),
                format!("WARN").yellow(),
                &data
            )
        }
        LoggingLevel::ERROR => {
            println!(
                "{} {}: {:?}",
                format!("{}", get_time()).truecolor(128, 128, 128),
                format!("ERROR").red(),
                &data
            )
        }
    }
}

fn get_time() -> String {
    let now = offset::Local::now().format("%_H:%M:%S.%3f").to_string();
    return now;
}

pub enum LoggingLevel {
    INFO,
    WARN,
    ERROR,
}
