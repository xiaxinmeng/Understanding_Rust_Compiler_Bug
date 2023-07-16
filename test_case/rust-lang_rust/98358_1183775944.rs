plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
################################################################          89.0%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/linker.rs at line 1068:
             // -fno-pie is the default on Emscripten.
             LinkOutputKind::StaticNoPicExe => {}
             LinkOutputKind::StaticPicExe => {}
-            LinkOutputKind::WasiReactorExe => unreachable!()
+            LinkOutputKind::WasiReactorExe => unreachable!(),
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_llvm/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
