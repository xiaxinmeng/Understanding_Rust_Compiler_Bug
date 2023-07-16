
[00:23:22]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:23:23] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:23:23]    --> librustc_traits/lowering.rs:226:18
[00:23:23]     |
[00:23:23] 226 |     let clause = Clause::Implies(where_clauses, normalize_goal);
[00:23:23]     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 parameter
[00:23:23] 
[00:23:23] error: aborting due to previous error
[00:23:23] 
[00:23:23] error: Could not compile `rustc_traits`.
