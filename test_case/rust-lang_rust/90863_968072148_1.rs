
   Compiling playground v0.0.1 (/playground)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   --> src/main.rs:9:18
    |
8   | / pub fn main() {
    | |            ^^ main does not return `Result` or `Option` to accept `?`
9   | |     give_result()?;
    | |                  ^ cannot use the `?` operator in a function that returns `()`
10  | | }
    |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
