
fn factorial(x: isize) -> isize {
    if x <= 0 { 1 } else { x * factorial(x - 1) }
}

fn main() {
    let x = 4;
    println!("{}", factorial(x));
}

