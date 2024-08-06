

use chrono::prelude::*;
use std::fmt::Write;

pub const CONSOLE_COLOR_RED: &str = "\x1b[1;31m";
pub const CONSOLE_COLOR_GREEN: &str = "\x1b[32m";
pub const CONSOLE_COLOR_YELLOW: &str = "\x1b[01;33m";
pub const CONSOLE_COLOR_BLUE: &str = "\x1b[34m";
pub const CONSOLE_COLOR_MAGENTA: &str = "\x1b[1;35m";
pub const CONSOLE_COLOR_CYAN: &str = "\x1b[0;36m";

pub const CONSOLE_COLOR_BOLD_WHITE: &str = "\x1b[97m";
pub const CONSOLE_BG_COLOR_RED: &str = "\x1b[41m";
pub const CONSOLE_BG_COLOR_GREEN: &str = "\x1b[42m";
pub const CONSOLE_COLOR_RESET: &str = "\x1b[0m";

pub fn get_system_time() -> String {
    let local: DateTime<Local> = Local::now();
    let mut time_str = String::new();

    write!(
        &mut time_str,
        "{:02}:{:02}:{:02}",
        local.hour(),
        local.minute(),
        local.second()
    )
    .unwrap();
    time_str
}

pub fn info(msg: &str) {
    println!("|{}| {}", get_system_time(), msg);
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        println!("{}|{}| {}{}",
        crate::Log::CONSOLE_COLOR_BLUE,
        crate::Log::get_system_time(),
        format!($($arg)*),
        crate::Log::CONSOLE_COLOR_RESET);
        
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        println!("{}|{}| {}{}",
        crate::Log::CONSOLE_COLOR_YELLOW,
        crate::Log::get_system_time(),
        format!($($arg)*),
        crate::Log::CONSOLE_COLOR_RESET);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        println!("{}|{}| {}{}",
        crate::Log::CONSOLE_COLOR_RED, 
        crate::Log::get_system_time(),
        format!($($arg)*),
        crate::Log::CONSOLE_COLOR_RESET);
    }}
}

#[macro_export]
macro_rules! crit {
    ($($arg:tt)*) => {{
        println!("{}{}|{}| {}{}",
        crate::Log::CONSOLE_BG_COLOR_RED, 
        crate::Log::CONSOLE_COLOR_BOLD_WHITE, 
        crate::Log::get_system_time(),
        format!($($arg)*),
        crate::Log::CONSOLE_COLOR_RESET);
    }}
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("{}{}|{}| {}{}",
        crate::Log::CONSOLE_BG_COLOR_GREEN,
        crate::Log::CONSOLE_COLOR_BOLD_WHITE,
        crate::Log::get_system_time(),
        format!($($arg)*),
        crate::Log::CONSOLE_COLOR_RESET);
    }}
}
