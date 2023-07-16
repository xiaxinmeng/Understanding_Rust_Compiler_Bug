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
Diff in /checkout/library/core/benches/str.rs at line 1:
 use test::{black_box, Bencher};
-
-
 const LOREM_SHORT: &str = "Lorem ipsum";
 
 const LOREM: &str = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.
Diff in /checkout/library/core/benches/str.rs at line 14:
 #[bench]
 #[bench]
 fn str_char_count_lorem(b: &mut Bencher) {
-    b.iter(|| {
-        black_box(LOREM).chars().count()
-    });
+    b.iter(|| black_box(LOREM).chars().count());
 
 #[bench]
 #[bench]
Diff in /checkout/library/core/benches/str.rs at line 23:
 fn str_char_count_lorem_short(b: &mut Bencher) {
-    b.iter(|| {
-        black_box(LOREM_SHORT).chars().count()
-    });
+    b.iter(|| black_box(LOREM_SHORT).chars().count());
 
 #[bench]
 #[bench]
Diff in /checkout/library/core/benches/str.rs at line 30:
 fn str_char_count_emoji(b: &mut Bencher) {
-    b.iter(|| {
-        black_box(EMOJI).chars().count()
-    });
+    b.iter(|| black_box(EMOJI).chars().count());
 
Build completed unsuccessfully in 0:00:13
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/futex.rs" "/checkout/library/core/benches/ascii.rs" "/checkout/library/std/src/sys/unix/condvar.rs" "/checkout/library/core/benches/iter.rs" "/checkout/library/std/src/sys/unix/pipe.rs" "/checkout/library/core/benches/str.rs" "/checkout/library/std/src/sys/unix/os_str/tests.rs" "/checkout/library/std/src/sys/unix/os/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
