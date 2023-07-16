
 Documenting libc v0.2.34
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.1 (05e2e1c41 2017-11-22) running on x86_64-apple-darwin

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'could not lock `/Volumes/andrew/Documents/rusty_projects/guessing_game/target/doc/.lock`: Operation not supported (os error 45)', src/librustc_data_structures/flock.rs:363:12
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: std::panicking::begin_panic_fmt
   7: rustc_data_structures::flock::<impl rustc_data_structures::flock::imp::Lock>::panicking_new
   8: rustdoc::html::render::write_shared
   9: rustdoc::html::render::run

error: Could not document `libc`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name libc /Users/andrew/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.34/src/lib.rs -o /Volumes/andrew/Documents/rusty_projects/guessing_game/target/doc --cfg feature="default" --cfg feature="use_std" -L dependency=/Volumes/andrew/Documents/rusty_projects/guessing_game/target/debug/deps` (exit code: 101)

</details>