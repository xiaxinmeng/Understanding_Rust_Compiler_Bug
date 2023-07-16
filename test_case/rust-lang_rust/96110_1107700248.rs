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
Diff in /checkout/library/core/src/num/mod.rs at line 353:
         issue = "95969"
     )]
     pub const fn to_ascii_digit(self, radix: u8) -> Option<u8> {
-        if let Some(n) = (self as char).to_digit(radix.into()) {
-            Some(n as u8)
-            None
-        }
-        }
+        if let Some(n) = (self as char).to_digit(radix.into()) { Some(n as u8) } else { None }
 
 
     /// Assumes self is ascii
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/dec2flt/common.rs" "/checkout/library/core/src/num/int_log10.rs" "/checkout/library/core/src/num/dec2flt/parse.rs" "/checkout/library/core/src/num/dec2flt/table.rs" "/checkout/library/core/src/num/nonzero.rs" "/checkout/library/core/src/num/dec2flt/float.rs" "/checkout/library/core/src/num/fmt.rs" "/checkout/library/core/src/num/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
