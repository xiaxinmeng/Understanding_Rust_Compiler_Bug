
> error: internal compiler error: unexpected panic
> 
> note: the compiler unexpectedly panicked. this is a bug.
> 
> note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
> 
> note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-apple-darwin
> 
> note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin
> 
> note: some of the compiler flags provided by cargo are hidden
> 
> query stack during panic:
> #0 [collect_mod_item_types] collecting item types in top-level module
> #1 [analysis] running analysis passes on this crate
> end of query stack
> 