rust
fn takes_cb(f: fn(i8)) {}

fn main() {
    fn callback(x: i32) {}
    takes_cb(callback)
}
