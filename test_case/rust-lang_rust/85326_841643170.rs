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
Diff in /checkout/src/bootstrap/tool.rs at line 55:
             Mode::ToolRustc => {
                 builder.ensure(compile::Std { compiler, target: compiler.host });
                 builder.ensure(compile::Rustc { compiler, target });
+            }
+            }
             Mode::ToolStd => builder.ensure(compile::Std { compiler, target }),
             Mode::ToolBootstrap => {} // uses downloaded stage0 compiler libs
             _ => panic!("unexpected Mode for tool build"),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/etc/test-float-parse/u64-pow2.rs" "/checkout/src/etc/test-float-parse/huge-pow10.rs" "/checkout/src/bootstrap/native.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
