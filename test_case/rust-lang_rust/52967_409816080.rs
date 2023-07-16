rust
#![feature(nll)]

fn main() {
    let mut stuff = ("left", "right");
    match stuff {
        (ref mut left, _) if *left == "left" => { }
        _ => {}
    }
}
