rust
macro n($i:ident) {
    m!($i, x); // The resolution of `x` depends on whether `$i` is `foo` or `bar`
}

// Here, we are "matching against names in patterns" with `foo` and `bar`:
macro m {
    (foo, $j:ident) => { let $j = 0; $j },
    (bar, $j:ident) => { $j },
}
