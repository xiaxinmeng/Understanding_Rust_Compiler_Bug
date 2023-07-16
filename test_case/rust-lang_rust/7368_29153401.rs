 rust
use std::os;

fn main() {
    let p = os::make_absolute(&Path::new("./../foo.txt"));
    println!("{}", p.display());
}
