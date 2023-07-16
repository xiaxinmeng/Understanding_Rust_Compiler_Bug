rust
#![feature(try_blocks)]

fn main() {
    match try { Ok::<_, ()>(0) } {
        Ok(_) => {},
        Err(()) => {}
    }
}
