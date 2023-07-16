
fn main() {
    let mut b = 1;

    while b < 10 {
        let a = b;
        b = a + b;
    }

    println!("Result: {}", b);
}
