
fn main() {
    let mut a = 1;
    let mut b = 1;

    while b < 10 {
        a = b;
        b = a + b;
    }

    println!("Result: {}", b);
}
