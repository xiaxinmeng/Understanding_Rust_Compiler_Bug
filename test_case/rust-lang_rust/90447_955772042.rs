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
 
-    Crate {
-        module,
-        externs,
-        external_traits: cx.external_traits.clone(),
-        collapsed: false,
-    }
+    Crate { module, externs, external_traits: cx.external_traits.clone(), collapsed: false }
 
 fn external_generic_args(
 fn external_generic_args(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/compiler/rustc_serialize/src/serialize.rs" "/checkout/compiler/rustc_serialize/src/json.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
