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
########################################                                  56.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/tools/cargotest/main.rs at line 119:
     if let Some(lockfile) = test.lock {
         fs::write(&dir.join("Cargo.lock"), lockfile).unwrap();
     }
-    if !run_cargo_test(cargo, &dir, test.packages, test.features, test.manifest_path, test.filters) {
+    if !run_cargo_test(cargo, &dir, test.packages, test.features, test.manifest_path, test.filters)
+    {
         panic!("tests failed for {}", test.repo);
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/header.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/tools/compiletest/src/read2.rs" "/checkout/src/tools/compiletest/src/runtest/debugger.rs" "/checkout/src/tools/compiletest/src/read2/tests.rs" "/checkout/src/tools/rustbook/src/main.rs" "/checkout/src/tools/cargotest/main.rs" "/checkout/src/tools/compiletest/src/runtest/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'thread 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
Build completed unsuccessfully in 0:00:13
