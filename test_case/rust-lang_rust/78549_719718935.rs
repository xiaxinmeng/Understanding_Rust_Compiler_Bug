
 ../../prebuilt/third_party/rust/linux-x64/bin/rustc --color=always --crate-name trust_dns_proto ../../third_party/rust_crates/vendor/trust-dns-proto/src/lib.rs --crate-type rlib --emit=dep-info=obj/t...
error: internal compiler error: compiler/rustc_mir_build/src/thir/pattern/_match.rs:934:18: trying to compare incompatible constructors Single and Str(Const { ty: &str, val: Value(Slice { data: Alloca...

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (388ef3490 2020-10-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z dep-info-omit-d-target -Z allow-features= -Z panic_abort_tests -C linker=/b/s/w/ir/k/fuchsia/prebuilt/third_party/clang/linux-x64/bin/lld -C link-arg=--sysroot=gen/build/confi...

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_match] match-checking `<rr::rdata::caa::Property as std::convert::From<std::string::String>>::from`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
