plain

failures:

---- collisions::collision_doc_profile_split stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
thread 'collisions::collision_doc_profile_split' panicked at '
Expected: execs
    but: differences:
  5 - |[RUNNING] `rustc --crate-name common [..]|
    + |   Compiling pm v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/pm)|

  6 - |[COMPILING] pm v0.1.0 [..]|
    + |     Running `rustc --crate-name pm pm/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no -C debuginfo=2 -C metadata=d426ce811325646e -C extra-filename=-d426ce811325646e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/target/debug/deps --extern common=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/target/debug/deps/libcommon-a36ddaf9ab4365ca.rlib --extern proc_macro`|

  7 - |[RUNNING] `rustc --crate-name pm [..]|
    + |     Running `rustc --crate-name common /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/home/.cargo/registry/src/-f61389b45965a7a6/common-1.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C metadata=eb33ff1e31213df8 -C extra-filename=-eb33ff1e31213df8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t526/foo/target/debug/deps --cap-lints allow`|

other output:
``', src/tools/cargo/crates/cargo-test-support/src/lib.rs:729:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:24:26
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
