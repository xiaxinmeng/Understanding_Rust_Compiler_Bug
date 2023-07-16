
extern mod std;

use std::io::IoResult;

fn something() -> IoResult<bool> {
    Ok(true)
}

fn main() {
    let a = unwrap!(something());
    println!("{}", a);
}

