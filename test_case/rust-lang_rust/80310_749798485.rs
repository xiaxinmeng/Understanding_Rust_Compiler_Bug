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
Diff in /checkout/library/alloc/src/boxed.rs at line 153:
 use core::ptr::{self, Unique};
 use core::task::{Context, Poll};
 
-use crate::alloc::{handle_alloc_error, Allocator, AllocError, Global, Layout};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/boxed.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+use crate::alloc::{handle_alloc_error, AllocError, Allocator, Global, Layout};
 use crate::borrow::Cow;
 use crate::raw_vec::RawVec;
 use crate::str::from_boxed_utf8_unchecked;
Diff in /checkout/library/alloc/src/boxed.rs at line 404:
     // #[unstable(feature = "new_uninit", issue = "63291")]
     pub fn try_new_zeroed_in(alloc: A) -> Result<Box<mem::MaybeUninit<T>, A>, AllocError> {
         let layout = Layout::new::<mem::MaybeUninit<T>>();
-        let ptr =
-            alloc.allocate_zeroed(layout)?.cast();
+        let ptr = alloc.allocate_zeroed(layout)?.cast();
         unsafe { Ok(Box::from_raw_in(ptr.as_ptr(), alloc)) }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:21
