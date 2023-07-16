 rust
match pattern.split( '*' ).collect::<~[&str]>().as_slice() {
    [head, tail] => println!( "{:?} {:?}", head, tail ),
    _ => ()
};
