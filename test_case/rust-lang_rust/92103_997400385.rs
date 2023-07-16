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
Diff in /checkout/compiler/rustc_data_structures/src/sip128.rs at line 416:
             0..=0xFF => $target.short_write(value as u8),
             0x100..=0xFFFF => $target.short_write(value as u16),
             0x10000..=0xFFFFFFFF => $target.short_write(value as u32),
-            _ => $target.short_write(value as u64)
+            _ => $target.short_write(value as u64),
-    }
+    };
 }
 
 
 impl Hasher for SipHasher128 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/sso/either_iter.rs" "/checkout/compiler/rustc_data_structures/src/macros.rs" "/checkout/compiler/rustc_data_structures/src/sso/set.rs" "/checkout/compiler/rustc_data_structures/src/sip128.rs" "/checkout/compiler/rustc_data_structures/src/frozen.rs" "/checkout/compiler/rustc_data_structures/src/vec_map.rs" "/checkout/compiler/rustc_resolve/src/macros.rs" "/checkout/compiler/rustc_data_structures/src/base_n/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
