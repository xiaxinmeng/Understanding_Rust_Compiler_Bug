 rust
struct Record {
  name: ~str,
  message: ~str
// etc...
}
struct Logger {
  name: ~str,
  stream: Writer,
  fmt: ~str
}
impl Logger {
  fn log(&mut self, message: ~str) {
    self.stream.write_line(self.format(Record{name:self.name, message: message}));
  }
  fn format(&mut self, record: Record) {
    std::format::dynamic_format(self.fmt, record) // <-- use case
  }
  fn setFormat(&mut self, format: ~str) {
    self.fmt = format;
  }
}

// else where
logger.setFormat(~"{name}.{level}: {message}");
