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
Diff in /checkout/compiler/rustc_llvm/build.rs at line 24:
     "bpf",
 ];
 
-const REQUIRED_COMPONENTS: &[&str] = &[
-    "ipo",
-    "bitreader",
-    "bitwriter",
-    "linker",
-    "asmparser",
-    "lto",
-    "coverage",
-];
-];
+const REQUIRED_COMPONENTS: &[&str] =
+    &["ipo", "bitreader", "bitwriter", "linker", "asmparser", "lto", "coverage", "instrumentation"];
 
 fn detect_llvm_link() -> (&'static str, &'static str) {
     // Force the link mode we want, preferring static by default, but
Diff in /checkout/compiler/rustc_llvm/build.rs at line 165:
     let target = env::var("TARGET").expect("TARGET was not set");
     let host = env::var("HOST").expect("HOST was not set");
     let is_crossed = target != host;
 
 
     let components = output(Command::new(&llvm_config).arg("--components"));
     let mut components = components.split_whitespace().collect::<Vec<_>>();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/constraints/mod.rs" "/checkout/compiler/rustc_serialize/src/leb128.rs" "/checkout/compiler/rustc_llvm/build.rs" "/checkout/compiler/rustc_serialize/src/opaque.rs" "/checkout/compiler/rustc_serialize/src/collection_impls.rs" "/checkout/compiler/rustc_serialize/src/lib.rs" "/checkout/compiler/rustc_plugin_impl/src/load.rs" "/checkout/compiler/rustc_serialize/tests/leb128.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
