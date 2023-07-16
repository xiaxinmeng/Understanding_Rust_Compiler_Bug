
 thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<smallvec::SmallVec<[u8; _]> as std::ops::Index<std::ops::RangeFull>>)`,
 right: `Binder(<smallvec::SmallVec<[u8; 36]> as std::ops::Index<std::ops::RangeFull>>)`', compiler/rustc_trait_selection/src/traits/codegen/mod.rs:30:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (4d247ad7d 2020-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Index` fulfills its obligations
#1 [resolve_instance] resolving instance `<smallvec::SmallVec<[u8; _]> as std::ops::Index<std::ops::RangeFull>>::index`
end of query stack
[RUSTC-TIMING] rustc_ap_rustc_data_structures test:false 1.603
error: could not compile **`rustc-ap-rustc_data_structures`**
