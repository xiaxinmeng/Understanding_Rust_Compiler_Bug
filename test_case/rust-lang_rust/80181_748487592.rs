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
Diff in /checkout/library/std/src/lib.rs at line 187:
 //! [rust-discord]: https://discord.gg/rust-lang
 #![cfg_attr(not(bootstrap), doc = "[array]: prim@array")]
 #![cfg_attr(not(bootstrap), doc = "[slice]: prim@slice")]
-
 #![cfg_attr(not(feature = "restricted-std"), stable(feature = "rust1", since = "1.0.0"))]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 #![cfg_attr(feature = "restricted-std", unstable(feature = "restricted_std", issue = "none"))]
 #![doc(
Build completed unsuccessfully in 0:00:27
