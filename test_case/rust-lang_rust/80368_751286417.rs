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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 3413:
                 match field.kind {
                     clean::StrippedItem(box clean::StructFieldItem(..)) => write!(w, "_"),
                     clean::StructFieldItem(ref ty) => {
-                        write!(w, "{}{}", field.visibility.print_with_space(cx.tcx(), field.def_id.expect_local()), ty.print())
+                            w,
+                            w,
+                            "{}{}",
+                            field
+                                .visibility
+                                .print_with_space(cx.tcx(), field.def_id.expect_local()),
+                            ty.print()
                     }
                     }
                     _ => unreachable!(),
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:21
