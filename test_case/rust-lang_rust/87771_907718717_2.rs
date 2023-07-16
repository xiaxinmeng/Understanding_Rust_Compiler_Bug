
warning: variable `x` is assigned to, but never used
 --> main.rs:2:9
  |
2 |     let mut x;
  |         ^^^^^
  |
  = note: `#[warn(unused_variables)]` on by default
  = note: consider using `_x` instead

warning: value assigned to `x` is never read
 --> main.rs:3:5
  |
3 |     x = x = true;
  |     ^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?

warning: value assigned to `x` is never read
 --> main.rs:3:9
  |
3 |     x = x = true;
  |         ^
  |
  = help: maybe it is overwritten before being read?

warning: 3 warnings emitted

error: internal compiler error[E0308]: mismatched types
 --> main.rs:3:9
  |
3 |     x = x = true;
  |         ^^^^^^^^ expected `bool`, found `()`

error: internal compiler error: broken MIR in DefId(0:3 ~ main[317d]::main) (_1 = const ()): bad assignment (bool = ()): NoSolution
 --> main.rs:3:5
  |
3 |     x = x = true;
  |     ^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:249:27

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3 ~ main[317d]::main), const_param_did: None }) (end of phase Optimization) at bb0[2]:
encountered `Assign((_1, const ()))` with incompatible types:
left-hand side has type: bool
right-hand side has type: ()
 --> main.rs:3:5
  |
3 |     x = x = true;
  |     ^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir/src/transform/validate.rs:120:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 (2fd73fabe 2021-03-23) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
