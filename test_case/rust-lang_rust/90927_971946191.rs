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
  |
2 |     a.5.2E+
  |       ^^^^^

thread 'main' panicked at 'unexpected components in a float token: [IdentLike("5"), Punct('.'), IdentLike("2E"), Punct('+')]', compiler/rustc_parse/src/parser/expr.rs:1041:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/etc/test-float-parse/src/bin/short-decimals.rs" "/checkout/src/etc/test-float-parse/src/bin/subnorm.rs" "/checkout/src/etc/test-float-parse/src/bin/long-fractions.rs" "/checkout/src/etc/test-float-parse/src/bin/u64-pow2.rs" "/checkout/src/etc/test-float-parse/src/bin/huge-pow10.rs" "/checkout/src/etc/test-float-parse/src/bin/tiny-pow10.rs" "/checkout/src/etc/test-float-parse/src/bin/issue-90728.rs" "/checkout/src/etc/test-float-parse/src/bin/u32-small.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
