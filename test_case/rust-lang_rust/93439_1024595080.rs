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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 953:
 
     let mut ret = FxHashSet::default();
     ret.reserve(7); // the minimum number of insertions
+    // Target bindings.
+    // Target bindings.
     ret.insert((sym::target_os, Some(Symbol::intern(os))));
     for fam in &sess.target.families {
         ret.insert((sym::target_family, Some(Symbol::intern(fam))));
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1133:
 
 
     fn longer(a: S, b: S) -> S {
-        if a.len() > b.len() {
-        } else {
-            b
-        }
-        }
+        if a.len() > b.len() { a } else { b }
 
 
     pub fn opt_s(a: S, b: S, c: S, d: S) -> R {
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1681:
         .into_iter()
         .flat_map(|(i, s)| {
             // NB: This can match a string without `=`.
-            if let Some("opt-level") = s.splitn(2, '=').next() {
-                Some(i)
-                None
-            }
-            }
+            if let Some("opt-level") = s.splitn(2, '=').next() { Some(i) } else { None }
         .max();
         .max();
     if max_o > max_c {
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1723:
         .into_iter()
         .flat_map(|(i, s)| {
             // NB: This can match a string without `=`.
-            if let Some("debuginfo") = s.splitn(2, '=').next() {
-                Some(i)
-                None
-            }
-            }
+            if let Some("debuginfo") = s.splitn(2, '=').next() { Some(i) } else { None }
         .max();
         .max();
     if max_g > max_c {
Diff in /checkout/compiler/rustc_session/src/config.rs at line 2346:
 
         // Only use this directory if it has a file we can expect to always find.
         // Only use this directory if it has a file we can expect to always find.
-        if candidate.join("library/std/src/lib.rs").is_file() {
-            Some(candidate)
-            None
-        }
-        }
+        if candidate.join("library/std/src/lib.rs").is_file() { Some(candidate) } else { None }
 
 
     let working_dir = std::env::current_dir().unwrap_or_else(|e| {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/context.rs" "/checkout/compiler/rustc_codegen_llvm/src/va_arg.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
