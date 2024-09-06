//! Write logs to standard I/O. No mystery configurations and fancy features.
//!
//! ```no_run
#![doc = include_str!("../examples/write_log.rs")]
//!```

use chrono::Utc;

static DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn write(level: &str, message: &str) {
    let now = Utc::now().naive_utc().format(DATE_TIME_FORMAT);
    println!("{} {: <6} {}", now, level, message)
}

#[doc(hidden)]
pub fn write_info(message: &str) {
    write("INFO", message);
}

#[doc(hidden)]
pub fn write_debug(message: &str) {
    write("DEBUG", message);
}

#[doc(hidden)]
pub fn write_warn(message: &str) {
    write("WARN", message);
}

#[doc(hidden)]
pub fn write_error(message: &str) {
    write("ERROR", message);
}

#[macro_export]
macro_rules! info {
    ($format: expr) => {
        vap::write_info(format!($format).as_str())
    };

    ($format: expr, $($arg: tt)*) => {
        vap::write_info(format!($format, $($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! debug {
    ($format: expr) => {
        vap::write_debug(format!($format).as_str())
    };

    ($format: expr, $($arg: tt)*) => {
        vap::write_debug(format!($format, $($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! warn {
    ($format: expr) => {
       vap::write_warn(format!($format).as_str())
    };

    ($format: expr, $($arg: tt)*) => {
       vap::write_warn(format!($format, $($arg)*).as_str())
    };
}

#[macro_export]
macro_rules! error {
    ($format: expr) => {
       vap::write_error(format!($format).as_str())
    };

    ($format: expr, $($arg: tt)*) => {
       vap::write_error(format!($format, $($arg)*).as_str())
    };
}
