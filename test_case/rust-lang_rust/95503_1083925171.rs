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
Diff in /checkout/src/bootstrap/test.rs at line 1903:
         let krate = builder.crate_paths[&run.path];
         let test_kind = builder.kind.into();
-        builder.ensure(CrateLibrustc {
-            compiler,
-            target: run.target,
-            test_kind,
-            test_kind,
-            krate,
-        });
+        builder.ensure(CrateLibrustc { compiler, target: run.target, test_kind, krate });
 
 
     fn run(self, builder: &Builder<'_>) {
Diff in /checkout/src/bootstrap/test.rs at line 1945:
         let test_kind = builder.kind.into();
         let krate = builder.crate_paths[&run.path];
-        builder.ensure(Crate {
-            compiler,
-            target: run.target,
-            mode: Mode::Std,
-            mode: Mode::Std,
-            test_kind,
-            krate,
-        });
+        builder.ensure(Crate { compiler, target: run.target, mode: Mode::Std, test_kind, krate });
 
 
     /// Runs all unit tests plus documentation tests for a given crate defined
Diff in /checkout/src/bootstrap/doc.rs at line 603:
         cargo.rustdocflag("--extern-html-root-url");
         cargo.rustdocflag("ena=https://docs.rs/ena/latest/");
 
-        let root_crates =  if paths.is_empty() {
+        let root_crates = if paths.is_empty() {
             vec![
                 INTERNER.intern_str("rustc_driver"),
                 INTERNER.intern_str("rustc_codegen_llvm"),
Diff in /checkout/src/bootstrap/doc.rs at line 614:
         // Find dependencies for top level crates.
         // Find dependencies for top level crates.
         let compiler_crates = root_crates.iter().flat_map(|krate| {
-            builder
-                .in_tree_crates(krate, Some(target))
-                .into_iter()
-                .map(|krate| krate.name)
+            builder.in_tree_crates(krate, Some(target)).into_iter().map(|krate| krate.name)
 
         let mut to_open = None;
Diff in /checkout/src/bootstrap/compile.rs at line 528:
 
 
     fn make_run(run: RunConfig<'_>) {
         let krate = run.builder.crate_paths[&run.path];
-        run.builder.ensure(Crate {
-            target: run.target,
-            mode: Mode::Rustc,
-            krate,
-        });
+        run.builder.ensure(Crate { target: run.target, mode: Mode::Rustc, krate });
 
 
     fn run(self, _: &Builder<'_>) {
Diff in /checkout/src/bootstrap/compile.rs at line 592:
         builder.info(&format!(
         builder.info(&format!(
             "{} {} stage{} ({} -> {})",
-            builder.kind.as_str(), krate, compiler.stage, &compiler.host, target
+            builder.kind.as_str(),
+            krate,
+            compiler.stage,
+            &compiler.host,
         ));
 
-        run_cargo(
-            builder,
-            builder,
-            cargo,
-            vec![],
-            &stamp,
-            vec![],
-            false,
-        );
+        run_cargo(builder, cargo, vec![], &stamp, vec![], false);
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs" "/checkout/src/bootstrap/setup.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
