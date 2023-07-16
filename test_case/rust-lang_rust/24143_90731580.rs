 rust
// Before.
match Some("hi".to_string()) {
    ref op_string_ref @ Some(ref s) => ...
    None => ...
}

// After.
match Some("hi".to_string()) {
    Some(ref s) => {
        let opt_string_ref = &Some(&s);
        ...
    }
    None => ...
}
