
error: lifetime may not live long enough
   --> src/payload.rs:173:42
    |
    |         let closure = |s: &str| -> &str {&s[..]};
    |                           -        -     ^^^^^^ returning this value requires that `'1` must outlive `'2`
    |                           |        |
    |                           |        let's call the lifetime of this reference `'2`
    |                           let's call the lifetime of this reference `'1`
