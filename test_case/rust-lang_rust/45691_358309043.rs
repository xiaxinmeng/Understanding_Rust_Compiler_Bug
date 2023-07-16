rust
fn main() {
    method_t(5);
    method_t(5.5);
}

fn method_t<T: Debug>(v: T) {
    println!("{:?}", v);
}
