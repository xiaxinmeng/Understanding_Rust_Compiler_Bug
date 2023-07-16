
fn main() {
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => println!("{:?}", line.as_bytes()),
            Err(e) => panic!("{}", e),
        }
    }
}
