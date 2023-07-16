
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:603: collection encountered polymorphic constant
  --> /home/programming/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/src/int/mod.rs:50:5
   |
50 |     const ONE: Self;
   |     ^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
