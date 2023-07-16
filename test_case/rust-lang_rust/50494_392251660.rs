rust
let cell = Cell::new(Some(String::from("foo")));
cell.get_with(|s| {
    let s = s.as_ref().unwrap();
    cell.set(None);
    let s2 = String::from("bar");
    println!("{}", s); // crash or print "bar" or garbage
    println!("{}", s2); // keep `s2` alive
});
