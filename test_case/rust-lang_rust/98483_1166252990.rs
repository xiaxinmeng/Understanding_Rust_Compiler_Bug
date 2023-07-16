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
Diff in /checkout/src/bootstrap/dist.rs at line 2018:
             "llvm-config-wrapper",
             "rustc",
             "rustdoc",
-            "sccache-plus-cl"
+            "sccache-plus-cl",
         ] {
             tarball.add_file(bootstrap_outdir.join(file), "bin", 0o755);
Diff in /checkout/src/bootstrap/dist.rs at line 2025:
 
 
         tarball.set_overlay(OverlayKind::LLVM);
 
 
         /* run only if llvm-config isn't used */
         if let Some(config) = builder.config.target_config.get(&target) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/cache.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/dist.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
