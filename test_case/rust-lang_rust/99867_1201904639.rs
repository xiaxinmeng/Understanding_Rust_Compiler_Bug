plain
   Compiling hashbrown v0.12.3
   Compiling miniz_oxide v0.4.0
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:5949 ~ core[396b]::iter::adapters::cloned::clone_try_fold), local_id: 40 }, span: library/core/src/iter/adapters/cloned.rs:27:88: 27:90 (#0), name: Param(DefId(0:58908 ~ core[396b]::iter::adapters::cloned::clone_try_fold::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {},
                                        next_early_index: 4,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:5949 ~ core[396b]::iter::adapters::cloned::clone_try_fold),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
  --> library/core/src/iter/adapters/cloned.rs:27:88
   |
27 | fn clone_try_fold<T: Clone, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
   |
   = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:6026 ~ core[396b]::iter::adapters::copied::copy_fold), local_id: 38 }, span: library/core/src/iter/adapters/copied.rs:27:81: 27:83 (#0), name: Param(DefId(0:58918 ~ core[396b]::iter::adapters::copied::copy_fold::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {},
                                        next_early_index: 3,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:6026 ~ core[396b]::iter::adapters::copied::copy_fold),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
  --> library/core/src/iter/adapters/copied.rs:27:81
   |
27 | fn copy_fold<T: Copy, Acc>(mut f: impl FnMut(Acc, T) -> Acc) -> impl FnMut(Acc, &T) -> Acc {
   |
   = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:6032 ~ core[396b]::iter::adapters::copied::copy_try_fold), local_id: 39 }, span: library/core/src/iter/adapters/copied.rs:31:86: 31:88 (#0), name: Param(DefId(0:58920 ~ core[396b]::iter::adapters::copied::copy_try_fold::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {},
                                        next_early_index: 4,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:6032 ~ core[396b]::iter::adapters::copied::copy_try_fold),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
  --> library/core/src/iter/adapters/copied.rs:31:86
   |
31 | fn copy_try_fold<T: Copy, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
   |
   = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
  --> library/core/src/iter/adapters/cloned.rs:27:88
   |
27 | fn clone_try_fold<T: Clone, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
   |
   = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
  --> library/core/src/iter/adapters/copied.rs:27:81
   |
27 | fn copy_fold<T: Copy, Acc>(mut f: impl FnMut(Acc, T) -> Acc) -> impl FnMut(Acc, &T) -> Acc {
   |
   = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
  --> library/core/src/iter/adapters/copied.rs:31:86
   |
31 | fn copy_try_fold<T: Copy, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
   |
   = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:7374 ~ core[396b]::iter::adapters::skip_while::{impl#2}::next::check), local_id: 59 }, span: library/core/src/iter/adapters/skip_while.rs:46:25: 46:27 (#0), name: Param(DefId(0:59168 ~ core[396b]::iter::adapters::skip_while::{impl#2}::next::check::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:7375 ~ core[396b]::iter::adapters::skip_while::{impl#2}::next::check::'a): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:7375 ~ core[396b]::iter::adapters::skip_while::{impl#2}::next::check::'a),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:7374 ~ core[396b]::iter::adapters::skip_while::{impl#2}::next::check),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
  --> library/core/src/iter/adapters/skip_while.rs:46:25
   |
46 |         ) -> impl FnMut(&T) -> bool + 'a {
   |
   = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
  --> library/core/src/iter/adapters/skip_while.rs:46:25
   |
46 |         ) -> impl FnMut(&T) -> bool + 'a {
   |
   = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:8663 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false), local_id: 59 }, span: library/core/src/iter/traits/iterator.rs:2093:26: 2093:32 (#0), name: Param(DefId(0:59396 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::{opaque#1}::'_#2), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:8664 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::'a): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:8664 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::'a),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:8663 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
    --> library/core/src/iter/traits/iterator.rs:2093:26
     |
2093 |         ) -> impl FnMut(&&mut T) -> bool + 'a {
     |
     = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:8663 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false), local_id: 61 }, span: library/core/src/iter/traits/iterator.rs:2093:25: 2093:32 (#0), name: Param(DefId(0:59394 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:8664 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::'a): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:8664 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false::'a),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:8663 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_false),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
    --> library/core/src/iter/traits/iterator.rs:2093:25
     |
2093 |         ) -> impl FnMut(&&mut T) -> bool + 'a {
     |
     = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
    --> library/core/src/iter/traits/iterator.rs:2093:26
     |
2093 |         ) -> impl FnMut(&&mut T) -> bool + 'a {
     |
     = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
    --> library/core/src/iter/traits/iterator.rs:2093:25
     |
2093 |         ) -> impl FnMut(&&mut T) -> bool + 'a {
     |
     = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:8669 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true), local_id: 36 }, span: library/core/src/iter/traits/iterator.rs:2102:77: 2102:84 (#0), name: Param(DefId(0:59401 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::{opaque#1}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:59399 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::'_): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:59399 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::'_),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:8669 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
    --> library/core/src/iter/traits/iterator.rs:2102:77
     |
2102 |         fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
     |
     = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:8669 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true), local_id: 37 }, span: library/core/src/iter/traits/iterator.rs:2102:78: 2102:84 (#0), name: Param(DefId(0:59403 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::{opaque#1}::'_#2), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:59399 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::'_): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:59399 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true::'_),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:8669 ~ core[396b]::iter::traits::iterator::Iterator::partition_in_place::is_true),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
    --> library/core/src/iter/traits/iterator.rs:2102:78
     |
2102 |         fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
     |
     = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
    --> library/core/src/iter/traits/iterator.rs:2102:77
     |
2102 |         fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
     |
     = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
    --> library/core/src/iter/traits/iterator.rs:2102:78
     |
2102 |         fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
     |
     = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7fb71434957d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09508ff2bf609df
   1:     0x7fb7143b1cc8 - core::fmt::write::h75736d5168df1a59
   2:     0x7fb71433a731 - std::io::Write::write_fmt::hd1a6120e10e78abb
   3:     0x7fb71434c74e - std::panicking::default_hook::{{closure}}::hb816096665b89bb7
   4:     0x7fb71434c49e - std::panicking::default_hook::h435d8268c11c2801
   5:     0x7fb714e53de4 - rustc_driver[87078a9a48453e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb71434cec1 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
   7:     0x7fb717b1f993 - std[51639afae0382935]::panicking::begin_panic::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>::{closure#0}
   8:     0x7fb717b1bfd6 - std[51639afae0382935]::sys_common::backtrace::__rust_end_short_backtrace::<std[51639afae0382935]::panicking::begin_panic<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>::{closure#0}, !>
   9:     0x7fb714e0dbe6 - std[51639afae0382935]::panicking::begin_panic::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>
  10:     0x7fb717b29526 - std[51639afae0382935]::panic::panic_any::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>
  11:     0x7fb717b2daad - <rustc_errors[2e9488e57e3b41c0]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7fb714e64cb2 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[aa9f798f87b09fd5]::parse::ParseSess>
  13:     0x7fb714e69215 - <alloc[11297c88fab3d63e]::rc::Rc<rustc_session[aa9f798f87b09fd5]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7fb714ec870c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[16a315bebaa0a951]::interface::Compiler>
  15:     0x7fb714ec6ba7 - rustc_span[58f4f46925086679]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_interface[16a315bebaa0a951]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fb714e6f49a - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[58f4f46925086679]::SessionGlobals>>::set::<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  17:     0x7fb714ec56d9 - std[51639afae0382935]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  18:     0x7fb714ebd429 - <<std[51639afae0382935]::thread::Builder>::spawn_unchecked_<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fb7143593d5 - std::sys::unix::thread::Thread::new::thread_start::h634e4e323cdffe8d
  20:     0x7fb70e8a8609 - start_thread
  21:     0x7fb7141bb133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (95c33f77b 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:308 ~ hashbrown[c476]::raw::{impl#11}::prepare_resize), local_id: 132 }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/raw/mod.rs:1349:64: 1349:73 (#0), name: Param(DefId(0:2517 ~ hashbrown[c476]::raw::{impl#11}::prepare_resize::{opaque#0}::'_), Fresh) } in scope ObjectLifetimeDefault {
                                    lifetime: Some(
                                    ),
                                    s: ObjectLifetimeDefault {
                                        lifetime: Some(
                                            Static,
                                            Static,
                                        ),
                                        s: Elision {
                                            s: Binder {
                                                lifetimes: {
                                                    DefId(0:2516 ~ hashbrown[c476]::raw::{impl#11}::prepare_resize::'_): LateBound(
                                                        DebruijnIndex(0),
                                                        0,
                                                        DefId(0:2516 ~ hashbrown[c476]::raw::{impl#11}::prepare_resize::'_),
                                                },
                                                next_early_index: 1,
                                                opaque_type_parent: true,
                                                scope_type: Normal,
                                                scope_type: Normal,
                                                hir_id: HirId {
                                                    owner: DefId(0:308 ~ hashbrown[c476]::raw::{impl#11}::prepare_resize),
                                                    local_id: 0,
                                                },
                                                s: TraitRefBoundary {
                                                    s: Binder {
                                                        lifetimes: {},
                                                        next_early_index: 1,
                                                        opaque_type_parent: true,
                                                        scope_type: Normal,
                                                        hir_id: HirId {
                                                            owner: DefId(0:283 ~ hashbrown[c476]::raw::{impl#11}),
                                                            local_id: 0,
                                                        s: Root,
                                                        where_bound_origin: None,
                                                    },
                                                },
                                                },
                                                where_bound_origin: None,
                                            },
                                        },
                                    },
                                }
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/raw/mod.rs:1349:64
     |
1349 |     ) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self)>, TryReserveError> {
     |
     = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/raw/mod.rs:1349:64
     |
1349 |     ) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self)>, TryReserveError> {
     |
     = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:573 ~ hashbrown[c476]::map::make_hasher), local_id: 48 }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:212:68: 212:75 (#0), name: Param(DefId(0:2570 ~ hashbrown[c476]::map::make_hasher::{opaque#0}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:2569 ~ hashbrown[c476]::map::make_hasher::'_): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:2569 ~ hashbrown[c476]::map::make_hasher::'_),
                                        },
                                        next_early_index: 4,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:573 ~ hashbrown[c476]::map::make_hasher),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:212:68
    |
212 | pub(crate) fn make_hasher<K, Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
    |
    = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:212:68
    |
212 | pub(crate) fn make_hasher<K, Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
    |
    = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:580 ~ hashbrown[c476]::map::equivalent_key), local_id: 39 }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:224:46: 224:53 (#0), name: Param(DefId(0:2574 ~ hashbrown[c476]::map::equivalent_key::{opaque#0}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:2573 ~ hashbrown[c476]::map::equivalent_key::'_): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:2573 ~ hashbrown[c476]::map::equivalent_key::'_),
                                        },
                                        next_early_index: 3,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:580 ~ hashbrown[c476]::map::equivalent_key),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:224:46
    |
224 | fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
    |
    = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:224:46
    |
224 | fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
    |
    = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30


error: internal compiler error: Could not resolve Lifetime { hir_id: HirId { owner: DefId(0:586 ~ hashbrown[c476]::map::equivalent), local_id: 37 }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:235:39: 235:41 (#0), name: Param(DefId(0:2578 ~ hashbrown[c476]::map::equivalent::{opaque#0}::'_), Fresh) } in scope Elision {
                                    s: Binder {
                                        lifetimes: {
                                            DefId(0:2577 ~ hashbrown[c476]::map::equivalent::'_): LateBound(
                                                DebruijnIndex(0),
                                                0,
                                                DefId(0:2577 ~ hashbrown[c476]::map::equivalent::'_),
                                        },
                                        next_early_index: 2,
                                        opaque_type_parent: true,
                                        scope_type: Normal,
                                        scope_type: Normal,
                                        hir_id: HirId {
                                            owner: DefId(0:586 ~ hashbrown[c476]::map::equivalent),
                                            local_id: 0,
                                        s: Root,
                                        where_bound_origin: None,
                                    },
                                }
                                }
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:235:39
    |
235 | fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
    |
    = note: delayed at compiler/rustc_resolve/src/late/lifetimes.rs:1676:23

error: internal compiler error: unelided lifetime in signature
error: internal compiler error: unelided lifetime in signature
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:235:39
    |
235 | fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
    |
    = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:247:30

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7fc18113157d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09508ff2bf609df
   1:     0x7fc181199cc8 - core::fmt::write::h75736d5168df1a59
   2:     0x7fc181122731 - std::io::Write::write_fmt::hd1a6120e10e78abb
   3:     0x7fc18113474e - std::panicking::default_hook::{{closure}}::hb816096665b89bb7
   4:     0x7fc18113449e - std::panicking::default_hook::h435d8268c11c2801
   5:     0x7fc181c3bde4 - rustc_driver[87078a9a48453e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc181134ec1 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
   7:     0x7fc184907993 - std[51639afae0382935]::panicking::begin_panic::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>::{closure#0}
   8:     0x7fc184903fd6 - std[51639afae0382935]::sys_common::backtrace::__rust_end_short_backtrace::<std[51639afae0382935]::panicking::begin_panic<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>::{closure#0}, !>
   9:     0x7fc181bf5be6 - std[51639afae0382935]::panicking::begin_panic::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>
  10:     0x7fc184911526 - std[51639afae0382935]::panic::panic_any::<rustc_errors[2e9488e57e3b41c0]::ExplicitBug>
  11:     0x7fc184915aad - <rustc_errors[2e9488e57e3b41c0]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7fc181c4ccb2 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[aa9f798f87b09fd5]::parse::ParseSess>
  13:     0x7fc181c51215 - <alloc[11297c88fab3d63e]::rc::Rc<rustc_session[aa9f798f87b09fd5]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7fc181cb070c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[16a315bebaa0a951]::interface::Compiler>
  15:     0x7fc181caeba7 - rustc_span[58f4f46925086679]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_interface[16a315bebaa0a951]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fc181c5749a - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[58f4f46925086679]::SessionGlobals>>::set::<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  17:     0x7fc181cad6d9 - std[51639afae0382935]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  18:     0x7fc181ca5429 - <<std[51639afae0382935]::thread::Builder>::spawn_unchecked_<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fc1811413d5 - std::sys::unix::thread::Thread::new::thread_start::h634e4e323cdffe8d
  20:     0x7fc17b690609 - start_thread
  21:     0x7fc180fa3133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

