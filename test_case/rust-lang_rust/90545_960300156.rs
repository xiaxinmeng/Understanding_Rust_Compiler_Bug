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
Diff in /checkout/library/alloc/src/slice.rs at line 903:
             // If `is_less` panics at any point during the process, `hole` will get dropped and
             // fill the hole in `v` with `tmp`, thus ensuring that `v` still holds every object it
             // initially held exactly once.
-            let mut hole = InsertionHole {
-                src: &*tmp,
-                dest: v.get_unchecked_mut(end - 1),
-            };
+            let mut hole = InsertionHole { src: &*tmp, dest: v.get_unchecked_mut(end - 1) };
             ptr::copy_nonoverlapping(hole.dest, v.get_unchecked_mut(end), 1);
             let mut i = end - 1;
Diff in /checkout/library/alloc/src/slice.rs at line 1074:
Diff in /checkout/library/alloc/src/slice.rs at line 1074:
             // $is_less(&$v[$right], &$v[$left])
             {
                 debug_assert!($left < $v.len() && $right < $v.len());
-                $is_less(unsafe {&$v.get_unchecked($right)}, unsafe {&$v.get_unchecked($left)})
+                $is_less(unsafe { &$v.get_unchecked($right) }, unsafe { &$v.get_unchecked($left) })
         };
     }
Diff in /checkout/library/alloc/src/slice.rs at line 1103:
Diff in /checkout/library/alloc/src/slice.rs at line 1103:
     // shallow copies of the contents of `v` without risking the dtors running on copies if
     // `is_less` panics. When merging two slices, this buffer holds a copy of the right-hand slice,
     // which will always have length at most `(len + 1) / 2`.
-    let mut buf = Vec::with_capacity((len + 1)/2);
+    let mut buf = Vec::with_capacity((len + 1) / 2);
     slice_merge_sort(v, 0, buf.as_mut_ptr(), &mut is_less);
 
     // Do a recursive depth-first merge while slice's length is greater than SMALL_SLICE_LEN*2.
Diff in /checkout/library/alloc/src/slice.rs at line 1114:
         F: FnMut(&T, &T) -> bool,
     {
         let len = v.len();
-    // find length of sorted prefix
+        // find length of sorted prefix
         if sorted == 0 {
-            sorted =
-                if len <= 1 {
-                // strictly descending?
-                // strictly descending?
-                } else if gt!(v, 0, 1, is_less) {
-                    let mut i = 2;
-                    while i < len && gt!(v, i - 1, i, is_less) {
-                        i += 1;
-                    }
-                    // Reverse the slice so we don't have to sort it later.
-                    v[..i].reverse();
-                // ascending
-                } else {
-                    let mut i = 2;
-                    let mut i = 2;
-                    while i < len && ! gt!(v, i - 1, i, is_less) {
-                        i += 1;
-                    i
-                };
-                };
+            sorted = if len <= 1 {
+            // strictly descending?
+            // strictly descending?
+            } else if gt!(v, 0, 1, is_less) {
+                let mut i = 2;
+                while i < len && gt!(v, i - 1, i, is_less) {
+                    i += 1;
+                }
+                // Reverse the slice so we don't have to sort it later.
+                v[..i].reverse();
+            // ascending
+            } else {
+                let mut i = 2;
+                let mut i = 2;
+                while i < len && !gt!(v, i - 1, i, is_less) {
+                    i += 1;
+                i
+            };
         }
 
 
         // Do merge sort, using `sorted` to avoid redundant sorting.
Diff in /checkout/library/alloc/src/slice.rs at line 1146:
             } else {
                 let mid;
                 let mid;
-                if len > SMALL_SLICE_LEN*2 {
-                    mid = sorted.max(len/2);
+                if len > SMALL_SLICE_LEN * 2 {
+                    mid = sorted.max(len / 2);
                     if sorted < mid {
                         slice_merge_sort(&mut v[..mid], sorted, buf_ptr, is_less);
Diff in /checkout/library/alloc/src/slice.rs at line 1154:
Diff in /checkout/library/alloc/src/slice.rs at line 1154:
                     slice_merge_sort(&mut v[mid..], 0, buf_ptr, is_less);
-                    if ! gt!(v, mid - 1, mid, is_less) {
+                    if !gt!(v, mid - 1, mid, is_less) {
                         return;
-                    } else if gt!(v, 0, len - 1, is_less) {  // strictly reverse sorted?
+                    } else if gt!(v, 0, len - 1, is_less) {
+                        // strictly reverse sorted?
                         swap_slices(v, mid, buf_ptr);
                     }
Diff in /checkout/library/alloc/src/slice.rs at line 1165:
Diff in /checkout/library/alloc/src/slice.rs at line 1165:
                     for i in SMALL_SLICE_LEN + 1..len {
                         insert_end(&mut v[SMALL_SLICE_LEN..=i], is_less);
                     }
-                    if ! gt!(v, SMALL_SLICE_LEN - 1, SMALL_SLICE_LEN, is_less) {
+                    if !gt!(v, SMALL_SLICE_LEN - 1, SMALL_SLICE_LEN, is_less) {
                     }
                     }
                     mid = SMALL_SLICE_LEN;
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/tests/string.rs" "/checkout/library/alloc/tests/vec_deque.rs" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/src/slice.rs" "/checkout/library/rtstartup/rsbegin.rs" "/checkout/library/alloc/src/macros.rs" "/checkout/library/alloc/tests/arc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
