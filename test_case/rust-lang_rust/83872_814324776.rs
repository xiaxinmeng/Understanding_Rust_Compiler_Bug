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
Diff in /checkout/src/librustdoc/doctree.rs at line 82:
                         // just keep `existing_item` and return at once
                         return;
                     }
-                    (false, false) => unreachable!() // todo: how to handle this?
+                    (false, false) => unreachable!(), // todo: how to handle this?
             }
         }
         }
Diff in /checkout/src/librustdoc/doctree.rs at line 99:
                     debug!("push_mod: {:?} shadowed by {:?}", existing_mod.name, new_mod.name);
                     *existing_mod = new_mod;
                     return;
+                }
                 (false, true) => {
                 (false, true) => {
                     // `existing_mod` is not from glob but `new_mod` is
                     // just keep `existing_mod` and return at once
Diff in /checkout/src/librustdoc/doctree.rs at line 106:
                     return;
+                }
+                }
                 (false, false) => unreachable!(),
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/theme.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
