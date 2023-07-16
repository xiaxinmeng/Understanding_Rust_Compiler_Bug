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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/doctest.rs at line 720:
         }
         // We now check if there is an unclosed delimiter for the attribute. To do so, we look at
         // the `unclosed_delims` and see if the opening square bracket was closed.
-        parser.unclosed_delims().get(0).map(|unclosed| {
-            unclosed.unclosed_span.map(|s| s.lo()).unwrap_or(BytePos(0)) != BytePos(2)
-        }).unwrap_or(true)
+        parser
+            .unclosed_delims()
+            .get(0)
+            .map(|unclosed| {
+                unclosed.unclosed_span.map(|s| s.lo()).unwrap_or(BytePos(0)) != BytePos(2)
+            })
+            .unwrap_or(true)
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/layout.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/html/highlight.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/html/length_limit/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
