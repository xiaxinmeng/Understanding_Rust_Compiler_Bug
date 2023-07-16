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
Diff in /checkout/library/alloc/tests/fmt.rs at line 68:
     t!(format!("{:?}", 10_usize), "10");
     t!(format!("{:?}", "true"), "\"true\"");
     t!(format!("{:?}", "foo\nbar"), "\"foo\\nbar\"");
-    t!(
-        format!("{:?}", "foo\n\"bar\"\r\n\'baz\'\t\\qux\\"),
-        r#""foo\n\"bar\"\r\n'baz'\t\\qux\\""#
-    );
+    t!(format!("{:?}", "foo\n\"bar\"\r\n\'baz\'\t\\qux\\"), r#""foo\n\"bar\"\r\n'baz'\t\\qux\\""#);
     t!(format!("{:?}", "foo\0bar\x01baz\u{7f}q\u{75}x"), r#""foo\u{0}bar\u{1}baz\u{7f}qux""#);
     t!(format!("{:o}", 10_usize), "12");
     t!(format!("{:x}", 10_usize), "a");
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/tests/cow_str.rs" "/checkout/library/alloc/tests/arc.rs" "/checkout/library/alloc/tests/borrow.rs" "/checkout/library/alloc/tests/lib.rs" "/checkout/library/alloc/tests/fmt.rs" "/checkout/library/alloc/src/string.rs" "/checkout/library/alloc/src/tests.rs" "/checkout/library/alloc/tests/rc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
