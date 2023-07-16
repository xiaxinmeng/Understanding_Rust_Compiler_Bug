console
Compiling cargo v0.42.0 (/checkout/src/tools/cargo)

error: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
  --> src/tools/cargo/src/cargo/ops/cargo_doc.rs:97:23
   |
97 | e.description()
   |   ^^^^^^^^^^^
   |
note: lint level defined here
  --> src/tools/cargo/src/cargo/lib.rs:1:24
   |
 1 | #![cfg_attr(test, deny(warnings))]
   |                        ^^^^^^^^
   = note: `#[deny(deprecated)]` implied by `#[deny(warnings)]`
