
   Compiling playground v0.0.1 (/playground)
warning: unused variable: `x`
 --> src/main.rs:9:9
  |
9 |     let x: Void = unsafe { mem::transmute(()) };
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: the type `Void` does not permit zero-initialization
 --> src/main.rs:9:28
  |
9 |     let x: Void = unsafe { mem::transmute(()) };
  |                            ^^^^^^^^^^^^^^^^^^
  |                            |
  |                            this code causes undefined behavior when executed
  |                            help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
  |
  = note: `#[warn(invalid_value)]` on by default
  = note: enums with no variants have no valid value

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/playground`
timeout: the monitored command dumped core
/playground/tools/entrypoint.sh: line 11:     7 Illegal instruction     timeout --signal=KILL ${timeout} "$@"
