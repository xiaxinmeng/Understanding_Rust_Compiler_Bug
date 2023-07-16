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
Diff in /checkout/src/bootstrap/bin/rustc.rs at line 29:
     let target = args.windows(2).find(|w| &*w[0] == "--target").and_then(|w| w[1].to_str());
     let version = args.iter().find(|w| &**w == "-vV");
     let is_proc_macro = args.windows(2).any(|w| {
-        w[0] == "--crate-type=proc-macro" ||
-        w[0] == "--crate-type" && w[1] == "proc-macro"
+        w[0] == "--crate-type=proc-macro" || w[0] == "--crate-type" && w[1] == "proc-macro"
     });
 
     let verbose = match env::var("RUSTC_VERBOSE") {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/html/highlight.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/librustdoc/html/render/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
