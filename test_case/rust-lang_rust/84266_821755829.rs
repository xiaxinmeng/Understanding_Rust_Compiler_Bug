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
Diff in /checkout/library/alloc/src/string.rs at line 50:
 use core::iter::FromIterator;
 use core::iter::FusedIterator;
 #[cfg(feature = "global-oom-handling")]
-use core::ops::Bound::{Excluded, Included, Unbounded};
-#[cfg(feature = "global-oom-handling")]
 use core::ops::Add;
-use core::ops::{self, Index, IndexMut, Range, RangeBounds};
 #[cfg(feature = "global-oom-handling")]
 use core::ops::AddAssign;
+#[cfg(feature = "global-oom-handling")]
+use core::ops::Bound::{Excluded, Included, Unbounded};
+use core::ops::{self, Index, IndexMut, Range, RangeBounds};
 use core::ptr;
 use core::slice;
 #[cfg(feature = "global-oom-handling")]
Diff in /checkout/library/alloc/src/rc.rs at line 258:
 #[cfg(feature = "global-oom-handling")]
 use core::iter;
 use core::marker::{self, PhantomData, Unpin, Unsize};
-use core::mem::{self, align_of_val_raw, forget};
 #[cfg(feature = "global-oom-handling")]
 use core::mem::size_of_val;
+use core::mem::{self, align_of_val_raw, forget};
 use core::ops::{CoerceUnsized, Deref, DispatchFromDyn, Receiver};
 use core::pin::Pin;
 use core::ptr::{self, NonNull};
Diff in /checkout/library/alloc/src/borrow.rs at line 4:
 use core::cmp::Ordering;
 use core::cmp::Ordering;
 use core::hash::{Hash, Hasher};
+use core::ops::Deref;
 #[cfg(feature = "global-oom-handling")]
 use core::ops::{Add, AddAssign};
 
 
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use core::borrow::{Borrow, BorrowMut};
Diff in /checkout/library/alloc/src/sync.rs at line 15:
 #[cfg(feature = "global-oom-handling")]
 use core::iter;
 use core::marker::{PhantomData, Unpin, Unsize};
-use core::mem::{self, align_of_val_raw};
 #[cfg(feature = "global-oom-handling")]
 use core::mem::size_of_val;
+use core::mem::{self, align_of_val_raw};
 use core::ops::{CoerceUnsized, Deref, DispatchFromDyn, Receiver};
 use core::pin::Pin;
 use core::ptr::{self, NonNull};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/num/flt2dec/strategy/dragon.rs" "/checkout/library/alloc/src/string.rs" "/checkout/library/core/tests/num/i128.rs" "/checkout/library/core/tests/mem.rs" "/checkout/library/alloc/src/fmt.rs" "/checkout/library/alloc/src/prelude/mod.rs" "/checkout/library/alloc/src/prelude/v1.rs" "/checkout/library/core/tests/num/flt2dec/strategy/grisu.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
