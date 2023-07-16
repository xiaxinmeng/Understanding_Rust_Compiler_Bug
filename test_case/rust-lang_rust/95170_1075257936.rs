plain
   Compiling diff v0.1.12
   Compiling ansi_term v0.12.1
   Compiling pretty_assertions v0.7.2
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0451]: field `llvm_link_shared` of struct `config::Config` is private
  --> src/bootstrap/builder/tests.rs:67:43
   |
67 |         let config = Config { stage: 0, ..configure("build", &["A"], &["A"]) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `llvm_link_shared` is private

error[E0451]: field `llvm_link_shared` of struct `config::Config` is private
  --> src/bootstrap/builder/tests.rs:89:43
   |
89 |         let config = Config { stage: 1, ..configure("build", &["A", "B"], &["A", "B"]) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `llvm_link_shared` is private

error[E0451]: field `llvm_link_shared` of struct `config::Config` is private
   --> src/bootstrap/builder/tests.rs:171:30
    |
171 |         Config { stage: 2, ..super::configure("dist", host, target) }
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `llvm_link_shared` is private
For more information about this error, try `rustc --explain E0451`.
error: could not compile `bootstrap` due to 3 previous errors
Build completed unsuccessfully in 0:28:11
