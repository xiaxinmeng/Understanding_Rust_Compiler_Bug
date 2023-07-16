rust
fn add_something(vals: &mut Vec<i32>) {
    vals.push(1); // WARN unused variable: `vals`
}
  
fn main() {
    let mut vals = Vec::new();
    add_something(vals);
    println!("{:?}", vals);
}
