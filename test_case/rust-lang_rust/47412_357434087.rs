rust
#![feature(nll)]

fn main() {
    let pair = (Box::new(22), Box::new(22));
    drop(pair);
    match pair { 
        _ => { }
    }
}):
