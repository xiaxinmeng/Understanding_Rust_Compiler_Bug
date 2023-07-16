 rust
fn main() {
    let i = 0i;
    // let f = box &i as Box<Clone + 'static>; // This gives a lifetime error
    let f: Box<Clone + 'static> = box &i; // This compiles!
}
