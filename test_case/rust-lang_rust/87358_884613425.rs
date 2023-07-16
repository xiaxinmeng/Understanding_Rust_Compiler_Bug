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
Diff in /checkout/src/bootstrap/config.rs at line 796:
                     // just assume dynamic for now
                 } else {
                 } else {
-                    let link_type = t!(std::fs::read_to_string(ci_llvm.join("link-type.txt")), format!("CI llvm missing: {}", ci_llvm.display()));
+                    let link_type = t!(
+                        std::fs::read_to_string(ci_llvm.join("link-type.txt")),
+                        format!("CI llvm missing: {}", ci_llvm.display())
                     link_type == "dynamic"
                 };
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lexer/src/tests.rs" "/checkout/src/build_helper/lib.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/format.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
