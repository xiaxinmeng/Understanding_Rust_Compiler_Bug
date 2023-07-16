
extern mod std;

use std::io::IoResult;

fn something() -> IoResult<bool> {
    Ok(true)
}

fn main() {
    let a = if_ok!(something());
    println!("{}", a);
}
