
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler\rustc_metadata\src\rmeta\def_path_hash_map.rs:18:85
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0 (02072b482 2022-01-11) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C linker=lld-link -C incremental -C target-feature=+crt-static -C link-args=-fuse-ld=lld-link -C lto=no --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `util::sdf_mesh::SdfManager: core::marker::Send`
#1 [check_item_well_formed] checking that `backend::<impl at components\render-vulkan\src\backend.rs:837:1: 1345:2>` is well-formed
#2 [analysis] running analysis passes on this crate
end of query stack
