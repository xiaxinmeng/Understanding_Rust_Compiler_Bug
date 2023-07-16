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
Diff in /checkout/library/core/tests/lib.rs at line 6:
 #![feature(bool_to_option)]
 #![feature(box_syntax)]
 #![feature(cell_update)]
-#![cfg_attr(bootstrap,feature(cfg_panic))]
+#![cfg_attr(bootstrap, feature(cfg_panic))]
 #![cfg_attr(bootstrap, feature(cfg_target_has_atomic))]
 #![feature(const_assume)]
 #![feature(const_black_box)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/nonzero.rs" "/checkout/library/core/tests/option.rs" "/checkout/library/core/tests/slice.rs" "/checkout/library/core/tests/lib.rs" "/checkout/library/core/tests/cmp.rs" "/checkout/library/core/tests/num/int_macros.rs" "/checkout/library/core/tests/mem.rs" "/checkout/library/core/tests/num/i8.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
