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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/tool.rs at line 599:
                     true,
                     // If `tools` is set, search list for this tool.
                     |tools| tools.iter().any(|tool| tool == "cargo"),
+                ),
         )
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/tool.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
