plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0282]: type annotations needed
   --> compiler/rustc_query_system/src/query/plumbing.rs:328:32
    |
328 |                 .lookup(&key, |value, index| (value.clone(), index))
    |                                ^^^^^ consider giving this closure parameter a type
    = note: type must be known at this point

error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> compiler/rustc_query_system/src/query/plumbing.rs:328:18
   --> compiler/rustc_query_system/src/query/plumbing.rs:328:18
    |
328 |                 .lookup(&key, |value, index| (value.clone(), index))
    |                  |
    |                  expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_query_system/src/query/caches.rs:36:8
    |
36  |     fn lookup(&self, key: &Self::Key) -> Option<(Self::Stored, DepNodeIndex)>;

error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
   --> compiler/rustc_query_system/src/query/plumbing.rs:329:18
    |
    |
329 |                 .unwrap_or_else(|_| panic!("value must be in cache after waiting"));
    |                  ^^^^^^^^^^^^^^ --- takes 1 argument
    |                  expected closure that takes 0 arguments

Some errors have detailed explanations: E0061, E0282, E0593.
For more information about an error, try `rustc --explain E0061`.
