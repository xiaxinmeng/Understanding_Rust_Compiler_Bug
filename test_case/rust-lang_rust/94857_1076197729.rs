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
Diff in /checkout/library/proc_macro/src/lib.rs at line 34:
 #![feature(min_specialization)]
 #![feature(panic_update_hook)]
 #![recursion_limit = "256"]
-
 #![no_core]
 #[allow(unused_imports)]
 #[allow(unused_imports)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/hash/sip.rs" "/checkout/library/proc_macro/src/bridge/client.rs" "/checkout/library/proc_macro/src/bridge/buffer.rs" "/checkout/library/proc_macro/src/bridge/scoped_cell.rs" "/checkout/library/proc_macro/src/diagnostic.rs" "/checkout/library/proc_macro/src/lib.rs" "/checkout/library/proc_macro/src/quote.rs" "/checkout/library/rtstartup/rsbegin.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
