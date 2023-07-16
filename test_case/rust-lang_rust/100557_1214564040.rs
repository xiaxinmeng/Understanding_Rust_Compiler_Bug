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
Diff in /checkout/src/bootstrap/native.rs at line 124:
         let mut rev_list = config.git();
         rev_list.args(&[
             PathBuf::from("rev-list"),
-            format!("--author={}", builder.config.stage0_metadata.config.git_merge_commit_email).into(),
+            format!("--author={}", builder.config.stage0_metadata.config.git_merge_commit_email)
+                .into(),
             "-n1".into(),
             "--first-parent".into(),
             "HEAD".into(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_abort/src/android.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/native.rs" "/checkout/library/panic_unwind/src/miri.rs" "/checkout/library/proc_macro/src/diagnostic.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
