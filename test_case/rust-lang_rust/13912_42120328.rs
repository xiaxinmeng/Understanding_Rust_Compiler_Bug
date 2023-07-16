
struct LogLocation {
    file: &'static str,
    line: uint,
    module_path: &'static str,
}

fn log(lvl: LogLevel, loc: &LogLocation, args: &fmt::Arguments) { ... }
