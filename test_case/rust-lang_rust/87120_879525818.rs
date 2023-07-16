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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/doctest.rs at line 167:
 
     test_args.insert(0, "rustdoctest".to_string());
-    test::test_main(
-        &test_args,
-        tests,
-        tests,
-        Some(test::Options::new().display_output(display_warnings)),
-    );
+    test::test_main(&test_args, tests, Some(test::Options::new().display_output(display_warnings)));
 
     // Collect and warn about unused externs, but only if we've gotten
     // reports for each doctest
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/tests.rs" "/checkout/src/etc/test-float-parse/tiny-pow10.rs" "/checkout/src/librustdoc/html/escape.rs" "/checkout/src/librustdoc/doctest.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
