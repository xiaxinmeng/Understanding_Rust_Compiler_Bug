plain
Successfully built 59c936844dd9
Successfully tagged rust-ci:latest
Built container sha256:59c936844dd9fbcb0f11805714662ac1e3cfeb75f2ab529f35ab17c69b1ed5dd
Uploading finished image to https://ci-caches.rust-lang.org/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3
upload failed: - to s3://rust-lang-ci-sccache2/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3 Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=mingw-check]
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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/config.rs at line 459:
                 })
                 .collect(),
         ];
-        let default_settings = default_settings.into_iter().flatten()
+        let default_settings = default_settings
+            .into_iter()
+            .flatten()
             .map(
                 // The keys here become part of `data-` attribute names in the generated HTML.  The
                 // browser does a strange mapping when converting them into attributes on the
Diff in /checkout/src/librustdoc/config.rs at line 479:
                 // `getSettingValue` in `storage.js.`) Converting `-` to `_` is simple in JS.
                 //
                 // The values will be HTML-escaped by the default Tera escaping.
-                |(k, v)| (k.replace('-', "_"), v)
+                |(k, v)| (k.replace('-', "_"), v),
             .collect();
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/etc/test-float-parse/src/bin/few-ones.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/visit_ast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
