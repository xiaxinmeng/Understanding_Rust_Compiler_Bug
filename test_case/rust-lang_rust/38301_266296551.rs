rust
error[E0282]: unable to infer enough type information about `_`
   --> /checkout/src/librustc/hir/def_id.rs:124:21
    |
124 |                     write!(f, " => {}", def_path.to_string(tcx))?;
    |                     ---------------------------------------------
    |                     |
    |                     cannot infer type for `_`
    |                     in this macro invocation
    |
    = note: type annotations or generic parameter binding required

error: aborting due to previous error

error: Could not compile `rustc`.
