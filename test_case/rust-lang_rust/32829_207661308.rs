 rust
static S : u64 = { { panic!("foo"); 0 } };

fn main() {
    println!("{:?}", S);
}
