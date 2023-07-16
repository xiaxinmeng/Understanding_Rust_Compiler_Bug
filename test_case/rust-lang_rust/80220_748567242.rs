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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 413:
             );
             return Symbol::intern("()");
         }
-        PatKind::Range(..) => {
-            rustc_hir_pretty::pat_to_string(p)
-        },
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+        PatKind::Range(..) => rustc_hir_pretty::pat_to_string(p),
         PatKind::Slice(ref begin, ref mid, ref end) => {
             let begin = begin.iter().map(|p| name_from_pat(&**p).to_string());
             let mid = mid.as_ref().map(|p| format!("..{}", name_from_pat(&**p))).into_iter();
Build completed unsuccessfully in 0:00:19
