 rust
fn changer<'a>(mut things: Box<Iterator<Item=&'a mut u8>>) {
    for item in things { *item = 0 }
}

fn main() {
}
