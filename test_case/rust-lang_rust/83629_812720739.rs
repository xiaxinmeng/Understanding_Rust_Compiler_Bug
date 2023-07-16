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
Diff in /checkout/library/alloc/tests/vec.rs at line 1:
+use alloc::boxed::Box;
 use std::borrow::Cow;
 use std::cell::Cell;
 use std::collections::TryReserveError::*;
Diff in /checkout/library/alloc/tests/vec.rs at line 9:
 use std::rc::Rc;
 use std::sync::atomic::{AtomicU32, Ordering};
 use std::vec::{Drain, IntoIter};
-use alloc::boxed::Box;
 
 struct DropCounter<'a> {
     count: &'a mut u32,
Diff in /checkout/library/alloc/tests/vec.rs at line 1091:
 
     assert_eq!(unsafe { DROP_COUNTER }, 1);
     // clean up the leak to keep miri happy
-    unsafe { Vec::from_raw_parts(to_free, 0, cap); }
+    unsafe {
+        Vec::from_raw_parts(to_free, 0, cap);
 }
 
 #[test]
 #[test]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/slice/iter/macros.rs" "/checkout/library/core/benches/lib.rs" "/checkout/library/alloc/tests/vec.rs" "/checkout/library/alloc/tests/lib.rs" "/checkout/library/alloc/tests/string.rs" "/checkout/library/core/benches/num/mod.rs" "/checkout/library/alloc/tests/binary_heap.rs" "/checkout/library/alloc/benches/btree/set.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
