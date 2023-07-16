rust
fn check<'a>() {
    'a: loop {}
    'a: loop {} // warning: label name `'a` shadows a label name that is already in scope
}
