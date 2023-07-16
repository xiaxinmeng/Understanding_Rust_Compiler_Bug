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
Diff in /checkout/src/bootstrap/test.rs at line 911:
         let src_path = "src/test/rustdoc-gui/src";
         // We generate docs for the libraries present in the rustdoc-gui's src folder.
         let mut cargo = Command::new(&builder.initial_cargo);
-        cargo.arg("doc")
+        cargo
+            .arg("doc")
             .arg("--workspace")
             .arg("--target-dir")
             .arg(&out_dir)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/header/tests.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/tools/rustdoc/main.rs" "/checkout/src/tools/rustc-workspace-hack/lib.rs" "/checkout/src/tools/jsondocck/src/cache.rs" "/checkout/src/tools/jsondocck/src/error.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/cc_detect.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
