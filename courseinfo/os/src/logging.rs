use crate::console::Stdout;
use core::fmt::{self, Write};

pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub fn get_color_str(&self) -> &str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }

    pub fn get_color_code(&self) -> u8 {
        match self {
            Level::Error => 31,
            Level::Warn => 93,
            Level::Info => 34,
            Level::Debug => 32,
            Level::Trace => 90,
        }
    }
}

macro_rules! with_color {
    ($args: ident, $color: ident) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $color as u8, $args)
    }};
}

pub fn print_with_color(args: fmt::Arguments, color: u8) {
    Stdout.write_fmt(with_color!(args, color)).unwrap();
}

// struct Logger;

// impl Logger {
//     fn log(&self, args: fmt::Arguments) {
//         print_with_color(
//             with_meta!(args),
//             Level::Error.get_color_code()
//         )
//     }
// }

pub fn log(args: fmt::Arguments, level: Level) {
    print_with_color(
        format_args!("[{:>5}][{}] {}\n", level.get_color_str(), 0, args),
        level.get_color_code(),
    )
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::log(format_args!($fmt $(, $($arg)+)?), $crate::logging::Level::Error);
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::log(format_args!($fmt $(, $($arg)+)?), $crate::logging::Level::Warn);
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::log(format_args!($fmt $(, $($arg)+)?), $crate::logging::Level::Info);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::log(format_args!($fmt $(, $($arg)+)?), $crate::logging::Level::Debug);
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::logging::log(format_args!($fmt $(, $($arg)+)?), $crate::logging::Level::Trace);
    }
}
