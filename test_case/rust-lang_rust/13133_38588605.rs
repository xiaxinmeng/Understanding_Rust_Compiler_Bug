 rust
extern crate green;
extern crate rustuv;

use std::io::File;

#[start]
fn start(argc: int, argv: **u8) -> int {
    green::start(argc, argv, main)
}

fn main() {
    let mut f = File::create(&Path::new("test"));

    println!("allocating");
    let mut vec = Vec::with_capacity(1 << 32);
    unsafe { vec.set_len(1 << 32); }
    println!("writing");
    println!("{}", f.write(vec.as_slice()));
    println!("closing");
    drop(f);
    println!("stat");

    let a = Path::new("test").stat().unwrap();
    assert_eq!(a.size, 1 << 32);
}
