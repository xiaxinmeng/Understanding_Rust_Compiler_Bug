rust
error: lifetime may not live long enough
  --> src/main.rs:16:21
   |
16 |     go(|line: &str| line.chars()); // error
   |               -   - ^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |               |   |
   |               |   return type of closure is Chars<'2>
   |               let's call the lifetime of this reference `'1`
