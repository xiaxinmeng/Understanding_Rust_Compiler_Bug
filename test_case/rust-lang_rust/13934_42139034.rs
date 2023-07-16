 rust
let foo = 3;
unsafe {
    let foo: &mut int = transmute(&foo);
    *foo = 4;
}
