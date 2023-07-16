 rust
let foo = Rc::new(3);
match foo {
    box n => println!("{}", n),
}
