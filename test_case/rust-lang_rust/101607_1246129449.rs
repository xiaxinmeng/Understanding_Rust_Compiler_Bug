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
Diff in /checkout/library/core/src/ffi/mod.rs at line 22:
 #[unstable(feature = "cstr_macro", issue = "101607")]
 pub use self::c_str::{
-    cstr,
-    cstr,
     __cstr_macro_impl_as_bytes, __cstr_macro_impl_from_bytes_with_nul,
-    __cstr_macro_impl_to_bytes_with_nul,
+    __cstr_macro_impl_to_bytes_with_nul, cstr,
 
 
 macro_rules! type_alias_no_nz {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/ffi/mod.rs" "/checkout/library/core/src/task/ready.rs" "/checkout/library/core/tests/simd.rs" "/checkout/library/core/tests/ptr.rs" "/checkout/library/core/src/task/poll.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/tests/convert.rs" "/checkout/library/core/src/task/wake.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
