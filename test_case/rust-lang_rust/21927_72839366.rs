 rust
fn main() {
    let vs = vec![1, 2, 3, 4];
    for v in vs {

        println!("a");
        println!("b");
        println!("c");

        match v {
            &1 => println!("one"),
            _ => println!("more"),
        }
    }
}
