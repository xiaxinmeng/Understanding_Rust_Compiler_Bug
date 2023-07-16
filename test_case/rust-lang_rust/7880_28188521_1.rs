 rust
let foo = returns_option();
match foo {
    Some(ref a) => { /* use a */ }
    None => {}
}
do_things(&foo);
