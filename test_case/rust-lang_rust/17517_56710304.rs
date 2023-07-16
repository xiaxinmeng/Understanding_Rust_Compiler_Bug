
fn main() {
    let mut my_stuff = std::collections::HashMap::new();
    my_stuff.insert(0i, 42i);

    let (_, thing) = my_stuff.iter().next().unwrap();

    my_stuff.clear(); // Error: cannot borrow while `thing` still exists

    println!("{}", *thing);
}
