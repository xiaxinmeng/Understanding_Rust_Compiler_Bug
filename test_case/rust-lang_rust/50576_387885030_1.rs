
error[E0268]: `break` outside of loop
 --> <anon>:2:16
  |
2 |     Vec::<[u8; break]>::new();
  |                ^^^^^ cannot break outside of a loop

error: internal compiler error: librustc_mir/hair/cx/expr.rs:544: invalid loop id for break: not inside loop scope

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to 2 previous errors
