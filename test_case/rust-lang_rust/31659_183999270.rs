 rust
fn Some(x: i32) -> Option<i32> { Some(1) }

fn main() {
    let x = 0;
    match Some(x) {
        Some(x) => println!("{}", x), // This would print "1"
        _ => {}
    }
}
