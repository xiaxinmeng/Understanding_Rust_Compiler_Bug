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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/check.rs at line 50:
             .clippy_lint_allow
             .iter()
             .for_each(|v| clippy_lint_levels.push(format!("-A{}", v)));
-        builder.config.clippy_lint_deny.iter().for_each(|v| clippy_lint_levels.push(format!("-D{}", v)));
-        builder.config.clippy_lint_warn.iter().for_each(|v| clippy_lint_levels.push(format!("-W{}", v)));
+            .config
+            .clippy_lint_deny
+            .iter()
+            .iter()
+            .for_each(|v| clippy_lint_levels.push(format!("-D{}", v)));
+            .config
+            .clippy_lint_warn
+            .iter()
+            .iter()
+            .for_each(|v| clippy_lint_levels.push(format!("-W{}", v)));
             .config
             .clippy_lint_forbid
             .clippy_lint_forbid
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/library/test/src/term/terminfo/parm.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
