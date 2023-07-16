plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0621]: explicit lifetime required in the type of `c`
    --> compiler/rustc_middle/src/mir/mod.rs:2772:23
     |
2766 |     c: &ty::Const<'tcx>,
     |        ---------------- help: add explicit lifetime `'tcx` to the type of `c`: `&'tcx ty::consts::Const<'tcx>`
...
2772 |         let literal = tcx.lift(c).unwrap();
     |                       ^^^^^^^^^^^ lifetime `'tcx` required
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:406:19
    |
    |
185 | impl<'tcx, Tag> Scalar<Tag> {
    |      ---- lifetime `'tcx` defined here
406 |         let val = self.to_u8()?;
406 |         let val = self.to_u8()?;
    |                   ^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:415:19
    |
    |
185 | impl<'tcx, Tag> Scalar<Tag> {
    |      ---- lifetime `'tcx` defined here
415 |         let val = self.to_u32()?;
415 |         let val = self.to_u32()?;
    |                   ^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:589:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
589 |         self.check_init()?.to_bool()
589 |         self.check_init()?.to_bool()
    |         ^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:594:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
...
594 |         self.check_init()?.to_char()
    |         ^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:599:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
599 |         self.check_init()?.to_f32()
599 |         self.check_init()?.to_f32()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:604:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
604 |         self.check_init()?.to_f64()
604 |         self.check_init()?.to_f64()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:609:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
609 |         self.check_init()?.to_u8()
609 |         self.check_init()?.to_u8()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:614:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
614 |         self.check_init()?.to_u16()
614 |         self.check_init()?.to_u16()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:619:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
619 |         self.check_init()?.to_u32()
619 |         self.check_init()?.to_u32()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:624:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
624 |         self.check_init()?.to_u64()
624 |         self.check_init()?.to_u64()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:629:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
629 |         self.check_init()?.to_machine_usize(cx)
629 |         self.check_init()?.to_machine_usize(cx)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:634:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
634 |         self.check_init()?.to_i8()
634 |         self.check_init()?.to_i8()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:639:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
639 |         self.check_init()?.to_i16()
639 |         self.check_init()?.to_i16()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:644:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
644 |         self.check_init()?.to_i32()
644 |         self.check_init()?.to_i32()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:649:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
649 |         self.check_init()?.to_i64()
649 |         self.check_init()?.to_i64()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/mir/interpret/value.rs:654:9
    |
    |
567 | impl<'tcx, Tag> ScalarMaybeUninit<Tag> {
    |      ---- lifetime `'tcx` defined here
654 |         self.check_init()?.to_machine_isize(cx)
654 |         self.check_init()?.to_machine_isize(cx)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/traits/mod.rs:108:9
    |
    |
103 | impl Deref for ObligationCause<'tcx> {
    |                                ---- lifetime `'tcx` defined here
...
108 |         self.data.as_deref().unwrap_or(&DUMMY_OBLIGATION_CAUSE_DATA)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/traits/mod.rs:151:54
    |
    |
127 | impl<'tcx> ObligationCause<'tcx> {
    |      ---- lifetime `'tcx` defined here
...
151 |         Rc::make_mut(self.data.get_or_insert_with(|| Rc::new(DUMMY_OBLIGATION_CAUSE_DATA)))
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'static`
    |
    = help: consider replacing `'tcx` with `'static`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/ty/codec.rs:298:9
    |
    |
296 | impl<'tcx, D: TyDecoder<I = TyInterner<'tcx>>> Decodable<D> for &ty::AdtDef {
    |      ---- lifetime `'tcx` defined here
297 |     fn decode(decoder: &mut D) -> Result<Self, D::Error> {
    |                                   ---------------------- return type is Result<&'1 adt::AdtDef, <D as rustc_serialize::Decoder>::Error>
298 |         RefDecodable::decode(decoder)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'1`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/ty/codec.rs:304:9
    |
    |
302 | impl<'tcx, D: TyDecoder<I = TyInterner<'tcx>>> Decodable<D> for &ty::Const<'_> {
    |      ---- lifetime `'tcx` defined here
303 |     fn decode(decoder: &mut D) -> Result<Self, D::Error> {
    |                                   ---------------------- return type is Result<&'1 ty::consts::Const<'_>, <D as rustc_serialize::Decoder>::Error>
304 |         RefDecodable::decode(decoder)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'tcx` must outlive `'1`
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/ty/codec.rs:304:9
    |
    |
302 | impl<'tcx, D: TyDecoder<I = TyInterner<'tcx>>> Decodable<D> for &ty::Const<'_> {
    |      ---- lifetime `'tcx` defined here
303 |     fn decode(decoder: &mut D) -> Result<Self, D::Error> {
    |                                   ---------------------- return type is Result<&ty::consts::Const<'2>, <D as rustc_serialize::Decoder>::Error>
304 |         RefDecodable::decode(decoder)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'2` must outlive `'tcx`

error[E0521]: borrowed data escapes outside of associated function
    |
111 | / macro_rules! format {
112 | |     ($($arg:tt)*) => {{
113 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
    | |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    | |                                       |
    | |                                       `self` escapes the associated function body here
114 | |         res
115 | |     }}
116 | | }
    | |_- in this expansion of `format!` (#1)
    | |_- in this expansion of `format!` (#1)
    | 
   ::: compiler/rustc_middle/src/ty/error.rs:239:24
    |
239 |       pub fn sort_string(&self, tcx: TyCtxt<'_>) -> Cow<'static, str> {
    |                          |
    |                          |
    |                          `self` declared here, outside of the associated function body
    |                          `self` is a reference that is only valid in the associated function body
...
242 |                   format!("`{}`", self).into()
    | 
   ::: /checkout/library/core/src/macros/mod.rs:829:5
    |
829 | /     macro_rules! format_args {
829 | /     macro_rules! format_args {
830 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
831 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `$crate::__export::format_args!` (#2)


error[E0621]: explicit lifetime required in the type of `other`
    --> compiler/rustc_middle/src/ty/layout.rs:1985:44
     |
1982 |     pub fn same_size(self, other: SizeSkeleton<'_>) -> bool {
     |                                   ---------------- help: add explicit lifetime `'tcx` to the type of `other`: `SizeSkeleton<'tcx>`
...
1985 |             (SizeSkeleton::Pointer { tail: a, .. }, SizeSkeleton::Pointer { tail: b, .. }) => {
     |                                            ^ lifetime `'tcx` required

error[E0621]: explicit lifetime required in the type of `other`
    --> compiler/rustc_middle/src/ty/layout.rs:1985:83
     |
1982 |     pub fn same_size(self, other: SizeSkeleton<'_>) -> bool {
     |                                   ---------------- help: add explicit lifetime `'tcx` to the type of `other`: `SizeSkeleton<'tcx>`
...
1985 |             (SizeSkeleton::Pointer { tail: a, .. }, SizeSkeleton::Pointer { tail: b, .. }) => {
     |                                                                                   ^ lifetime `'tcx` required
error: lifetime may not live long enough
  --> compiler/rustc_middle/src/ty/subst.rs:82:9
   |
   |
80 | impl<'tcx> Ord for GenericArg<'tcx> {
   |      ---- lifetime `'tcx` defined here
81 |     fn cmp(&self, other: &GenericArg<'_>) -> Ordering {
   |                                      -- let's call this `'1`
82 |         self.unpack().cmp(&other.unpack())
   |         ^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'1`

error[E0521]: borrowed data escapes outside of associated function
  --> compiler/rustc_middle/src/ty/subst.rs:82:28
   |
81 |     fn cmp(&self, other: &GenericArg<'_>) -> Ordering {
   |            -----  ----- `other` is a reference that is only valid in the associated function body
   |            |
   |            `self` declared here, outside of the associated function body
82 |         self.unpack().cmp(&other.unpack())
   |                            ^^^^^^^^^^^^^^ `other` escapes the associated function body here
error: lifetime may not live long enough
  --> compiler/rustc_middle/src/ty/subst.rs:88:14
   |
   |
86 | impl<'tcx> PartialOrd for GenericArg<'tcx> {
   |      ---- lifetime `'tcx` defined here
87 |     fn partial_cmp(&self, other: &GenericArg<'_>) -> Option<Ordering> {
   |                                              -- let's call this `'1`
88 |         Some(self.cmp(&other))
   |              ^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'1`

error[E0521]: borrowed data escapes outside of associated function
  --> compiler/rustc_middle/src/ty/subst.rs:88:14
   |
87 |     fn partial_cmp(&self, other: &GenericArg<'_>) -> Option<Ordering> {
   |                    -----  ----- `other` is a reference that is only valid in the associated function body
   |                    |
   |                    `self` declared here, outside of the associated function body
88 |         Some(self.cmp(&other))
   |              ^^^^^^^^^^^^^^^^ `other` escapes the associated function body here
error: lifetime may not live long enough
   --> compiler/rustc_middle/src/ty/subst.rs:196:9
    |
    |
190 | impl<'a, 'tcx> InternalSubsts<'tcx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
196 |         ClosureSubsts { substs: self }
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'tcx`
    |
    = help: consider adding the following bound: `'a: 'tcx`
error: lifetime may not live long enough
  --> compiler/rustc_middle/src/ty/consts.rs:32:9
   |
   |
28 | impl<S: rustc_type_ir::TyEncoder<I = TyInterner<'tcx>>> rustc_serialize::Encodable<S>
   |                                                 ---- lifetime `'tcx` defined here
...
31 |     fn encode(&self, s: &mut S) -> Result<(), S::Error> {
   |               ----- has type `&&ty::consts::Const<'1>`
32 |         (*self).encode(s)
   |         ^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'1`
error: lifetime may not live long enough
  --> compiler/rustc_middle/src/ty/consts.rs:32:9
   |
   |
28 | impl<S: rustc_type_ir::TyEncoder<I = TyInterner<'tcx>>> rustc_serialize::Encodable<S>
   |                                                 ---- lifetime `'tcx` defined here
...
31 |     fn encode(&self, s: &mut S) -> Result<(), S::Error> {
   |               ----- has type `&&ty::consts::Const<'1>`
32 |         (*self).encode(s)
   |         ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'tcx`

error[E0521]: borrowed data escapes outside of associated function
     |
2    | / macro_rules! bug {
2    | / macro_rules! bug {
3    | |     () => ( $crate::bug!("impossible case reached") );
4    | |     ($msg:expr) => ({ $crate::util::bug::bug_fmt(::std::format_args!($msg)) });
5    | |     ($msg:expr,) => ({ $crate::bug!($msg) });
7    | |         $crate::util::bug::bug_fmt(::std::format_args!($fmt, $($arg)+))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     | |                                    |
     | |                                    |
     | |                                    `self` escapes the associated function body here
8    | |     });
9    | | }
9    | | }
     | |_- in this expansion of `bug!` (#1)
    ::: compiler/rustc_middle/src/ty/sty.rs:1740:34
     |
     |
1740 |       pub fn sequence_element_type(&self, tcx: TyCtxt<'tcx>) -> Ty<'tcx> {
     |                                    |
     |                                    |
     |                                    `self` declared here, outside of the associated function body
     |                                    `self` is a reference that is only valid in the associated function body
...
1744 |               _ => bug!("`sequence_element_type` called on non-sequence value: {}", self),
     | 
    ::: /checkout/library/core/src/macros/mod.rs:829:5
     |
829  | /     macro_rules! format_args {
829  | /     macro_rules! format_args {
830  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
831  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
832  | |     }
     | |_____- in this expansion of `::std::format_args!` (#2)

error[E0521]: borrowed data escapes outside of associated function
     |
2    | / macro_rules! bug {
2    | / macro_rules! bug {
3    | |     () => ( $crate::bug!("impossible case reached") );
4    | |     ($msg:expr) => ({ $crate::util::bug::bug_fmt(::std::format_args!($msg)) });
5    | |     ($msg:expr,) => ({ $crate::bug!($msg) });
7    | |         $crate::util::bug::bug_fmt(::std::format_args!($fmt, $($arg)+))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     | |                                    |
     | |                                    |
     | |                                    `self` escapes the associated function body here
8    | |     });
9    | | }
9    | | }
     | |_- in this expansion of `bug!` (#1)
    ::: compiler/rustc_middle/src/ty/sty.rs:1812:21
     |
     |
1812 |       pub fn boxed_ty(&self) -> Ty<'tcx> {
     |                       |
     |                       |
     |                       `self` declared here, outside of the associated function body
     |                       `self` is a reference that is only valid in the associated function body
...
1815 |               _ => bug!("`boxed_ty` is called on non-box type {:?}", self),
     | 
    ::: /checkout/library/core/src/macros/mod.rs:829:5
     |
829  | /     macro_rules! format_args {
829  | /     macro_rules! format_args {
830  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
831  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
832  | |     }
     | |_____- in this expansion of `::std::format_args!` (#2)

error[E0521]: borrowed data escapes outside of associated function
     |
2    | / macro_rules! bug {
2    | / macro_rules! bug {
3    | |     () => ( $crate::bug!("impossible case reached") );
4    | |     ($msg:expr) => ({ $crate::util::bug::bug_fmt(::std::format_args!($msg)) });
5    | |     ($msg:expr,) => ({ $crate::bug!($msg) });
7    | |         $crate::util::bug::bug_fmt(::std::format_args!($fmt, $($arg)+))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     | |                                    |
     | |                                    |
     | |                                    `self` escapes the associated function body here
8    | |     });
9    | | }
9    | | }
     | |_- in this expansion of `bug!` (#1)
    ::: compiler/rustc_middle/src/ty/sty.rs:1931:19
     |
     |
1931 |       pub fn fn_sig(&self, tcx: TyCtxt<'tcx>) -> PolyFnSig<'tcx> {
     |                     |
     |                     |
     |                     `self` declared here, outside of the associated function body
     |                     `self` is a reference that is only valid in the associated function body
...
1942 |               _ => bug!("Ty::fn_sig() called on non-fn type: {:?}", self),
     | 
    ::: /checkout/library/core/src/macros/mod.rs:829:5
     |
829  | /     macro_rules! format_args {
829  | /     macro_rules! format_args {
830  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
831  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
832  | |     }
     | |_____- in this expansion of `::std::format_args!` (#2)

error[E0521]: borrowed data escapes outside of associated function
     |
2    | / macro_rules! bug {
2    | / macro_rules! bug {
3    | |     () => ( $crate::bug!("impossible case reached") );
4    | |     ($msg:expr) => ({ $crate::util::bug::bug_fmt(::std::format_args!($msg)) });
5    | |     ($msg:expr,) => ({ $crate::bug!($msg) });
7    | |         $crate::util::bug::bug_fmt(::std::format_args!($fmt, $($arg)+))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     | |                                    |
     | |                                    |
     | |                                    `self` escapes the associated function body here
8    | |     });
9    | | }
9    | | }
     | |_- in this expansion of `bug!` (#1)
    ::: compiler/rustc_middle/src/ty/sty.rs:2128:32
     |
     |
2128 |       pub fn to_opt_closure_kind(&self) -> Option<ty::ClosureKind> {
     |                                  |
     |                                  |
     |                                  `self` declared here, outside of the associated function body
     |                                  `self` is a reference that is only valid in the associated function body
...
2143 |               _ => bug!("cannot convert type `{:?}` to a closure kind", self),
     | 
