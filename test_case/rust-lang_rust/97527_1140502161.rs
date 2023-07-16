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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/test.rs at line 666:
 
         cargo.env("RUSTC_TEST_SUITE", builder.rustc(compiler));
         cargo.env("RUSTC_LIB_PATH", builder.rustc_libdir(compiler));
-        let host_libs = builder.stage_out(compiler, Mode::ToolRustc).join(builder.cargo_dir(Mode::ToolRustc));
+        let host_libs =
+            builder.stage_out(compiler, Mode::ToolRustc).join(builder.cargo_dir(Mode::ToolRustc));
         cargo.env("HOST_LIBS", host_libs);
 
         cargo.arg("--").args(builder.config.cmd.test_args());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/formatters/terse.rs" "/checkout/library/test/src/formatters/junit.rs" "/checkout/library/test/src/formatters/pretty.rs" "/checkout/library/test/src/formatters/mod.rs" "/checkout/library/test/src/formatters/json.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/library/test/src/stats/tests.rs" "/checkout/src/bootstrap/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
