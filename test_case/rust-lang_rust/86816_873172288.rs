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
Diff in /checkout/library/proc_macro/src/lib.rs at line 853:
     /// (`Alone`) so the operator has certainly ended.
     #[stable(feature = "proc_macro_lib2", since = "1.29.0")]
     pub fn spacing(&self) -> Spacing {
-        if self.0.joint {
-            Spacing::Joint
-            Spacing::Alone
-        }
-        }
+        if self.0.joint { Spacing::Joint } else { Spacing::Alone }
 
 
     /// Returns the span for this punctuation character.
Diff in /checkout/library/proc_macro/src/lib.rs at line 973:
 #[stable(feature = "proc_macro_lib2", since = "1.29.0")]
 impl fmt::Display for Ident {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-        if self.0.is_raw {
-            write!(f, "r#{}", self.0.sym)
-        } else {
-            write!(f, "{}", self.0.sym)
-        }
+        if self.0.is_raw { write!(f, "r#{}", self.0.sym) } else { write!(f, "{}", self.0.sym) }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_unwind/src/dwarf/tests.rs" "/checkout/library/panic_unwind/src/dwarf/mod.rs" "/checkout/library/panic_unwind/src/hermit.rs" "/checkout/library/panic_unwind/src/dummy.rs" "/checkout/library/panic_unwind/src/dwarf/eh.rs" "/checkout/library/proc_macro/src/lib.rs" "/checkout/library/proc_macro/src/quote.rs" "/checkout/library/proc_macro/src/bridge/buffer.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
