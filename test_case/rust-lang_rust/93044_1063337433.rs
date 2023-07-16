rust
use std::io::{Result, Write};
use std::sync::Arc;

#[derive(Debug)]
enum Parity {
    Even,
    Odd,
}

impl Write for &Parity {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.iter().map(|x| x.count_ones()).sum::<u32>() % 2 == 1 {
            match self {
                Parity::Even => *self = &Parity::Odd,
                Parity::Odd => *self = &Parity::Even,
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let mut parity = Arc::new(Parity::Even);
    write!(parity, "###").unwrap();
    println!("{:?}", parity);  // Even
    write!(parity, "###").unwrap();
    println!("{:?}", parity);  // Even
}
