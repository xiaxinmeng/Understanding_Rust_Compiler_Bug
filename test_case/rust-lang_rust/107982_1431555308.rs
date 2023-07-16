rust
let options = if cond {
    Some(format!(...))
} else {
    None
};
// ...
let obj = Struct {
    val: options.as_ref(),
};
