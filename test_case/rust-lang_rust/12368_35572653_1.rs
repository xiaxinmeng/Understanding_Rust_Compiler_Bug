 rust
fn main() {
    for line in io::stdin().lines().fail_on_error() {
        print!("received: {}", line);
    }
}
