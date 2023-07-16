plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0317]: `if` may be missing an `else` clause
     |
1086 | /                 if let Err(err) = res {
1087 | |                     match err {
1088 | |                         TestFailure::CompileError => {
1088 | |                         TestFailure::CompileError => {
1089 | |                             eprint!("Couldn't compile the test.");
...    |
1135 | |                     panic::resume_unwind(Box::new(()));
     | |_________________^ expected `()`, found enum `Result`
     |
     = note: expected unit type `()`
                     found enum `Result<(), std::string::String>`
                     found enum `Result<(), std::string::String>`
     = note: `if` expressions without `else` evaluate to `()`
     = help: consider adding an `else` block that evaluates to the expected type
For more information about this error, try `rustc --explain E0317`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:47
