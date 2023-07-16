rust
fn main() {
    let mut tuples = vec![(1, 2)];
    for &(s, _) in &tuples {
        f(s);
    }
    let first = tuples.pop();
}

fn f(u: usize) {
    println!("{}", u);
}
