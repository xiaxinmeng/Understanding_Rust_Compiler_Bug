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
-mod nan;
 mod ieee754;
+mod nan;
 
 /// Adds the attribute to all items in the block.
 macro_rules! cfg_block {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/num/u128.rs" "/checkout/library/core/tests/num/mod.rs" "/checkout/library/core/tests/num/i16.rs" "/checkout/library/core/tests/num/int_macros.rs" "/checkout/library/core/tests/num/u16.rs" "/checkout/library/core/tests/num/i32.rs" "/checkout/library/core/tests/num/i8.rs" "/checkout/library/core/tests/task.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
