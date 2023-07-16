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
Diff in /checkout/library/core/src/time.rs at line 770:
                 let t = fract >> (20 + exp.abs());
                 let nanos = (NANOS_PER_SEC as u64 * t) / (1u64 << 32);
                 (0, nanos as u32)
+            }
+            }
             0..=51 => {
                 let secs = fract >> (FRACT_BITS - exp as u64);
                 let t = ((fract << exp) & FRACT_MASK) >> 20;
Diff in /checkout/library/core/src/time.rs at line 777:
                 let nanos = (NANOS_PER_SEC as u64 * t) / (1u64 << 32);
                 (secs, nanos as u32)
+            }
+            }
             52..=63 => {
                 let secs = fract << (exp as u64 - FRACT_BITS);
                 (secs, 0)
Diff in /checkout/library/core/src/time.rs at line 853:
                 let t = (fract << 9) >> exp.abs();
                 let nanos = (NANOS_PER_SEC as u64 * t) / (1u64 << 32);
                 (0, nanos as u32)
+            }
+            }
             0..=22 => {
                 let secs = fract >> (FRACT_BITS - exp as u32);
                 let t = (fract << exp) & FRACT_MASK as u64;
Diff in /checkout/library/core/src/time.rs at line 860:
                 let nanos = (NANOS_PER_SEC as u64 * t) / (1u64 << FRACT_BITS);
                 (secs, nanos as u32)
+            }
+            }
             23..=63 => {
                 let secs = fract << (exp as u32 - FRACT_BITS);
                 (secs, 0)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/fmt/num.rs" "/checkout/library/core/src/fmt/float.rs" "/checkout/library/core/src/fmt/mod.rs" "/checkout/library/core/src/stream/stream/mod.rs" "/checkout/library/core/src/primitive.rs" "/checkout/library/core/src/time.rs" "/checkout/library/core/src/iter/mod.rs" "/checkout/library/core/src/fmt/nofloat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
