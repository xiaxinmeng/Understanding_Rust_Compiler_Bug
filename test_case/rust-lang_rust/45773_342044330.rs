
[01:22:25] ---- [pretty] run-pass/range_inclusive.rs stdout ----
[01:22:25] 	
[01:22:25] error: pretty-printing failed in round 1 revision None
[01:22:25] status: exit code: 101
[01:22:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zunstable-options" "--unpretty" "normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/range_inclusive.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:22:25] stdout:
[01:22:25] ------------------------------------------
[01:22:25] 
[01:22:25] ------------------------------------------
[01:22:25] stderr:
[01:22:25] ------------------------------------------
[01:22:25] error: `...` syntax cannot be used in expressions
[01:22:25]   --> <anon>:20:56
[01:22:25]    |
[01:22:25] 20 | fn return_range_to() -> RangeToInclusive<i32> { return ...1; }
[01:22:25]    |                                                        ^^^
[01:22:25]    |
[01:22:25]    = help: Use `..` if you need an exclusive range (a < b)
[01:22:25]    = help: or `..=` if you need an inclusive range (a <= b)
[01:22:25] 
[01:22:25] error: internal compiler error: unexpected panic
[01:22:25] 
[01:22:25] note: the compiler unexpectedly panicked. this is a bug.
[01:22:25] 
[01:22:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:22:25] 
[01:22:25] note: rustc 1.23.0-dev running on x86_64-unknown-linux-gnu
[01:22:25] 
[01:22:25] thread 'rustc' panicked at 'parse_prefix_range_expr: token DotDotDot is not DotDot/DotDotEq', /checkout/src/libsyntax/parse/parser.rs:3020:8
[01:22:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:25] 
[01:22:25] 
[01:22:25] ------------------------------------------
[01:22:25] 
[01:22:25] thread '[pretty] run-pass/range_inclusive.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2497:8
[01:22:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
