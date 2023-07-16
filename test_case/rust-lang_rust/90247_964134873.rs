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
Diff in /checkout/library/core/src/time.rs at line 817:
                 let secs = mant << (exp - MANT_BITS);
                 (secs, 0)
-            _ => return Err(FromFloatSecsError {
-            _ => return Err(FromFloatSecsError {
-                kind: FromFloatSecsErrorKind::OverflowOrNan,
-            }),
+            _ => return Err(FromFloatSecsError { kind: FromFloatSecsErrorKind::OverflowOrNan }),
 
 
         Ok(Duration { secs, nanos })
Diff in /checkout/library/core/src/time.rs at line 935:
                 let secs = u64::from(mant) << (exp - MANT_BITS);
                 (secs, 0)
-            _ => return Err(FromFloatSecsError {
-            _ => return Err(FromFloatSecsError {
-                kind: FromFloatSecsErrorKind::OverflowOrNan,
-            }),
+            _ => return Err(FromFloatSecsError { kind: FromFloatSecsErrorKind::OverflowOrNan }),
 
 
         Ok(Duration { secs, nanos })
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/enumerate.rs" "/checkout/library/core/src/time.rs" "/checkout/library/core/tests/num/uint_macros.rs" "/checkout/library/core/tests/num/i32.rs" "/checkout/library/core/tests/num/i64.rs" "/checkout/library/core/tests/num/u8.rs" "/checkout/library/core/tests/num/int_macros.rs" "/checkout/library/core/src/iter/adapters/take.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
