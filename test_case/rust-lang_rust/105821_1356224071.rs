

warning: functions generic over types or consts must be mangled
   --> src/fair_mutex_minimal.rs:107:1
    |
106 |   #[no_mangle]
    |   ------------ help: remove this attribute
107 | / pub fn lock_cool_name<T>(slf: &FairMutex<T>) -> Guard<'_, T> {
108 | |     let raw = slf.state.fetch_add(SECOND_CNT_ONE, Ordering::Acquire) & TICKET_MASK;
109 | |     let ticket = raw & TICKET_MASK;
110 | |     // println!("tid: {} ticket: {}", thread_id::get(), ticket);
...   |
120 | |     Guard(slf)
121 | | }
    | |_^

warning: functions generic over types or consts must be mangled
   --> src/fair_mutex_minimal.rs:126:1
    |
124 |   #[no_mangle]
    |   ------------ help: remove this attribute
125 |   #[inline(never)]
126 | / pub fn lock_slow_cool_name<T>(slf: &FairMutex<T>, ticket: usize) {
127 | |     let mut spin_wait = SpinWait::new();
128 | |     let mut state = slf.state.load(Ordering::Relaxed);
129 | |     while (curr_ticket_to_ticket(get_curr_ticket(state)))/*((state & !PARKED_BIT) >> (usize::BITS / 2))*/ != ticket {
...   |
140 | |     }
141 | | }
    | |_^

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:24:10
   |
24 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:25:10
   |
25 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:24:10
   |
24 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:24:10
   |
24 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:25:10
   |
25 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:25:10
   |
25 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: expected fullfillment errors
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:461:23

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:127:10
    |
127 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:128:10
    |
128 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:127:10
    |
127 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:127:10
    |
127 |     ptr: [NonNull<T>; const { T::PTRS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:128:10
    |
128 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:128:10
    |
128 |     val: [ManuallyDrop<Unaligned<T>>; const { T::VALS_LEN }],
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:34:31
   |
34 |                 ptr: unsafe { transmute([create(val)]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:34:31
   |
34 |                 ptr: unsafe { transmute([create(val)]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:34:31
   |
34 |                 ptr: unsafe { transmute([create(val)]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:35:31
   |
35 |                 val: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:35:31
   |
35 |                 val: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:35:31
   |
35 |                 val: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:39:31
   |
39 |                 ptr: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:39:31
   |
39 |                 ptr: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:39:31
   |
39 |                 ptr: unsafe { transmute([]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
  --> src/inlinable_ptr.rs:40:31
   |
40 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
  --> src/inlinable_ptr.rs:40:31
   |
40 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
  --> src/inlinable_ptr.rs:40:31
   |
40 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
   |                               ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/compiler/rustc_middle/src/ty/relate.rs:417:59

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:137:31
    |
137 |                 ptr: unsafe { transmute([create(val)]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:137:31
    |
137 |                 ptr: unsafe { transmute([create(val)]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:137:31
    |
137 |                 ptr: unsafe { transmute([create(val)]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:138:31
    |
138 |                 val: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:138:31
    |
138 |                 val: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:138:31
    |
138 |                 val: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:142:31
    |
142 |                 ptr: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:142:31
    |
142 |                 ptr: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:142:31
    |
142 |                 ptr: unsafe { transmute([]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: Missing value for constant, but no error reported?
   --> src/inlinable_ptr.rs:143:31
    |
143 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:77:71

error: internal compiler error: `report_selection_error` did not emit an error
   --> src/inlinable_ptr.rs:143:31
    |
143 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:592:22

error: internal compiler error: `ErrorGuaranteed` without an error
   --> src/inlinable_ptr.rs:143:31
    |
143 |                 val: unsafe { transmute([ManuallyDrop::new(Unaligned(val))]) },
    |                               ^^^^^^^^^
    |
    = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1372:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:638:18

error: internal compiler error: broken MIR in DefId(0:912 ~ AssemblyTest[7d02]::inlinable_ptr::{impl#0}::new_with) ("return type"): bad type [type error]
  --> src/inlinable_ptr.rs:31:5
   |
31 |     pub fn new_with(val: T, create: impl FnMut(T) -> NonNull<T>) -> Self {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:768:20

error: internal compiler error: broken MIR in DefId(0:912 ~ AssemblyTest[7d02]::inlinable_ptr::{impl#0}::new_with) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/inlinable_ptr.rs:31:5: 31:73 (#0), scope: scope[0] } }): bad type [type error]
  --> src/inlinable_ptr.rs:31:5
   |
31 |     pub fn new_with(val: T, create: impl FnMut(T) -> NonNull<T>) -> Self {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

error: internal compiler error: broken MIR in DefId(0:940 ~ AssemblyTest[7d02]::inlinable_ptr::{impl#5}::new_with) ("return type"): bad type [type error]
   --> src/inlinable_ptr.rs:134:5
    |
134 |     pub fn new_with(val: T, create: impl FnMut(T) -> NonNull<T>) -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

error: internal compiler error: broken MIR in DefId(0:940 ~ AssemblyTest[7d02]::inlinable_ptr::{impl#5}::new_with) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/inlinable_ptr.rs:134:5: 134:73 (#0), scope: scope[0] } }): bad type [type error]
   --> src/inlinable_ptr.rs:134:5
    |
134 |     pub fn new_with(val: T, create: impl FnMut(T) -> NonNull<T>) -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1610:13
stack backtrace:
   0:     0x7f15bca3bd5a - std::backtrace_rs::backtrace::libunwind::trace::hf3e23dba944728e7
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f15bca3bd5a - std::backtrace_rs::backtrace::trace_unsynchronized::ha7a661dbe14f9bec
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f15bca3bd5a - std::sys_common::backtrace::_print_fmt::h8bb3e793399a1d31
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f15bca3bd5a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8f001b4156af7ff0
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f15b89c836e - core::fmt::write::h98c67bacb0ad9cb8
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f15bca2fe35 - std::io::Write::write_fmt::h21d43ee83b8bf3da
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/io/mod.rs:1682:15
   6:     0x7f15bca3bb25 - std::sys_common::backtrace::_print::hed3e659c4722fff2
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f15bca3bb25 - std::sys_common::backtrace::print::hf3af686623d05885
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f15bca3deaf - std::panicking::default_hook::{{closure}}::hfbc532e2813a4fd8
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/panicking.rs:267:22
   9:     0x7f15bca3dbeb - std::panicking::default_hook::hf22d9a6bedeb8cbb
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/panicking.rs:286:9
  10:     0x7f15bbc67351 - rustc_driver[8203396c4671b89c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f15bca3e6ad - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h839d8563a5defb2f
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/alloc/src/boxed.rs:2032:9
  12:     0x7f15bca3e6ad - std::panicking::rust_panic_with_hook::h2e7c62506c5adf89
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/panicking.rs:692:13
  13:     0x7f15bbca04d1 - std[fecc2c98d64c5e20]::panicking::begin_panic::<rustc_errors[6c86e693eff69e36]::ExplicitBug>::{closure#0}
  14:     0x7f15bbc9eda6 - std[fecc2c98d64c5e20]::sys_common::backtrace::__rust_end_short_backtrace::<std[fecc2c98d64c5e20]::panicking::begin_panic<rustc_errors[6c86e693eff69e36]::ExplicitBug>::{closure#0}, !>
  15:     0x7f15bbc7dd66 - std[fecc2c98d64c5e20]::panicking::begin_panic::<rustc_errors[6c86e693eff69e36]::ExplicitBug>
  16:     0x7f15bbc9c806 - std[fecc2c98d64c5e20]::panic::panic_any::<rustc_errors[6c86e693eff69e36]::ExplicitBug>
  17:     0x7f15bb155906 - <rustc_errors[6c86e693eff69e36]::HandlerInner>::flush_delayed::<alloc[4e38580f0181725e]::vec::Vec<rustc_errors[6c86e693eff69e36]::diagnostic::Diagnostic>, &str>
  18:     0x7f15baeee0ac - <rustc_interface[3feed755d27e3616]::passes::QueryContext>::enter::<<rustc_interface[3feed755d27e3616]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[4c48bf2e04a135c7]::result::Result<alloc[4e38580f0181725e]::boxed::Box<dyn core[4c48bf2e04a135c7]::any::Any>, rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>
  19:     0x7f15baeeb586 - <rustc_interface[3feed755d27e3616]::queries::Queries>::ongoing_codegen
  20:     0x7f15baeeaaa7 - <rustc_interface[3feed755d27e3616]::interface::Compiler>::enter::<rustc_driver[8203396c4671b89c]::run_compiler::{closure#1}::{closure#2}, core[4c48bf2e04a135c7]::result::Result<core[4c48bf2e04a135c7]::option::Option<rustc_interface[3feed755d27e3616]::queries::Linker>, rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>
  21:     0x7f15baee5a88 - rustc_span[b683b297262634fe]::with_source_map::<core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>, rustc_interface[3feed755d27e3616]::interface::run_compiler<core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>, rustc_driver[8203396c4671b89c]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7f15baee5575 - <scoped_tls[b4ee55afb72dd1f]::ScopedKey<rustc_span[b683b297262634fe]::SessionGlobals>>::set::<rustc_interface[3feed755d27e3616]::interface::run_compiler<core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>, rustc_driver[8203396c4671b89c]::run_compiler::{closure#1}>::{closure#0}, core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>
  23:     0x7f15baee4b62 - std[fecc2c98d64c5e20]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3feed755d27e3616]::util::run_in_thread_pool_with_globals<rustc_interface[3feed755d27e3616]::interface::run_compiler<core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>, rustc_driver[8203396c4671b89c]::run_compiler::{closure#1}>::{closure#0}, core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>
  24:     0x7f15bb5251fe - <<std[fecc2c98d64c5e20]::thread::Builder>::spawn_unchecked_<rustc_interface[3feed755d27e3616]::util::run_in_thread_pool_with_globals<rustc_interface[3feed755d27e3616]::interface::run_compiler<core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>, rustc_driver[8203396c4671b89c]::run_compiler::{closure#1}>::{closure#0}, core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4c48bf2e04a135c7]::result::Result<(), rustc_errors[6c86e693eff69e36]::ErrorGuaranteed>>::{closure#1} as core[4c48bf2e04a135c7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f15bca45923 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha40757527af5f65d
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/alloc/src/boxed.rs:2000:9
  26:     0x7f15bca45923 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf5bd55cd4ab92745
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/alloc/src/boxed.rs:2000:9
  27:     0x7f15bca45923 - std::sys::unix::thread::Thread::new::thread_start::h087de3b4a94ba02b
                               at /rustc/9c07efe84f28a44f3044237696acc295aa407ee5/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f15b871414d - start_thread
  29:     0x7f15b8795a00 - __GI___clone3
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (9c07efe84 2022-12-16) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `AssemblyTest` (bin "AssemblyTest") generated 158 warnings (run `cargo fix --bin "AssemblyTest"` to apply 59 suggestions)
error: could not compile `AssemblyTest`; 158 warnings emitted

