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
Diff in /checkout/src/bootstrap/native.rs at line 145:
         if progress {
             git.arg("--progress");
-        git.arg(llvm_project)
-        git.arg(llvm_project)
-           .current_dir(&builder.config.src);
+        git.arg(llvm_project).current_dir(&builder.config.src);
     };
     };
     // NOTE: doesn't use `try_run` because this shouldn't print an error if it fails.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/lib.rs" "/checkout/src/tools/unicode-table-generator/src/unicode_download.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/tools/unicode-table-generator/src/range_search.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/tools/unicode-table-generator/src/case_mapping.rs" "/checkout/src/tools/unicode-table-generator/src/raw_emitter.rs" "/checkout/src/tools/unicode-table-generator/src/skiplist.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
