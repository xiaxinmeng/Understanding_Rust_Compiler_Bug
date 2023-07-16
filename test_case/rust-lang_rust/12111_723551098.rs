rust
fn main() {
    let mut x = 2134;
    // simulate: console.log(x, ++x);
    println!("{}, {}", x, {let y = x; x += 1; y} );
}
