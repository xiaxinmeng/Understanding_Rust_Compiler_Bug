plain
    Finished release [optimized] target(s) in 9.38s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
..........iF....ii...................................................................... 88/229
...............................i...ii...........F....
failures:

---- [run-make] src/test/run-make-fulldeps/alloc-no-oom-handling stdout ----
---- [run-make] src/test/run-make-fulldeps/alloc-no-oom-handling stdout ----

error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2021 -Dwarnings --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg no_global_oom_handling
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
    --> ../../../../library/alloc/src/boxed.rs:2327:14
     |
     |
2327 | impl<'a, 'b> From<Cow<'b, str>> for Box<dyn Error + Send + Sync + 'a> {
     |              ^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
note: required because of the requirements on the impl of `ToOwned` for `str`
    --> ../../../../library/alloc/src/borrow.rs:83:9
     |
     |
83   | impl<T> ToOwned for T
note: required because it appears within the type `Cow<'b, str>`
    --> ../../../../library/alloc/src/borrow.rs:180:10
     |
     |
180  | pub enum Cow<'a, B: ?Sized + 'a>
note: required by a bound in `From`
    --> /checkout/library/core/src/convert/mod.rs:371:16
     |
     |
371  | pub trait From<T>: Sized {
     |                ^ required by this bound in `From`
error[E0277]: the trait bound `str: Clone` is not satisfied
    --> ../../../../library/alloc/src/boxed.rs:2349:10
     |
     |
2349 | impl<'a> From<Cow<'a, str>> for Box<dyn Error> {
     |          ^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
note: required because of the requirements on the impl of `ToOwned` for `str`
    --> ../../../../library/alloc/src/borrow.rs:83:9
     |
     |
83   | impl<T> ToOwned for T
note: required because it appears within the type `Cow<'a, str>`
    --> ../../../../library/alloc/src/borrow.rs:180:10
     |
     |
180  | pub enum Cow<'a, B: ?Sized + 'a>
note: required by a bound in `From`
    --> /checkout/library/core/src/convert/mod.rs:371:16
     |
     |
371  | pub trait From<T>: Sized {
     |                ^ required by this bound in `From`
error[E0277]: the trait bound `str: Clone` is not satisfied
    --> ../../../../library/alloc/src/boxed.rs:2342:18
     |
     |
2342 |     fn from(err: Cow<'b, str>) -> Box<dyn Error + Send + Sync + 'a> {
     |                  ^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
note: required because of the requirements on the impl of `ToOwned` for `str`
    --> ../../../../library/alloc/src/borrow.rs:83:9
     |
     |
83   | impl<T> ToOwned for T
note: required by a bound in `Cow`
    --> ../../../../library/alloc/src/borrow.rs:182:8
     |
     |
180  | pub enum Cow<'a, B: ?Sized + 'a>
     |          --- required by a bound in this
181  | where
182  |     B: ToOwned,
     |        ^^^^^^^ required by this bound in `Cow`
error[E0277]: the trait bound `str: Clone` is not satisfied
    --> ../../../../library/alloc/src/boxed.rs:2363:18
     |
     |
2363 |     fn from(err: Cow<'a, str>) -> Box<dyn Error> {
     |                  ^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
note: required because of the requirements on the impl of `ToOwned` for `str`
    --> ../../../../library/alloc/src/borrow.rs:83:9
     |
     |
83   | impl<T> ToOwned for T
note: required by a bound in `Cow`
    --> ../../../../library/alloc/src/borrow.rs:182:8
     |
     |
180  | pub enum Cow<'a, B: ?Sized + 'a>
     |          --- required by a bound in this
181  | where
182  |     B: ToOwned,
     |        ^^^^^^^ required by this bound in `Cow`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
make: *** [Makefile:4: all] Error 1


---- [run-make] src/test/run-make-fulldeps/core-no-fp-fmt-parse stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse  --edition=2021 --crate-type=rlib ../../../../library/core/src/lib.rs --cfg no_fp_fmt_parse
--- stderr -------------------------------
error[E0773]: attempted to define built-in macro more than once
    --> /checkout/library/core/src/macros/mod.rs:1319:5
     |
     |
1319 |     macro_rules! cfg {
     |     ^^^^^^^^^^^^^^^^
     |
note: previously defined here
    --> ../../../../library/core/src/macros/mod.rs:1319:5
     |
1319 | /     macro_rules! cfg {
1320 | |         ($($cfg:tt)*) => {
1322 | |         };
1323 | |     }
     | |_____^

---

error[E0152]: found duplicate lang item `const_eval_select`
    --> ../../../../library/core/src/intrinsics.rs:2687:1
     |
2687 | / pub const unsafe fn const_eval_select<ARG, F, G, RET>(
2688 | |     arg: ARG,
2689 | |     _called_in_const: F,
2690 | |     called_at_rt: G,
...    |
2693 | |     F: ~const FnOnce<ARG, Output = RET>,
2694 | |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `const_eval_select_ct`
    --> ../../../../library/core/src/intrinsics.rs:2706:1
     |
2706 | / pub const unsafe fn const_eval_select_ct<ARG, F, G, RET>(
2707 | |     arg: ARG,
2708 | |     called_in_const: F,
2709 | |     _called_at_rt: G,
...    |
2712 | |     F: ~const FnOnce<ARG, Output = RET>,
2713 | |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `manually_drop`
  --> ../../../../library/core/src/mem/manually_drop.rs:50:1
   |
50 | pub struct ManuallyDrop<T: ?Sized> {
   |
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `maybe_uninit`
   --> ../../../../library/core/src/mem/maybe_uninit.rs:256:1
256 | pub union MaybeUninit<T> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `transmute_trait`
  --> ../../../../library/core/src/mem/transmutability.rs:12:1
   |
12 | / pub unsafe trait BikeshedIntrinsicFrom<
14 | |     Context,
15 | |     const ASSUME_ALIGNMENT: bool,
...  |
18 | |     const ASSUME_VISIBILITY: bool,
18 | |     const ASSUME_VISIBILITY: bool,
19 | | > where
   |
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `pointee_trait`
  --> ../../../../library/core/src/ptr/metadata.rs:53:1
53 | pub trait Pointee {
   | ^^^^^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `dyn_metadata`
   --> ../../../../library/core/src/ptr/metadata.rs:178:1
    |
178 | pub struct DynMetadata<Dyn: ?Sized> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `drop_in_place`
   --> ../../../../library/core/src/ptr/mod.rs:487:1
    |
487 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `align_offset`
    --> ../../../../library/core/src/ptr/mod.rs:1562:1
     |
1562 | pub(crate) unsafe fn align_offset<T: Sized>(p: *const T, a: usize) -> usize {
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `clone`
   --> ../../../../library/core/src/clone.rs:110:1
110 | pub trait Clone: Sized {
    | ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `eq`
   --> ../../../../library/core/src/cmp.rs:221:1
    |
221 | pub trait PartialEq<Rhs: ?Sized = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `partial_ord`
    --> ../../../../library/core/src/cmp.rs:1080:1
     |
1080 | pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sized`
  --> ../../../../library/core/src/marker.rs:92:1
92 | pub trait Sized {
   | ^^^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `unsize`
   --> ../../../../library/core/src/marker.rs:123:1
    |
123 | pub trait Unsize<T: ?Sized> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `structural_peq`
   --> ../../../../library/core/src/marker.rs:150:1
150 | pub trait StructuralPartialEq {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `structural_teq`
   --> ../../../../library/core/src/marker.rs:203:1
203 | pub trait StructuralEq {
    | ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)
error[E0152]: found duplicate lang item `copy`
   --> ../../../../library/core/src/marker.rs:382:1
    |
382 | pub trait Copy: Clone {
382 | pub trait Copy: Clone {
    | ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sync`
   --> ../../../../library/core/src/marker.rs:466:1
466 | pub unsafe auto trait Sync {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `phantom_data`
   --> ../../../../library/core/src/marker.rs:678:1
    |
678 | pub struct PhantomData<T: ?Sized>;
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `discriminant_kind`
   --> ../../../../library/core/src/marker.rs:702:1
702 | pub trait DiscriminantKind {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `freeze`
   --> ../../../../library/core/src/marker.rs:714:1
714 | pub(crate) unsafe auto trait Freeze {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `unpin`
   --> ../../../../library/core/src/marker.rs:770:1
    |
770 | pub auto trait Unpin {}
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `destruct`
   --> ../../../../library/core/src/marker.rs:801:1
801 | pub trait Destruct {}
    | ^^^^^^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `add`
   --> ../../../../library/core/src/ops/arith.rs:100:1
    |
100 | pub trait Add<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sub`
   --> ../../../../library/core/src/ops/arith.rs:207:1
    |
207 | pub trait Sub<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `mul`
   --> ../../../../library/core/src/ops/arith.rs:336:1
    |
336 | pub trait Mul<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `div`
   --> ../../../../library/core/src/ops/arith.rs:469:1
    |
469 | pub trait Div<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `rem`
   --> ../../../../library/core/src/ops/arith.rs:571:1
    |
571 | pub trait Rem<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `neg`
   --> ../../../../library/core/src/ops/arith.rs:685:1
685 | pub trait Neg {
    | ^^^^^^^^^^^^^
    |
    = note: the lang item is first defined in crate `core`.
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `add_assign`
   --> ../../../../library/core/src/ops/arith.rs:758:1
    |
758 | pub trait AddAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `sub_assign`
   --> ../../../../library/core/src/ops/arith.rs:825:1
    |
825 | pub trait SubAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `mul_assign`
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   --> ../../../../library/core/src/ops/arith.rs:883:1
    |
883 | pub trait MulAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `div_assign`
   --> ../../../../library/core/src/ops/arith.rs:941:1
    |
941 | pub trait DivAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `rem_assign`
    --> ../../../../library/core/src/ops/arith.rs:1002:1
     |
1002 | pub trait RemAssign<Rhs = Self> {
     |
     = note: the lang item is first defined in crate `core`.
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
     = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `not`
  --> ../../../../library/core/src/ops/bit.rs:34:1
34 | pub trait Not {
   | ^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitand`
   --> ../../../../library/core/src/ops/bit.rs:146:1
    |
146 | pub trait BitAnd<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitor`
   --> ../../../../library/core/src/ops/bit.rs:247:1
    |
247 | pub trait BitOr<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitxor`
   --> ../../../../library/core/src/ops/bit.rs:348:1
    |
348 | pub trait BitXor<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shl`
   --> ../../../../library/core/src/ops/bit.rs:448:1
    |
448 | pub trait Shl<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shr`
   --> ../../../../library/core/src/ops/bit.rs:567:1
    |
567 | pub trait Shr<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitand_assign`
   --> ../../../../library/core/src/ops/bit.rs:695:1
    |
695 | pub trait BitAndAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitor_assign`
   --> ../../../../library/core/src/ops/bit.rs:767:1
    |
767 | pub trait BitOrAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `bitxor_assign`
   --> ../../../../library/core/src/ops/bit.rs:839:1
    |
839 | pub trait BitXorAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shl_assign`
   --> ../../../../library/core/src/ops/bit.rs:909:1
    |
909 | pub trait ShlAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `shr_assign`
   --> ../../../../library/core/src/ops/bit.rs:992:1
    |
992 | pub trait ShrAssign<Rhs = Self> {
    |
    = note: the lang item is first defined in crate `core`.
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
    = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `Continue`
  --> ../../../../library/core/src/ops/control_flow.rs:87:5
87 |     Continue(C),
   |     ^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `Break`
  --> ../../../../library/core/src/ops/control_flow.rs:91:5
91 |     Break(B),
   |     ^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

error[E0152]: found duplicate lang item `deref`
  --> ../../../../library/core/src/ops/deref.rs:64:1
64 | pub trait Deref {
   | ^^^^^^^^^^^^^^^
   |
   = note: the lang item is first defined in crate `core`.
   = note: the lang item is first defined in crate `core`.
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`lib`)

