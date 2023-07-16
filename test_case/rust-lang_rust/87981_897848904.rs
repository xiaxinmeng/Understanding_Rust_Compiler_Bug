plain
    Checking toml v0.5.7
error[E0530]: match bindings cannot shadow struct variants
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:229:10
    |
219 |     use StmtKind::*;
    |         ----------- the struct variant `Empty` is imported here
...
229 |         (Empty, Empty) => true,
    |          ^^^^^ cannot be named the same as a struct variant

error[E0416]: identifier `Empty` is bound more than once in the same pattern
    |
    |
229 |         (Empty, Empty) => true,
    |                 ^^^^^ used in a pattern more than once
Some errors have detailed explanations: E0416, E0530.
For more information about an error, try `rustc --explain E0416`.
error: could not compile `clippy_utils` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
