rust
fn main() {
    let mut lock = vec![1; 10];
    while let Some(x) = lock.pop() {
        lock.push(x);
        println!("the lock works as expected");
    }
}
