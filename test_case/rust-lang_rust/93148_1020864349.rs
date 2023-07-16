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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/uniq.rs at line 54:
     fn cmp(&self, other: &Uniq<T>) -> Ordering {
         // Pointer equality implies equality, due to the uniqueness constraint,
         // but the contents must be compared otherwise.
-        if self == other { Ordering::Equal } else {
+        if self == other {
+            Ordering::Equal
+        } else {
             let res = self.cmp(other);
             debug_assert!(res != Ordering::Equal);
             res
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/hir/mod.rs" "/checkout/compiler/rustc_middle/src/uniq.rs" "/checkout/compiler/rustc_middle/src/thir.rs" "/checkout/compiler/rustc_middle/src/lib.rs" "/checkout/compiler/rustc_middle/src/thir/abstract_const.rs" "/checkout/compiler/rustc_middle/src/thir/visit.rs" "/checkout/compiler/rustc_parse_format/src/tests.rs" "/checkout/compiler/rustc_middle/src/middle/exported_symbols.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
