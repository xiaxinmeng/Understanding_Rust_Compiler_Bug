 rust
let foo = 3;
unsafe {
    let foo: &mut int = &mut *(&foo as *int as *mut int);
    *foo = 4;
}
