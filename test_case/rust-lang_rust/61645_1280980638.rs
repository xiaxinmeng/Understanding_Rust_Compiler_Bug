
warning: `playground` (bin "playground") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
     Running `target/debug/playground`
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:18:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/da7ffa2d1df2daf747b8e4bdf5659c942abb22af/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/da7ffa2d1df2daf747b8e4bdf5659c942abb22af/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/da7ffa2d1df2daf747b8e4bdf5659c942abb22af/library/core/src/panicking.rs:48:5
   3: core::option::Option<T>::unwrap
             at /rustc/da7ffa2d1df2daf747b8e4bdf5659c942abb22af/library/core/src/option.rs:775:21
   4: playground::main
             at ./src/main.rs:14:9 <------- this is what the issue is complaining about
   5: core::ops::function::FnOnce::call_once
             at /rustc/da7ffa2d1df2daf747b8e4bdf5659c942abb22af/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
