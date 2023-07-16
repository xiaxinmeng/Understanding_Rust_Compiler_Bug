plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/collections/vec_deque/tests.rs at line 29:
 #[bench]
 fn bench_pop_back_100(b: &mut test::Bencher) {
     let size = 100;
-    let mut deq = VecDeque::<i32>::with_capacity(size+1);
+    let mut deq = VecDeque::<i32>::with_capacity(size + 1);
     // We'll mess with private state to pretend like `deq` is filled.
     // Make sure the buffer is initialized so that we don't read uninit memory.
-    unsafe { deq.ptr().write_bytes(0u8, size+1) };
+    unsafe { deq.ptr().write_bytes(0u8, size + 1) };
 
     b.iter(|| {
         deq.head = size;
Diff in /checkout/library/alloc/src/collections/vec_deque/tests.rs at line 72:
 
     b.iter(|| {
         let mut v = v.clone();
-        v.retain(|x| *x > size/2)
+        v.retain(|x| *x > size / 2)
 }
 
 
Diff in /checkout/library/alloc/src/collections/vec_deque/tests.rs at line 79:
 #[bench]
 fn bench_pop_front_100(b: &mut test::Bencher) {
     let size = 100;
-    let mut deq = VecDeque::<i32>::with_capacity(size+1);
+    let mut deq = VecDeque::<i32>::with_capacity(size + 1);
     // We'll mess with private state to pretend like `deq` is filled.
     // Make sure the buffer is initialized so that we don't read uninit memory.
-    unsafe { deq.ptr().write_bytes(0u8, size+1) };
+    unsafe { deq.ptr().write_bytes(0u8, size + 1) };
 
     b.iter(|| {
         deq.head = size;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/vec_deque/spec_extend.rs" "/checkout/library/alloc/src/collections/vec_deque/tests.rs" "/checkout/library/alloc/src/collections/vec_deque/drain.rs" "/checkout/library/alloc/src/collections/vec_deque/pair_slices.rs" "/checkout/library/alloc/src/collections/vec_deque/into_iter.rs" "/checkout/library/alloc/src/collections/vec_deque/iter.rs" "/checkout/library/alloc/src/collections/vec_deque/ring_slices.rs" "/checkout/library/alloc/src/rc.rs"` failed.
Build completed unsuccessfully in 0:00:10
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
