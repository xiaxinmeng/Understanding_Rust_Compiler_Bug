 rust
let foo = returns_option();
do_things(&foo);
match foo {
    Some(ref a) => { /* use a */ }
    None => {}
}
