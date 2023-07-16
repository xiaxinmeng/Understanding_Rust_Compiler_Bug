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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/str/mod.rs at line 243:
             self.len()
         } else {
             let lower_bound = index.saturating_sub(3);
-            let index = self.as_bytes()[lower_bound..=index].iter().rposition(|b| b.is_utf8_char_boundary());
+            let index = self.as_bytes()[lower_bound..=index]
+                .iter()
+                .rposition(|b| b.is_utf8_char_boundary());
 
             // SAFETY: we know that the character boundary will be within four bytes
             unsafe { index.unwrap_unchecked() }
Diff in /checkout/library/core/src/str/mod.rs at line 278:
             slice_error_fail(self, index, index)
         } else {
             let upper_bound = (index + 4).clamp(0, self.len());
-            let index = self.as_bytes()[index..upper_bound].iter().position(|b| b.is_utf8_char_boundary());
+            let index =
+                self.as_bytes()[index..upper_bound].iter().position(|b| b.is_utf8_char_boundary());
 
             // this case should only be reached if `upper_bound == self.len()`
             index.unwrap_or(upper_bound)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/array/iter.rs" "/checkout/library/core/src/slice/raw.rs" "/checkout/library/core/src/char/convert.rs" "/checkout/library/core/src/str/error.rs" "/checkout/library/core/src/str/lossy.rs" "/checkout/library/core/src/char/methods.rs" "/checkout/library/core/src/str/mod.rs" "/checkout/library/core/src/slice/index.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
