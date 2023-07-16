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
Diff in /checkout/library/core/src/time.rs at line 1394:
 impl FromFloatSecsError {
     const fn description(&self) -> &'static str {
         match self.kind {
-            FromFloatSecsErrorKind::NonFinite => "non-finite value when converting float to duration",
+            FromFloatSecsErrorKind::NonFinite => {
+                "non-finite value when converting float to duration"
+            }
             FromFloatSecsErrorKind::Overflow => "overflow when converting float to duration",
             FromFloatSecsErrorKind::Negative => "negative value when converting float to duration",
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/slice/mod.rs" "/checkout/library/core/src/prelude/mod.rs" "/checkout/library/core/src/tuple.rs" "/checkout/library/core/src/prelude/v1.rs" "/checkout/library/core/src/default.rs" "/checkout/library/core/src/option.rs" "/checkout/library/core/src/slice/raw.rs" "/checkout/library/core/src/time.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
