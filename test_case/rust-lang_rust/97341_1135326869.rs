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
Diff in /checkout/src/bootstrap/doc.rs at line 434:
         if builder.no_std(target) == Some(true) {
             panic!(
                 "building std documentation for no_std target {} is not supported\n\
-                 Set `docs = false` in the config to disable documentation.", target
+                 Set `docs = false` in the config to disable documentation.",
             );
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/build.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/bin/rustdoc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         let out = builder.doc_out(target);
