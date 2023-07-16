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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/pin.rs at line 1146:
     //
     // Finally, we don't hit problems _w.r.t._ the privacy of the `pointer` field, or the
     // unqualified `Pin` name, thanks to `decl_macro`s being _fully_ hygienic (`def_site` hygiene).
-    Pin {
-        pointer: &mut { $value },
-    }
+    Pin { pointer: &mut { $value } }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/iter/adapters/copied.rs" "/checkout/library/core/tests/iter/adapters/filter.rs" "/checkout/library/core/src/internal_macros.rs" "/checkout/library/core/src/pin.rs" "/checkout/library/core/tests/iter/adapters/peekable.rs" "/checkout/library/core/tests/iter/adapters/cloned.rs" "/checkout/library/core/src/ffi.rs" "/checkout/library/core/src/panic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
