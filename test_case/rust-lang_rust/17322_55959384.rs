
use std::io;

fn main() {
    let mut wr: Box<Writer + 'static> = box io::stdout();
    let x: &mut Box<Writer> = &mut wr;
    let f: &mut Writer = x;
}
