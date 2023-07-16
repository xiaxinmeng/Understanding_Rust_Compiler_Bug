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
Diff in /checkout/library/proc_macro/src/lib.rs at line 1240:
     #[unstable(feature = "proc_macro_tracked_path", issue = "73921")]
     use std::path::Path;
-
-
     /// Track a file as if it was a dependency.
     /// The file is located relative to the current file where the proc-macro
     /// The file is located relative to the current file where the proc-macro
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/proc_macro/src/bridge/mod.rs" "/checkout/library/proc_macro/src/bridge/buffer.rs" "/checkout/library/proc_macro/src/bridge/rpc.rs" "/checkout/library/proc_macro/src/bridge/scoped_cell.rs" "/checkout/library/proc_macro/src/bridge/client.rs" "/checkout/library/proc_macro/src/bridge/closure.rs" "/checkout/library/proc_macro/src/lib.rs" "/checkout/library/proc_macro/src/bridge/server.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
