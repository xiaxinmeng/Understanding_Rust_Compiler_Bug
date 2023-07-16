 bash
Compiling rustc_metadata v0.0.0 (file:///build/src/librustc_metadata)
error[E0282]: unable to infer enough type information about `_`
   --> src/librustc_metadata/decoder.rs:358:25
    |
358 |             predicates: (0..self.read_usize()?).map(|_| {
    |                         ^ cannot infer type for `_`
    |
    = note: type annotations or generic parameter binding required

error: aborting due to previous error
