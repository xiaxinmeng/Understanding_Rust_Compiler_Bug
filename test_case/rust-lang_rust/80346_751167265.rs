
docker.io/triscuitcircuit/build-rust-nightly:latest
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8) and no specific platform was requested
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling autocfg v1.0.1
   Compiling pkg-config v0.3.19
   Compiling gimli v0.23.0
   Compiling rustc-demangle v0.1.18
   Compiling object v0.22.0
   Compiling proc-macro-nested v0.1.6
   Compiling serde_json v1.0.60
   Compiling semver v0.9.0
   Compiling byteorder v1.3.4
   Compiling form_urlencoded v1.0.0
   Compiling http v0.2.2
   Compiling failure_derive v0.1.8
   Compiling stdweb-internal-runtime v0.1.5
   Compiling yew v0.17.4
   Compiling heck v0.3.2
   Compiling proc-macro2 v1.0.24
thread 'rustc' panicked at 'invalid terminator state', compiler/rustc_middle/src/mir/mod.rs:1221:34
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (0edce6f4b 2020-12-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_const] processing MIR for `header::name::parse_hdr`
#1 [mir_promoted] processing `header::name::parse_hdr`
end of query stack
error: could not compile `http`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
qemu: uncaught target signal 11 (Segmentation fault) - core dumped`