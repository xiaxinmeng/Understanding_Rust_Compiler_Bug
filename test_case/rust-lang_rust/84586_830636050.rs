plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---

warning: associated function is never used: `relocate_for_jit`
  --> src/debuginfo/emit.rs:71:19
   |
71 |     pub(super) fn relocate_for_jit(mut self, jit_module: &cranelift_jit::JITModule) -> Vec<u8> {
   |
   = note: `#[warn(dead_code)]` on by default

warning: 2 warnings emitted
---
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 623 error codes
* highest error code: E0781
tidy error: /checkout/src/ci/scripts/should-skip-this.sh:21: line longer than 100 chars
Found 0 error codes with no tests
Done!
* 330 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:10
