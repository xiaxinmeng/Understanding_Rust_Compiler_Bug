
error[E0658]: `impl Trait` in type aliases is unstable
  --> src/lib.rs:15:10
   |
15 | type A = impl Fn(Matrix<<DefaultAllocator as Allocator<()>>::Buffer>);
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

error: internal compiler error: src/librustc_mir/borrow_check/type_check/free_region_relations.rs:318: failed to compute implied bounds A

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:904:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.1 (c7087fe00 2020-06-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `ice`.
