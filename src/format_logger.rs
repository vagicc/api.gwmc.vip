// 格式化日志,应该研究下怎么同时把日志写到文件中
 
struct Padded<T> {
    value: T,
    width: usize,
}
use std::fmt;
impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

pub fn formatlog(
    buf: &mut env_logger::fmt::Formatter,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    // /home/luck/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/pretty_env_logger-0.4.0/src/lib.rs
    // 165行
    use std::io::Write;
    let target = record.target();
    let max_width = max_target_width(target);

    let mut style = buf.style();
    let level = colored_level(&mut style, record.level());

    let mut style = buf.style();
    let target = style.set_bold(true).value(Padded {
        value: target,
        width: max_width,
    });

    // Result<(),Error>
    let temp = writeln!(
        buf,
        " [{} {} {}] {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        level,
        target,
        record.args()
    );
    temp
}

fn colored_level<'a>(
    style: &'a mut env_logger::fmt::Style,
    level: log::Level,
) -> env_logger::fmt::StyledValue<'a, &'static str> {
    use env_logger::fmt::Color;
    use log::Level;
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

fn max_target_width(target: &str) -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

pub fn get_log_level() -> log::LevelFilter {
    let level = crate::common::get_env("RUST_LOG");

    let mut log_level = log::LevelFilter::Debug;
    if level.eq("Debug") {
        log_level = log::LevelFilter::Debug;
    } else if level.eq("Info") {
        log_level = log::LevelFilter::Info;
    } else if level.eq("Warn") {
        log_level = log::LevelFilter::Warn;
    } else if level.eq("Error") {
        log_level = log::LevelFilter::Error;
    } else if level.eq("Off") {
        log_level = log::LevelFilter::Off;
    } else if level.eq("Trace") {
        log_level = log::LevelFilter::Trace;
    }
    log_level
}
