
warning: unreachable expression
 --> mutant.rs:3:27
  |
3 |     let x: Box<[isize]> = box { loop {} };
  |                           ^^^^^^-------^^
  |                           |     |
  |                           |     any code following this expression is unreachable
  |                           unreachable expression
  |
  = note: `#[warn(unreachable_code)]` on by default

warning: unused variable: `x`
 --> mutant.rs:3:9
  |
3 |     let x: Box<[isize]> = box { loop {} };
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: 2 warnings emitted

error: internal compiler error: broken MIR in DefId(0:3 ~ mutant[317d]::main) (NoSolution): could not prove Binder(TraitPredicate(<[isize] as std::marker::Sized>), [])
 --> mutant.rs:3:27
  |
3 |     let x: Box<[isize]> = box { loop {} };
  |                           ^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:253:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1022:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (6d820866a 2021-06-29) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
