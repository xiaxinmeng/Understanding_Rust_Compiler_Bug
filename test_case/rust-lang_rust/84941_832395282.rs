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
Diff in /checkout/src/librustdoc/clean/types.rs at line 517:
                                 Some(ExternalLocation::Remote(ref s)) => {
                                     format!("{}/std/", s.trim_end_matches('/'))
                                 }
-                                Some(ExternalLocation::Unknown) | None => "https://doc.rust-lang.org/nightly/std/".to_string(),
+                                Some(ExternalLocation::Unknown) | None => {
+                                    "https://doc.rust-lang.org/nightly/std/".to_string()
                             };
                             };
                             // This is a primitive so the url is done "by hand".
                             let tail = fragment.find('#').unwrap_or_else(|| fragment.len());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/proc_macro/src/lib.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/library/proc_macro/src/bridge/buffer.rs" "/checkout/library/proc_macro/src/bridge/client.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/library/proc_macro/src/bridge/closure.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/library/proc_macro/src/quote.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
