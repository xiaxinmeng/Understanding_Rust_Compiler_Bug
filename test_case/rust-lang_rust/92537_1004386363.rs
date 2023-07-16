
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /rustc/efec545293b9263be9edfb283a7aa66350b3acbf\compiler\rustc_middle\src\ty\sty.rs:1038:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (efec54529 2021-12-04) running on x86_64-pc-windows-msvc


note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `[nodes::VNode<'_>]` is `Sized`
#1 [layout_of] computing layout of `&[nodes::VNode<'_>]`
end of query stack
error: could not compile `dioxus-core`
