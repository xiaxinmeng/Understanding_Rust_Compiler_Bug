plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_monomorphize/src/collector.rs at line 555:
     if s.chars().nth(33).is_some() {
         let shrunk = format!("{}", ty::ShortInstance(instance, 4));
         if shrunk == s {
-            return (s, None)
+            return (s, None);
 
 
         let path = tcx.output_filenames(()).temp_path_ext("long-type.txt", None);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_monomorphize/src/collector.rs" "/checkout/compiler/rustc_index/src/vec/tests.rs" "/checkout/compiler/rustc_symbol_mangling/src/errors.rs" "/checkout/compiler/rustc_symbol_mangling/src/test.rs" "/checkout/compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs" "/checkout/compiler/rustc_symbol_mangling/src/legacy.rs" "/checkout/compiler/rustc_symbol_mangling/src/typeid.rs" "/checkout/compiler/rustc_index/src/interval.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
