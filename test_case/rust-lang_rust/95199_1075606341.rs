
error: strict provenance disallows casting pointer `*const rustc_ast::Expr` to integer `usize`
   --> compiler\rustc_parse\src\parser\expr.rs:806:27
    |
806 |         let addr_before = &*cast_expr as *const _ as usize;
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D fuzzy-provenance-casts` implied by `-D warnings`
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting pointer `*const rustc_ast::Expr` to integer `usize`
   --> compiler\rustc_parse\src\parser\expr.rs:809:38
    |
809 |         let changed = addr_before != &*with_postfix as *const _ as usize;
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

   Compiling rustc_middle v0.0.0 (C:\Users\ninte\dev\rust-hell\rust\compiler\rustc_middle)
   Compiling rustc_ast_lowering v0.0.0 (C:\Users\ninte\dev\rust-hell\rust\compiler\rustc_ast_lowering)
error: could not compile `rustc_parse` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: strict provenance disallows casting pointer `*const RegionKind` to integer `usize`
  --> compiler\rustc_middle\src\ty\subst.rs:83:30
   |
83 |                 (REGION_TAG, lt.0.0 as *const ty::RegionKind as usize)
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D fuzzy-provenance-casts` implied by `-D warnings`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting pointer `*const TyS<'tcx>` to integer `usize`
  --> compiler\rustc_middle\src\ty\subst.rs:88:28
   |
88 |                 (TYPE_TAG, ty.0.0 as *const ty::TyS<'tcx> as usize)
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting pointer `*const ConstS<'tcx>` to integer `usize`
  --> compiler\rustc_middle\src\ty\subst.rs:93:29
   |
93 |                 (CONST_TAG, ct.0.0 as *const ty::ConstS<'tcx> as usize)
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting integer `usize` to pointer `*const RegionKind`
   --> compiler\rustc_middle\src\ty\subst.rs:154:23
    |
154 |                     &*((ptr & !TAG_MASK) as *const ty::RegionKind),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting integer `usize` to pointer `*const TyS<'tcx>`
   --> compiler\rustc_middle\src\ty\subst.rs:157:23
    |
157 |                     &*((ptr & !TAG_MASK) as *const ty::TyS<'tcx>),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting integer `usize` to pointer `*const ConstS<'tcx>`
   --> compiler\rustc_middle\src\ty\subst.rs:160:23
    |
160 |                     &*((ptr & !TAG_MASK) as *const ty::ConstS<'tcx>),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting pointer `*const adt::AdtDefData` to integer `usize`
   --> compiler\rustc_middle\src\ty\adt.rs:144:24
    |
144 |             let addr = self as *const AdtDefData as usize;
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting pointer `*const ImplicitCtxt<'_, 'tcx>` to integer `usize`
    --> compiler\rustc_middle\src\ty\context.rs:1836:17
     |
1836 |         set_tlv(context as *const _ as usize, || f(&context))
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
     = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting integer `usize` to pointer `*const ImplicitCtxt<'_, '_>`
    --> compiler\rustc_middle\src\ty\context.rs:1853:31
     |
1853 |             unsafe { f(Some(&*(context as *const ImplicitCtxt<'_, '_>))) }
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
     = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting pointer `*const T` to integer `usize`
  --> compiler\rustc_middle\src\ty\impls_ty.rs:26:24
   |
26 |             let key = (self.as_ptr() as usize, self.len(), hcx.hashing_controls());
   |                        ^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting pointer `*const list::List<T>` to integer `usize`
   --> compiler\rustc_middle\src\ty\list.rs:205:9
    |
205 |         self as *const List<T> as usize
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting integer `usize` to pointer `*const list::List<T>`
   --> compiler\rustc_middle\src\ty\list.rs:210:11
    |
210 |         &*(ptr as *const List<T>)
    |           ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address
