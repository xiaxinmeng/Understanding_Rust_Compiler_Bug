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
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 2000:
                 // Pass path.parent() as output_file to save_analysis config
                 let dir = format!(
                     "{{ \"output_file\":\"{}\", \"full_docs\":false, \"pub_only\":false, \"reachable_only\":false, \"distro_crate\": false,\"signatures\":false, \"borrow_data\":false }}",
-                    path.parent().unwrap().join("save-analysis").join("const-argument-non-static-lifetime.json").display());
+                    path.parent()
+                        .unwrap()
+                        .join("save-analysis")
+                        .join("const-argument-non-static-lifetime.json")
+                        .display()
                 println!("dir: {}", dir);
                 println!("dir: {}", dir);
                 rustc.env("RUST_SAVE_ANALYSIS_CONFIG", dir);
                 //rustc.arg("--out-dir").arg(&self.config.build_base);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/main.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/compiletest/src/read2.rs" "/checkout/src/tools/compiletest/src/errors.rs" "/checkout/src/tools/compiletest/src/tests.rs" "/checkout/src/tools/compiletest/src/json.rs" "/checkout/src/tools/compiletest/src/util.rs" "/checkout/src/tools/compiletest/src/header/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
