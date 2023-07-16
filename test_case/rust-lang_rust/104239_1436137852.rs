plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0423]: expected function, found macro `format`
   --> compiler/rustc_borrowck/src/region_infer/mod.rs:277:30
    |
277 |         components_str.push(&format(
    |
help: use `!` to invoke the macro
    |
    |
277 |         components_str.push(&format!(
help: consider importing one of these items instead
    |
1   | use rustc_span::sym::format;
    |
    |
1   | use std::fmt::format;
    |

error[E0599]: no method named `push` found for reference `&str` in the current scope
    |
    |
277 |         components_str.push(&format(
    |                        ^^^^ method not found in `&str`
Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
