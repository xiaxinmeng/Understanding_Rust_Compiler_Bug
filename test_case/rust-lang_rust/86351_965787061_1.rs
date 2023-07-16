
error: internal compiler error: /rustc/8b09ba6a5d5c644fe0f1c27c7f9c80b334241707/compiler/rustc_trait_selection/src/traits/util.rs:314:13: get_vtable_index_of_object_method: DefId(0:5 ~ scratch[c192]::TestTrait::func) was not found

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (8b09ba6a5 2021-11-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolve_instance] resolving instance `<dyn TestTrait<MyType = T> as TestTrait>::func`
#1 [mir_built] building MIR for `<impl at src/lib.rs:6:1: 13:2>::other_func`
end of query stack
