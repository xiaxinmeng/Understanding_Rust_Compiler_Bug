
[roc@localhost relocation-test]$ RUSTFLAGS="-C code-model=medium" cargo build
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling libc v0.2.74
   Compiling syn v1.0.38
   Compiling cfg-if v0.1.10
   Compiling memchr v2.3.3
   Compiling version_check v0.9.2
   Compiling log v0.4.11
   Compiling lazy_static v1.4.0
   Compiling slab v0.4.2
   Compiling futures-core v0.3.5
   Compiling bytes v0.5.6
   Compiling proc-macro-hack v0.5.18
   Compiling futures-sink v0.3.5
   Compiling pin-project-internal v0.4.23
error: could not compile `futures-core`.

Caused by:
  process didn't exit successfully: `rustc --crate-name futures_core --edition=2018 /home/roc/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-core-0.3.5/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -Cembed-bitcode=no -C debuginfo=2 -C debug-assertions=on --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=f2a9c760bbf5398e -C extra-filename=-f2a9c760bbf5398e --out-dir /home/roc/relocation-test/target/debug/deps -L dependency=/home/roc/relocation-test/target/debug/deps --cap-lints allow -C code-model=medium` (signal: 11, SIGSEGV: invalid memory reference)
