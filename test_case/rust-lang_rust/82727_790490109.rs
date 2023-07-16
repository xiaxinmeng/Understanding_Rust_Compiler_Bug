plain
Step 5/10 : RUN npm install es-check -g
 ---> Running in 30a12b5cc855
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.3
added 95 packages from 44 contributors in 3.556s
Removing intermediate container 30a12b5cc855
 ---> d3f391509c56
---
Successfully built ac2c3a60601b
Successfully tagged rust-ci:latest
Built container sha256:ac2c3a60601ba0f63b877fc164efa3d66c9fba174acf15bb9e0b86761690dbe2
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 667:
             source_info,
             lt,
             lt,
-            Rvalue::BinaryOp(BinOp::Lt, box(Operand::Copy(Place::from(index)), Operand::Copy(len))),
+            Rvalue::BinaryOp(
+                BinOp::Lt,
+                box (Operand::Copy(Place::from(index)), Operand::Copy(len)),
         );
         );
         let msg = BoundsCheck { len: Operand::Move(len), index: Operand::Copy(Place::from(index)) };
         // assert!(lt, "...")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/expr/into.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_rvalue.rs" "/checkout/compiler/rustc_mir_build/src/build/into.rs" "/checkout/compiler/rustc_mir_build/src/build/block.rs" "/checkout/compiler/rustc_mir_build/src/build/scope.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/mod.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
