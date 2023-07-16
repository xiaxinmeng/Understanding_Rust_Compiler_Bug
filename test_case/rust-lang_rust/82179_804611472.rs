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
     }
 }
 
-
 /// An error which can be returned when converting a floating-point value of seconds
 /// into a [`Duration`].
Diff in /checkout/library/core/src/time.rs at line 1207:
Diff in /checkout/library/core/src/time.rs at line 1207:
 impl fmt::Display for FromSecsError {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         let s = match self.kind {
-            FromSecsErrorKind::NonFinite => "got non-finite value when converting float to duration",
+            FromSecsErrorKind::NonFinite => {
+                "got non-finite value when converting float to duration"
+            }
             FromSecsErrorKind::Overflow => "overflow when converting float to duration",
-            FromSecsErrorKind::Underflow => "underflow when converting float to duration"
+            FromSecsErrorKind::Underflow => "underflow when converting float to duration",
         };
         fmt::Display::fmt(s, f)
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/unit.rs" "/checkout/library/core/src/prelude/v1.rs" "/checkout/library/core/src/alloc/mod.rs" "/checkout/library/core/src/result.rs" "/checkout/library/core/src/alloc/layout.rs" "/checkout/library/core/src/ffi.rs" "/checkout/library/core/src/alloc/global.rs" "/checkout/library/core/src/time.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
