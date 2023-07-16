rust
// orig
let Some(x) = blah() else {
    return;
};
loop {}

// desugar
let val = if let Some(x) = blah() {
    loop {}
} else {
    return;
};
val // unreachable
