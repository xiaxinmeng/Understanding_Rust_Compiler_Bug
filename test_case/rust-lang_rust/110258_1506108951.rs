code
Compiling rust_bug v0.1.0 (C:\Users\void\Documents\rust_bug)
error[E0631]: type mismatch in closure arguments
   --> src\main.rs:5:10
    |
5   |     pool.scope(|_s: Scope<()>| {});
    |          ^^^^^ --------------- found signature defined here
    |          |
    |          expected due to this
    |
    = note: expected closure signature `for<'scope> fn(&'scope bevy::tasks::Scope<'scope, '_, _>) -> _`
note: required by a bound in `TaskPool::scope`
   --> C:\Users\void\.cargo\registry\src\index.crates.io-6f17d22bba15001f\bevy_tasks-0.10.1\src\task_pool.rs:280:12
    |
280 |         F: for<'scope> FnOnce(&'scope Scope<'scope, 'env, T>),
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `TaskPool::scope`
help: consider borrowing the argument
    |
5   |     pool.scope(|_s: &Scope<()>| {});
    |                     +

For more information about this error, try `rustc --explain E0631`.
error: could not compile `rust_bug` (bin "rust_bug") due to previous error
