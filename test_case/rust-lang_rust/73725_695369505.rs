
error[E0601]: `main` function not found in crate `lib`
    --> lib.rs:1:1
     |
1    | / #[allow(clippy::unreadable_literal, clippy::collapsible_if, clippy::...
2    | | #[rustfmt::skip]
3    | | pub fn predict(x : &[f64]) -> f64 {
4    | |         
...    |
2008 | |  alpha
2009 | | }
     | |_^ consider adding a `main` function to `lib.rs`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

