plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/test.rs at line 1524:
             cmd.args(&test_args);
 
-
         if builder.is_verbose() {
         if builder.is_verbose() {
             cmd.arg("--verbose");
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/library/panic_unwind/src/gcc.rs" "/checkout/library/panic_unwind/src/hermit.rs" "/checkout/library/panic_unwind/src/dwarf/eh.rs" "/checkout/library/panic_unwind/src/miri.rs" "/checkout/library/panic_unwind/src/dummy.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
