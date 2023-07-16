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
###################                                                       26.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 2012:
         let mut tarball = Tarball::new(builder, "rust-dev", &target.triple);
 
         let bootstrap_outdir = &builder.bootstrap_out;
-        for file in &[
-            "bootstrap",
-            "llvm-config-wrapper",
-            "rustdoc",
-            "sccache-plus-cl",
-        ] {
-        ] {
+        for file in &["bootstrap", "llvm-config-wrapper", "rustc", "rustdoc", "sccache-plus-cl"] {
             tarball.add_file(bootstrap_outdir.join(file), "bin", 0o755);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs" "/checkout/src/bootstrap/tarball.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
