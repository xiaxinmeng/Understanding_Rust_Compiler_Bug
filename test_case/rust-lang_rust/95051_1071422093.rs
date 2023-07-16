plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2018 --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg feature=\"external_crate\" --cfg no_global_oom_handling
--- stderr -------------------------------
--- stderr -------------------------------
error[E0432]: unresolved imports `super::AsIntoIter`, `super::InPlaceDrop`
 --> ../../../../library/alloc/src/vec/source_iter_marker.rs:7:13
  |
7 | use super::{AsIntoIter, InPlaceDrop, SpecFromIter, SpecFromIterNested, Vec};
  |             ^^^^^^^^^^  ^^^^^^^^^^^ no `InPlaceDrop` in `vec`
  |             |
  |             no `AsIntoIter` in `vec`

error[E0432]: unresolved import `super::SetLenOnDrop`
 --> ../../../../library/alloc/src/vec/spec_extend.rs:6:35
  |
  |
6 | use super::{IntoIter, Operations, SetLenOnDrop, Vec};
  |                                   ^^^^^^^^^^^^ no `SetLenOnDrop` in `vec`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:137:41
    |
    |
137 |         Self::try_allocate_in(capacity, AllocInit::Uninitialized, alloc)
    |                                         ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:150:41
    |
    |
150 |         Self::try_allocate_in(capacity, AllocInit::Zeroed, alloc)
    |                                         ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:203:17
    |
    |
203 |                 AllocInit::Uninitialized => alloc.allocate(layout),
    |                 ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:204:17
    |
    |
204 |                 AllocInit::Zeroed => alloc.allocate_zeroed(layout),
    |                 ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared crate or module `cmp`
    --> ../../../../library/alloc/src/vec/mod.rs:1206:40
     |
     |
1206 |             self.buf.try_shrink_to_fit(cmp::max(self.len, min_capacity))?;
     |                                        ^^^ use of undeclared crate or module `cmp`
error[E0433]: failed to resolve: use of undeclared type `SetLenOnDrop`
    --> ../../../../library/alloc/src/vec/mod.rs:2916:33
     |
     |
2916 |             let mut local_len = SetLenOnDrop::new(&mut self.len);
     |                                 ^^^^^^^^^^^^ use of undeclared type `SetLenOnDrop`
error[E0433]: failed to resolve: use of undeclared type `SpecFromElem`
    --> ../../../../library/alloc/src/vec/mod.rs:2975:11
     |
     |
2975 |     <T as SpecFromElem>::from_elem::<Global, FallibleOperations>(elem, n, Global)
     |           ^^^^^^^^^^^^ use of undeclared type `SpecFromElem`
error[E0433]: failed to resolve: use of undeclared type `SpecFromElem`
    --> ../../../../library/alloc/src/vec/mod.rs:2992:11
     |
     |
2992 |     <T as SpecFromElem>::from_elem::<A, FallibleOperations>(elem, n, alloc)
     |           ^^^^^^^^^^^^ use of undeclared type `SpecFromElem`
error[E0412]: cannot find type `AllocInit` in this scope
   --> ../../../../library/alloc/src/raw_vec.rs:192:15
    |
192 |         init: AllocInit,
192 |         init: AllocInit,
    |               ^^^^^^^^^ not found in this scope

warning: unused import: `from_fn`
  --> ../../../../library/alloc/src/string.rs:51:18
   |
51 | use core::iter::{from_fn, FusedIterator};
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `Infallible`
warning: unused import: `Infallible`
  --> ../../../../library/alloc/src/vec/mod.rs:59:21
   |
59 | use core::convert::{Infallible, TryFrom};

error[E0282]: type annotations needed
   --> ../../../../library/alloc/src/vec/source_iter_marker.rs:103:11
    |
    |
103 |     move |mut sink, item| {
    |           ^^^^^^^^ consider giving this closure parameter a type
    = note: type must be known at this point

error: aborting due to 12 previous errors; 2 warnings emitted


Some errors have detailed explanations: E0282, E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0282`.
make: *** [Makefile:4: all] Error 1



failures:
