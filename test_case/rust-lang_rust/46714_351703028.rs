rust
let value = None;
loop {
    if value.is_some() {
        // the type of `value.unwrap()` is not yet known
        value.unwrap().bar(); 
    } else {
        // this would tell us that the type of `value.unwrap()` is `char`,
        // but we never get this far
        value = Some('a');
    }
}
