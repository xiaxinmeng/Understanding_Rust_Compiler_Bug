
error: internal compiler error: broken MIR in DefId(0:18 ~ playground[af62]::main[0]) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:16 ~ playground[af62]::test[0]), UserSubsts { substs: [Const { ty: &'static (dyn A + 'static), val: Unevaluated(DefId(0:19 ~ playground[af62]::main[0]::{{constant}}[0]), []) }], user_self_ty: None }) }, span: src/main.rs:12:5: 12:19, inferred_ty: fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>} }): bad user type AscribeUserType(fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>}, DefId(0:16 ~ playground[af62]::test[0]) UserSubsts { substs: [Const { ty: &'static (dyn A + 'static), val: Unevaluated(DefId(0:19 ~ playground[af62]::main[0]::{{constant}}[0]), []) }], user_self_ty: None }): NoSolution

error: internal compiler error: broken MIR in DefId(0:18 ~ playground[af62]::main[0]) (const test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>): constant const test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A> should have type fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>} but has fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>} (NoSolution)
  --> src/main.rs:12:5
   |
12 |     test::<{ &B }>();
   |     ^^^^^^^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:18 ~ playground[af62]::main[0]) (const test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>): bad constant user type CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:16 ~ playground[af62]::test[0]), UserSubsts { substs: [Const { ty: &'static (dyn A + 'static), val: Unevaluated(DefId(0:19 ~ playground[af62]::main[0]::{{constant}}[0]), []) }], user_self_ty: None }) }, span: src/main.rs:12:5: 12:19, inferred_ty: fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>} } vs fn() {test::<ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), AllocId(1))), (Size { raw: 8 }, ((), AllocId(2)))] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Immutable, extra: () }, offset: Size { raw: 0 } } : &dyn A>}: NoSolution
  --> src/main.rs:12:5
   |
12 |     test::<{ &B }>();
   |     ^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:361:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: <rustc_errors::Handler as core::ops::drop::Drop>::drop
  10: core::ptr::real_drop_in_place
  11: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  12: core::ptr::real_drop_in_place
  13: rustc_interface::interface::run_compiler_in_existing_thread_pool
  14: std::thread::local::LocalKey<T>::with
  15: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic
