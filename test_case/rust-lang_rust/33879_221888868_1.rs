 rust
fn main() {
    let f = foo;
    // error: binary operation `==` cannot be applied to type `fn(&u32, u32)` [E0369]
    let _ = foo as fn(&u32, u32) == f;

    // Compiles
    let _ = foo as fn(_, u32) == f;

}

fn foo(param: &u32, param2: u32) {
    println!("{} {}", *param, param2);
}
