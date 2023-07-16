plain
   Compiling difference v2.0.0
   Compiling ansi_term v0.11.0
   Compiling pretty_assertions v0.6.1
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0451]: field `incremental` of struct `config::Config` is private
  --> src/bootstrap/builder/tests.rs:67:43
   |
67 |         let config = Config { stage: 0, ..configure("build", &["A"], &["A"]) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `incremental` is private

error[E0451]: field `incremental` of struct `config::Config` is private
  --> src/bootstrap/builder/tests.rs:89:43
   |
89 |         let config = Config { stage: 1, ..configure("build", &["A", "B"], &["A", "B"]) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `incremental` is private

error[E0451]: field `incremental` of struct `config::Config` is private
   --> src/bootstrap/builder/tests.rs:171:30
    |
171 |         Config { stage: 2, ..super::configure("dist", host, target) }
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `incremental` is private
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0451`.
error: could not compile `bootstrap`
