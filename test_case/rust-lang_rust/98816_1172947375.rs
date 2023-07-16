plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0063]: missing field `implicit_sized` in initializer of `GenericPredicates<'_>`
    --> src/librustdoc/clean/mod.rs:1176:38
     |
1176 |                     let predicates = ty::GenericPredicates { parent: None, predicates: bounds };
     |                                      ^^^^^^^^^^^^^^^^^^^^^ missing `implicit_sized`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:51
