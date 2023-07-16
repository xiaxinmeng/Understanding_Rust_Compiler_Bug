rust
let (x, y) = if let Some((x, y)) = foo() {
    (Some(x), Some(y))
} else {
    (None, None)
};

// Use x and y separately
