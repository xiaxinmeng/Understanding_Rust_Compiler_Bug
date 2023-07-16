plain
.................................................................................................... 9300/11533
.................................................................................................... 9400/11533
..................................................................................i.......i......... 9500/11533
.................................................................................................... 9600/11533
.....................iiiiiii..iiiiii.i.............................................................. 9700/11533
.................................................................................................... 9900/11533
.................................................................................................... 10000/11533
.................................................................................................... 10100/11533
.................................................................................................... 10200/11533
---

---- [ui] ui/borrowck/issue-82126-mismatched-subst-and-hir.rs stdout ----
diff of stderr:

4 LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
5    |                                                           ^^^^^^^^^^^^---- help: remove these generics
-    |                                                           expected 0 lifetime arguments
+    |                                                           expected  0 lifetime arguments
8    |
8    |
- note: struct defined here, with 0 lifetime parameters
+ note: struct defined here, with  0 lifetime parameters 
11    |
11    |
12 LL | struct LockedMarket<T>(T);
13    |        ^^^^^^^^^^^^
14 
14 
- error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
+ error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
17    |
17    |
18 LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
-    |                                                           ^^^^^^^^^^^^ expected 1 type argument
+    |                                                           ^^^^^^^^^^^^ expected  1 generic argument
20    |
20    |
- note: struct defined here, with 1 type parameter: `T`
+ note: struct defined here, with  1 generic parameter : `T`
23    |
23    |
24 LL | struct LockedMarket<T>(T);
25    |        ^^^^^^^^^^^^ -
- help: add missing type argument
- help: add missing type argument
+ help: add missing generic argument
27    |
28 LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_, T> {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-82126-mismatched-subst-and-hir/issue-82126-mismatched-subst-and-hir.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-82126-mismatched-subst-and-hir.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-82126-mismatched-subst-and-hir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-82126-mismatched-subst-and-hir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-82126-mismatched-subst-and-hir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
   |                                                           ^^^^^^^^^^^^---- help: remove these generics
   |                                                           expected  0 lifetime arguments
   |
   |
note: struct defined here, with  0 lifetime parameters 
   |
   |
LL | struct LockedMarket<T>(T);


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   |
   |
LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
   |                                                           ^^^^^^^^^^^^ expected  1 generic argument
   |
note: struct defined here, with  1 generic parameter : `T`
   |
   |
LL | struct LockedMarket<T>(T);
help: add missing generic argument
   |
   |
LL | async fn buy_lock(generator: &Mutex<MarketMultiplier>) -> LockedMarket<'_, T> {

error[E0515]: cannot return value referencing temporary value
  --> /checkout/src/test/ui/borrowck/issue-82126-mismatched-subst-and-hir.rs:19:5
   |
   |
LL |     LockedMarket(generator.lock().unwrap().buy())
   |     |            |
   |     |            temporary value created here
   |     |            temporary value created here
   |     returns a value referencing data owned by the current function
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0515.
For more information about an error, try `rustc --explain E0107`.
For more information about an error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-81712-cyclic-traits.rs stdout ----
diff of stderr:

1 error[E0107]: missing generics for associated type `C::DType`
-   --> $DIR/issue-81712-cyclic-traits.rs:14:10
3    |
3    |
- LL |     type DType<T>: D<T, CType = Self>;
-    |          ^^^^^ expected 1 type argument
+ LL |     type CType: C<DType = Self>;
+    |                   ^^^^^^^^^^^^ expected  1 generic argument
6    |
- note: associated type defined here, with 1 type parameter: `T`
+ note: associated type defined here, with  1 generic parameter : `T`
9    |
9    |
10 LL |     type DType<T>: D<T, CType = Self>;
11    |          ^^^^^ -
11    |          ^^^^^ -
- help: use angle brackets to add missing type argument
+ help: add missing generic argument
13    |
- LL |     type DType<T><T>: D<T, CType = Self>;
-    |               ^^^
+ LL |     type CType: C<DType<T>>;
16 
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits/issue-81712-cyclic-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-81712-cyclic-traits.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-81712-cyclic-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: missing generics for associated type `C::DType`
   |
   |
LL |     type CType: C<DType = Self>;
   |                   ^^^^^^^^^^^^ expected  1 generic argument
   |
note: associated type defined here, with  1 generic parameter : `T`
   |
   |
LL |     type DType<T>: D<T, CType = Self>;
help: add missing generic argument
   |
   |
LL |     type CType: C<DType<T>>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
For more information about this error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/issues/issue-53251.rs stdout ----
diff of stderr:

1 error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
-   --> $DIR/issue-53251.rs:13:20
3    |
3    |
4 LL |                 S::f::<i64>();
5    |                    ^------- help: remove these generics
10    | --------------- in this macro invocation
11    |
11    |
12 note: associated function defined here, with  0 generic parameters 
-   --> $DIR/issue-53251.rs:6:8
14    |
15 LL |     fn f() {}
16    |        ^


17    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
18 
19 error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
-   --> $DIR/issue-53251.rs:13:20
21    |
21    |
22 LL |                 S::f::<i64>();
23    |                    ^------- help: remove these generics
28    | --------------- in this macro invocation
29    |
29    |
30 note: associated function defined here, with  0 generic parameters 
-   --> $DIR/issue-53251.rs:6:8
32    |
33 LL |     fn f() {}
34    |        ^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53251/issue-53251.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-53251.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53251.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53251" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53251/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |                 S::f::<i64>();
   |                    ^------- help: remove these generics
   |                    expected  0 generic arguments
...
...
LL | impl_add!(a b);
   |
   |
note: associated function defined here, with  0 generic parameters 
   |
LL |     fn f() {}
   |        ^
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |                 S::f::<i64>();
   |                    ^------- help: remove these generics
   |                    expected  0 generic arguments
...
...
LL | impl_add!(a b);
   |
   |
note: associated function defined here, with  0 generic parameters 
   |
LL |     fn f() {}
   |        ^
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---


---- [ui] ui/methods/method-call-lifetime-args-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'visited lifetime HirId { owner: DefId(0:6 ~ method_call_lifetime_args_fail[317d]::{impl#0}::late), local_id: 9 } twice', compiler/rustc_resolve/src/late/lifetimes.rs:3105:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (080fe1883 2021-03-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [get_lifetime_scope_info_for_diagnostics] get lifetime scope map
#1 [lifetime_scope_map] finds the lifetime scope for an HirId of a PathSegment
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str_with_substs
             10: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str
             11: <rustc_typeck::structured_errors::wrong_number_of_generic_args::WrongNumberOfGenericArgs as rustc_typeck::structured_errors::StructuredDiagnostic>::diagnostic_common
             12: rustc_typeck::structured_errors::StructuredDiagnostic::diagnostic
             13: rustc_typeck::astconv::generics::<impl dyn rustc_typeck::astconv::AstConv>::check_generic_arg_count
             14: rustc_typeck::astconv::generics::<impl dyn rustc_typeck::astconv::AstConv>::check_generic_arg_count_for_call
             15: rustc_typeck::check::method::confirm::ConfirmContext::confirm
             16: rustc_typeck::check::method::confirm::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_method
             17: rustc_typeck::check::method::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::lookup_method
             18: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             19: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             20: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
             21: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
             22: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             23: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             24: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
             25: rustc_typeck::check::check::check_fn
             26: rustc_infer::infer::InferCtxtBuilder::enter
             27: rustc_typeck::check::inherited::InheritedBuilder::enter
             28: rustc_typeck::check::typeck
             29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             30: rustc_data_structures::stack::ensure_sufficient_stack
             31: rustc_query_system::query::plumbing::force_query_with_job
             32: rustc_query_system::query::plumbing::get_query_impl
             33: rustc_query_system::query::plumbing::get_query
             34: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             35: rustc_typeck::check::typeck_item_bodies
             36: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             37: rustc_data_structures::stack::ensure_sufficient_stack
             38: rustc_query_system::query::plumbing::force_query_with_job
             39: rustc_query_system::query::plumbing::get_query_impl
             40: rustc_query_system::query::plumbing::get_query
             41: rustc_session::utils::<impl rustc_session::session::Session>::time
             42: rustc_typeck::check_crate
             43: rustc_interface::passes::analysis
             44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             45: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             46: rustc_data_structures::stack::ensure_sufficient_stack
             47: rustc_query_system::query::plumbing::force_query_with_job
             48: rustc_query_system::query::plumbing::get_query_impl
             49: rustc_query_system::query::plumbing::get_query
             50: rustc_interface::passes::QueryContext::enter
             51: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             52: rustc_span::with_source_map
             53: scoped_tls::ScopedKey<T>::set
             54: rustc_span::with_session_globals
             56: core::ops::function::FnOnce::call_once{{vtable.shim}}
             57: std::sys::unix::thread::Thread::new::thread_start
             58: start_thread
             59: __clone
             59: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1012:13
stack backtrace:
   0:     0x7f6f8d26d1bf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h333da29b436d78d6
   1:     0x7f6f8d2e492c - core::fmt::write::hf4433bc6cddbea6a
   2:     0x7f6f8d2600e5 - std::io::Write::write_fmt::h52ffeb70991ae796
   3:     0x7f6f8d2717f2 - std::panicking::default_hook::{{closure}}::hf16dca9b28c042c1
   4:     0x7f6f8d2711a7 - std::panicking::default_hook::hf1791697d850052e
   5:     0x7f6f8db6a62d - rustc_driver::report_ice::h2bf4396aad7a8851
   6:     0x7f6f8d272123 - std::panicking::rust_panic_with_hook::h9abdb4d9ba4774fb
   7:     0x7f6f8d271c67 - std::panicking::begin_panic_handler::{{closure}}::h4af8e8cf1f15274b
   8:     0x7f6f8d26d68c - std::sys_common::backtrace::__rust_end_short_backtrace::h6cf0c52d177f5a21
   9:     0x7f6f8d271bc9 - rust_begin_unwind
  10:     0x7f6f8d271b7b - std::panicking::begin_panic_fmt::h3a1965befd5bad5d
  11:     0x7f6f9035f014 - rustc_errors::HandlerInner::flush_delayed::h4e2ab043c47e61f3
  12:     0x7f6f9035c09c - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h5a57a2f1baa24824
  13:     0x7f6f8db78322 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::hee9bc175caf6ffd1
  14:     0x7f6f8db7d83a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h63f674b93207bc8f
  15:     0x7f6f8db790bd - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h89088c91349c17ed
  16:     0x7f6f8db73644 - rustc_span::with_source_map::h2ed2fe6b4fd5fa7b
  17:     0x7f6f8db7fb12 - scoped_tls::ScopedKey<T>::set::hca0da391d3f11bfa
  18:     0x7f6f8db73ad3 - rustc_span::with_session_globals::h46940ae36298f4f4
  19:     0x7f6f8db8331b - std::sys_common::backtrace::__rust_begin_short_backtrace::h5ac45e11dcd6016f
  20:     0x7f6f8dbd968a - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf8658d8f476585e4
  21:     0x7f6f8d28377a - std::sys::unix::thread::Thread::new::thread_start::h0d674cc134c80c83
  22:     0x7f6f883c06db - start_thread
  23:     0x7f6f8cf0071f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (080fe1883 2021-03-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
------------------------------------------


---- [ui] ui/suggestions/missing-lifetime-specifier.rs stdout ----
---- [ui] ui/suggestions/missing-lifetime-specifier.rs stdout ----
diff of stderr:

166 LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
168 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:18:44
+    |
+    |
+ LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
+    |                                            ^^^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:18:44
+    |
+    |
+ LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
+    |                                            ^^^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:23:44
+    |
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:23:45
+    |
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    |                                             ^^^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:23:44
+    |
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:23:45
+    |
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    |                                             ^^^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:30:48
+    |
+    |
+ LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
+    |                                                ^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:30:48
+    |
+    |
+ LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
+    |                                                ^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:35:44
+    |
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:35:49
+    |
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    |                                                 ^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:35:44
+    |
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/missing-lifetime-specifier.rs:35:49
+    |
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    |                                                 ^ expected 2 lifetime parameters
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:50:44
+    |
+    |
+ LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/missing-lifetime-specifier.rs:50:44
+    |
+    |
+ LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
+    |                                            ^ expected named lifetime parameter
+    |
+    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
+ help: consider using the `'static` lifetime
+    |
+ LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ 
169 error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
171    |


310 LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<Self, 'static, i32>>>>> = RefCell::new(HashMap::new());
312 
- error: aborting due to 22 previous errors
+ error: aborting due to 36 previous errors
314 
314 
315 Some errors have detailed explanations: E0106, E0107.
316 For more information about an error, try `rustc --explain E0106`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/missing-lifetime-specifier.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/missing-lifetime-specifier.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18:44
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18:44
   |
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:44
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:44
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:30:48
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
   |                                                ^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:30:48
   |
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
   |                                                ^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35:44
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35:49
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                                 ^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35:44
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35:49
   |
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   |                                                 ^ expected 2 lifetime parameters
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:50:44
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:50:44
   |
   |
---
test result: FAILED. 11435 passed; 5 failed; 93 ignored; 0 measured; 0 filtered out; finished in 138.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:11
