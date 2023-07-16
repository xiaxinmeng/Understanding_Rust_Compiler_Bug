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
Diff in /checkout/library/core/tests/array.rs at line 442:
     let mut v = [1, 2, 3, 4, 5, 6];
     {
     {
-       let (left, right) = v.split_array_mut::<0>();
-       assert_eq!(left, &mut []);
-       assert_eq!(right, &mut [1, 2, 3, 4, 5, 6]);
+        let (left, right) = v.split_array_mut::<0>();
+        assert_eq!(left, &mut []);
+        assert_eq!(right, &mut [1, 2, 3, 4, 5, 6]);
 
     {
Diff in /checkout/library/core/tests/slice.rs at line 2197:
Diff in /checkout/library/core/tests/slice.rs at line 2197:
     let v = &mut [1, 2, 3, 4, 5, 6][..];
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/cell.rs" "/checkout/library/core/tests/fmt/builders.rs" "/checkout/library/core/tests/hash/mod.rs" "/checkout/library/core/tests/array.rs" "/checkout/library/core/tests/hash/sip.rs" "/checkout/library/core/tests/const_ptr.rs" "/checkout/library/core/tests/ascii.rs" "/checkout/library/core/tests/slice.rs"` failed.
     {
     {
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-       let (left, right) = v.split_array_mut::<0>();
-       assert_eq!(left, &mut []);
-       assert_eq!(right, [1, 2, 3, 4, 5, 6]);
+        let (left, right) = v.split_array_mut::<0>();
+        assert_eq!(left, &mut []);
+        assert_eq!(right, [1, 2, 3, 4, 5, 6]);
 
     {
Build completed unsuccessfully in 0:00:13
