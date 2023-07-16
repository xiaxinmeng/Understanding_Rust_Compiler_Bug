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
Diff in /checkout/src/bootstrap/native.rs at line 126:
             .arg(llvm_project)
             .current_dir(&build.config.src),
-    let hash =
-    let hash =
-        recorded.split_whitespace().nth(2).unwrap_or_else(|| panic!("unexpected output `{}`", recorded));
+    let hash = recorded
+        .split_whitespace()
+        .nth(2)
+        .unwrap_or_else(|| panic!("unexpected output `{}`", recorded));
     // update_submodule
     // update_submodule
     if let Some(llvm_hash) = checked_out {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/run.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/library/term/src/terminfo/searcher.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/src/bootstrap/flags.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
