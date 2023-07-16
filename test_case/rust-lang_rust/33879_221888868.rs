 rust
fn main() {
    let f = foo;

    // Compiles, even though the single-argument version doesn't with
    // explicit parameter types
    let _ = foo as fn(u32, u32) == f;
}

fn foo(param: u32, param2: u32) {
    println!("{} {}", param, param2);
}
