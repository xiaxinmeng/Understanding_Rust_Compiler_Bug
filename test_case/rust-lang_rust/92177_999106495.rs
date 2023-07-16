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
Diff in /checkout/src/librustdoc/html/sources.rs at line 288:
         }
         SourceContext::Embedded { offset } => {
             for line in 1..=lines {
-               writeln!(line_numbers, "<span>{0:1$}</span>", line + offset, cols)
+                writeln!(line_numbers, "<span>{0:1$}</span>", line + offset, cols)
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/url_parts_builder/tests.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/html/escape.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
