Rust
let x = if cond {
    panic!()
} else {
    panic!();
    42 //~ WARNING unreachable code
};
42 //~ WARNING? unreachable code
