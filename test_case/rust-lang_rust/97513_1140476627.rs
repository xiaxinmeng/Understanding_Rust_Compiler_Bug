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
Diff in /checkout/src/tools/tidy/src/deps.rs at line 392:
     let metadata = t!(cmd.exec());
     let runtime_ids = HashSet::new();
     check_exceptions(&metadata, EXCEPTIONS_BOOTSTRAP, runtime_ids, bad);
-    check_dependencies(
-        &metadata,
-        PERMITTED_BOOTSTRAP_DEPENDENCIES,
-        &["bootstrap"],
-        bad,
-    );
+    check_dependencies(&metadata, PERMITTED_BOOTSTRAP_DEPENDENCIES, &["bootstrap"], bad);
 
 
 /// Check that all licenses are in the valid list in `LICENSES`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/target_specific_tests.rs" "/checkout/src/tools/tidy/src/style.rs" "/checkout/src/tools/tidy/src/deps.rs" "/checkout/src/tools/jsondocck/src/config.rs" "/checkout/src/tools/jsondocck/src/error.rs" "/checkout/src/tools/tidy/src/unit_tests.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
