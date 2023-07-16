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
             }
         } else {
-            field.attrs
+            field
+                .attrs
                 .iter()
                 .map(move |attr| {
                     let name = attr.path.segments.last().unwrap().ident.to_string();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc/src/main.rs" "/checkout/compiler/rustc_macros/src/serialize.rs" "/checkout/compiler/rustc_macros/src/query.rs" "/checkout/compiler/rustc_macros/src/lib.rs" "/checkout/compiler/rustc_macros/src/symbols.rs" "/checkout/compiler/rustc_macros/src/diagnostics/diagnostic.rs" "/checkout/compiler/rustc_macros/src/diagnostics/mod.rs" "/checkout/compiler/rustc_resolve/src/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
