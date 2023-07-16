 rust
use std::ops::Index;

struct Mem {
    mem: [u16, .. 0x10000]
}

impl Mem {
    fn new() -> Mem {
        Mem { mem: [0, .. 0x10000] }
    }
}

impl Index<u16, u16> for Mem {
    fn index<'a> (&'a self, index: &u16) -> &'a u16 {
        &self.mem[*index as uint]
    }
}

fn main() {
    let mem = Mem::new();
    println!("{}", mem[mem[0]]);
}
