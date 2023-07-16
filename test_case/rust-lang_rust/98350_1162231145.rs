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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_session/src/session.rs at line 1472:
             sess.warn(&format!(
                 "DWARF version {} is greater than the maximum supported DWARF version on \
                  this platform; using {} instead",
-                dwarf_version,
-                sess.target.max_dwarf_version));
+                dwarf_version, sess.target.max_dwarf_version
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/session.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/usefulness.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs" "/checkout/compiler/rustc_session/src/filesearch.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
