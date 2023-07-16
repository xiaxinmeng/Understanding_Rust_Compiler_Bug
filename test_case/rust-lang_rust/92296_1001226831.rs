plain
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0505]: cannot move out of `params` because it is borrowed
  --> src/librustdoc/clean/simplify.rs:76:20
   |
53 |     let equalities = equalities.into_iter().filter(|&(ref lhs, ref rhs)| {
   |                                                    --------------------- borrow of `params` occurs here
...
63 |         let (bounds, _) = match params.get_mut(generic) {
   |                                 ------ borrow occurs due to use in closure
...
76 |     clauses.extend(params.into_iter().map(|(k, (bounds, params))| WP::BoundPredicate {
   |                    ^^^^^^ move out of `params` occurs here
...
86 |     clauses.extend(equalities.map(|(lhs, rhs)| WP::EqPredicate { lhs, rhs }));

For more information about this error, try `rustc --explain E0505`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:33
