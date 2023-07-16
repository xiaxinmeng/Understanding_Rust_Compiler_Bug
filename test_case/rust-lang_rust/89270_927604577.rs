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
Diff in /checkout/library/std/src/path.rs at line 1253:
                         Component::CurDir => (),
                         Component::ParentDir => match buf.last() {
                             Some(Component::RootDir) => (),
-                            Some(Component::Prefix(_) | Component::ParentDir) | None => {
-                                buf.push(c)
-                            }
+                            Some(Component::Prefix(_) | Component::ParentDir) | None => buf.push(c),
                             _ => {
                                 let _ = buf.pop();
                             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/path/tests.rs" "/checkout/library/std/src/thread/tests.rs" "/checkout/library/std/src/io/cursor.rs" "/checkout/library/std/src/path.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/library/std/src/macros.rs" "/checkout/library/std/src/io/copy.rs" "/checkout/src/etc/test-float-parse/src/bin/short-decimals.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
