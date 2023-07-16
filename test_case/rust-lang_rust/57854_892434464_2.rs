
error[E0637]: `&` without an explicit lifetime name cannot be used here
 --> src/lib.rs:3:24
  |
3 | fn bar<R>(r: &R) where &R: std::io::Read {
  |                        ^ explicit lifetime name needed here

error[E0310]: the parameter type `R` may not live long enough
 --> src/lib.rs:3:28
  |
3 | fn bar<R>(r: &R) where &R: std::io::Read {
  |        -                   ^^^^^^^^^^^^^ ...so that the reference type `&'static R` does not outlive the data it points at
  |        |
  |        help: consider adding an explicit lifetime bound...: `R: 'static`
