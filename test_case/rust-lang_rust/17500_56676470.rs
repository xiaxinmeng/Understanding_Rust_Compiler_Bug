
fn main() {
    let mut my_stuff = std::collections::HashMap::new();

    // ok
    let foo: Entries<'static, int, int> = my_stuff.iter();

    // error: borrowed value does not live long enough
    // let bar: Items<'static, int> = vec!(1i).iter();
}
