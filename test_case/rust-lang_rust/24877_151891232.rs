 rust
use std::io;
struct Foo;

impl std::io::Write for Foo {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn example() -> Box<Foo> {
  Box::new(Foo)
}

fn main() {
  let mut out = example();
  writeln!(&mut out, "Hello World!");
}
