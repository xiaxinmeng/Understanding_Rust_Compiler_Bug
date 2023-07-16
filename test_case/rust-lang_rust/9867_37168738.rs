 rust
extern {
    fn read();
}

static a: u64 = read as u64;

fn main() {
    println!("{:u}", a);
}
