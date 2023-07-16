rust
fn check<'a>() {
    'a: loop {} // warning: label name `'a` shadows a lifetime name that is already in scope
}
