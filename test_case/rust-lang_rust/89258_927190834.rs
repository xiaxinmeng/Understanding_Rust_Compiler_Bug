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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/char/methods.rs at line 344:
             // Force the 6th bit to be set to ensure ascii is lower case.
             digit = (self as u32 | 0b10_0000).wrapping_sub('a' as u32).saturating_add(10);
         }
-        if digit < radix {
-            Some(digit)
-            None
-        }
-        }
+        if digit < radix { Some(digit) } else { None }
 
 
     /// Returns an iterator that yields the hexadecimal Unicode escape of a
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/char/methods.rs" "/checkout/library/core/src/char/decode.rs" "/checkout/library/core/src/char/mod.rs" "/checkout/library/core/src/char/convert.rs" "/checkout/library/core/src/panicking.rs" "/checkout/library/core/src/clone.rs" "/checkout/library/core/src/panic.rs" "/checkout/library/core/src/borrow.rs"` failed.
Build completed unsuccessfully in 0:00:25
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
