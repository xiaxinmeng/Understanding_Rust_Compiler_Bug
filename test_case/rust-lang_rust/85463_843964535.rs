
thread 'frozen::in_par_get_set_cancelation' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', tests/parallel/frozen.rs:64:33
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/core/src/panicking.rs:92:14
   2: core::result::unwrap_failed
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/core/src/result.rs:1355:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/core/src/result.rs:1037:23
   4: parallel::frozen::in_par_get_set_cancelation
             at ./tests/parallel/frozen.rs:64:18
   5: parallel::frozen::in_par_get_set_cancelation::{{closure}}
             at ./tests/parallel/frozen.rs:13:1
   6: core::ops::function::FnOnce::call_once
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/core/src/ops/function.rs:227:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/4e3e6db011c5b482d2bef8ba02274657f93b5e0d/library/core/src/ops/function.rs:227:5
