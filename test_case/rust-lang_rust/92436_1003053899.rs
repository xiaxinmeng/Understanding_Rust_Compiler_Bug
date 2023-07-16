rust
pub fn foobar(x: u8) -> [u8; 1024] {
    [x; 1024]
}

pub fn main() {
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();

    let fb = foobar(number as u8);
    println!("{}", fb[0]);
}
