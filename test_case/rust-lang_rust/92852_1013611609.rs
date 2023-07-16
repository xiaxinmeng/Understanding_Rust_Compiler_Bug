rs
fn foo<T>(_x: T) {
}

fn main() {
    let mut x = String::from("xxx");
    let x_mut_ref = &mut x;
    // foo(x_mut_ref); /* Why this doesn't work? */
    foo::<&mut String>(x_mut_ref); /* and why this does */
    println!("x = {}", x_mut_ref);
}
