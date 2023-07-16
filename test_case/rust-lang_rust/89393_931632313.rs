
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (6dc08b909 2021-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z dep-info-omit-d-target -Z mutable-noalias=off -Z panic_abort_tests -Z allow-features=min_specialization,specialization -C linker=../../prebuilt/third_party/clang/linux-x64/bin/lld -C link-arg=--sysroot=gen/zircon/public/sysroot/cpp -C link-arg=-L../../prebuilt/third_party/clang/linux-x64/bin/../lib/x86_64-unknown-fuchsia -C link-arg=-L../../prebuilt/third_party/clang/linux-x64/lib/clang/14.0.0/lib/x86_64-unknown-fuchsia -C link-arg=--pack-dyn-relocs=relr -C link-arg=-dynamic-linker=ld.so.1 -C link-arg=--icf=all -C force-frame-pointers -C opt-level=0 -C debuginfo=2 -C debug-assertions=yes -C link-args=-zstack-size=0x200000 -C panic=abort -C force-unwind-tables=yes -C link-arg=-Bdynamic -C link-arg=x64-shared/lib.unstripped/libcrypto.so --crate-type rlib

query stack during panic:
#0 [check_match] match-checking `ip::gmp::igmp::run_action`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
