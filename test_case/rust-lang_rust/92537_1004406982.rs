
☻~/rust/dioxus $ cargo build
   Compiling dioxus-html v0.1.2 (...)
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\compiler\rustc_middle\src\ty\sty.rs:974:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `[nodes::VNode<'_>]` is `Sized`
#1 [layout_of] computing layout of `&[nodes::VNode<'_>]`
end of query stack
error: could not compile `dioxus-core`
warning: build failed, waiting for other jobs to finish...
error: build failed
