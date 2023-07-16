plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
    --> library/core/src/str/iter.rs:1490:9
     |
1489 |     fn clone(&self) -> Self {
     |                        ---- expected `SplitRInclusive<'a, P>` because of return type
1490 |         SplitInclusive(self.0.clone())
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `SplitRInclusive`, found struct `str::iter::SplitInclusive`
     |
     = note: expected struct `SplitRInclusive<'a, P>`
                found struct `str::iter::SplitInclusive<'_, P>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:12
