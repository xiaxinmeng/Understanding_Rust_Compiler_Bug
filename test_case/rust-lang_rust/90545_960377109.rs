plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/slice.rs at line 1127:
             } else {
                 // ascending
                 let mut i = 2;
-                while i < len && ! gt!(v, i - 1, i, is_less) {
+                while i < len && !gt!(v, i - 1, i, is_less) {
                     i += 1;
                 i
                 i
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/sync/tests.rs" "/checkout/library/alloc/tests/vec.rs" "/checkout/library/alloc/src/str.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/tests/slice.rs" "/checkout/library/alloc/src/slice.rs" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/alloc/src/borrow.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
