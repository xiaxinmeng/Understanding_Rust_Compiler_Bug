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
Diff in /checkout/library/std/src/os/linux/net.rs at line 3:
 //! [`std::net`]: crate::net
 
 #[unstable(feature = "tcp_quickack", issue = "96256")]
 use crate::io;
 use crate::net;
 use crate::sealed::Sealed;
 use crate::sealed::Sealed;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/linux/mod.rs" "/checkout/library/std/src/os/emscripten/fs.rs" "/checkout/library/std/src/os/linux/net.rs" "/checkout/library/std/src/os/emscripten/raw.rs" "/checkout/library/std/src/os/linux/fs.rs" "/checkout/library/std/src/os/linux/raw.rs" "/checkout/library/std/src/os/emscripten/mod.rs" "/checkout/library/std/src/primitive_docs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
