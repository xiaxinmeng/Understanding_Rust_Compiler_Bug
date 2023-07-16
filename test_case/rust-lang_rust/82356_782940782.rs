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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 3521:
                     type_.print(cx.cache())
                 )));
                 debug!("Adding {} to deref id map", type_.print(cx.cache()));
-                cx.shared.deref_id_map
+                cx.shared
+                    .deref_id_map
                     .borrow_mut()
                     .insert(type_.def_id_full(cx.cache()).unwrap(), id.clone());
                 write!(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/toc.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/escape.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
