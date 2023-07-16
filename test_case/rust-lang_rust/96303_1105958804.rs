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
 
         // error_index_generator uses stage 0 to share rustdoc artifacts with the
         // rustdoc tool.
-        assert_eq!(
-            first(cache.all::<doc::ErrorIndex>()),
-            &[doc::ErrorIndex { target: a },]
-        );
+        assert_eq!(first(cache.all::<doc::ErrorIndex>()), &[doc::ErrorIndex { target: a },]);
         assert_eq!(
             first(cache.all::<tool::ErrorIndex>()),
             &[tool::ErrorIndex { compiler: Compiler { host: a, stage: 0 } }]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/formats/cache.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/library/unwind/src/lib.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
