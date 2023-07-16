 rust
fn log(record: &LogRecord) {
    write!(stdout, "{} {}: {}", record.file, record.line, record.args);
}
