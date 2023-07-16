
extern "C" {
    static STATIC: Vec<u8>;
}

fn main() {
    println!("{}", STATIC[0]);
}
