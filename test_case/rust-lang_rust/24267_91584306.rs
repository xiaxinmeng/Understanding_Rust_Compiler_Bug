 rust
fn consume(bar: Box<i32>) {
    println!("{:?}", x);
}

fn main() {
    let mut foo = Box::new(1);
    loop {
        foo = { consume(foo); break };
    }
    consume(foo);
}
