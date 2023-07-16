plain
 finished in 0.506 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
FFFFF................................FF......F.F...FF.F.....F...FFFF....F............F.. 88/154
............................FF.........F....F.FFF.................

---- [incremental] src/test/incremental/change_pub_inherent_method_body/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(fn_calls_changed_method::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(fn_calls_another_method::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(make_origin)` should be clean but is not
   |
LL |     pub fn make_origin() -> Point {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(get_x)` should be clean but is not
   |
   |
LL |     pub fn get_x(p: Point) -> f32 {


error: `hir_owner_nodes(inc_x)` should be clean but is not
   |
   |
LL |     pub fn inc_x(p: &mut Point) {

error: aborting due to 5 previous errors
------------------------------------------
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


---- [incremental] src/test/incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(fn_calls_methods_in_same_impl::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `typeck(fn_calls_methods_in_same_impl::check)` should have been loaded from disk but it was not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(fn_calls_methods_in_another_impl::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(make_origin)` should be clean but is not
   |
LL |     pub fn make_origin() -> Point {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(get_x)` should be clean but is not
   |
   |
LL |     pub fn get_x(p: Point) -> f32 {


error: `hir_owner_nodes(inc_x)` should be clean but is not
   |
   |
LL |     pub fn inc_x(p: &mut Point) {

error: aborting due to 6 previous errors
------------------------------------------



---- [incremental] src/test/incremental/change_private_impl_method/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(fn_calls_methods_in_same_impl::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(fn_calls_methods_in_another_impl::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(make_origin)` should be clean but is not
   |
LL |     pub fn make_origin() -> Point {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(get_x)` should be clean but is not
   |
   |
LL |     pub fn get_x(p: Point) -> f32 {


error: `hir_owner_nodes(inc_x)` should be clean but is not
   |
   |
LL |     pub fn inc_x(p: &mut Point) {

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] src/test/incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(fn_calls_changed_method::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(fn_calls_another_method::check)` should be clean but is not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: `hir_owner_nodes(make_origin)` should be clean but is not
   |
LL |     pub fn make_origin() -> Point {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(get_x)` should be clean but is not
   |
   |
LL |     pub fn get_x(p: Point) -> f32 {


error: `hir_owner_nodes(inc_x)` should be clean but is not
   |
   |
LL |     pub fn inc_x(p: &mut Point) {

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] src/test/incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(fn_with_type_in_sig::boop)` should be clean but is not
   |
   |
LL |     pub fn boop(p: Option<&Point>) -> f32 {


error: `hir_owner_nodes(call_fn_with_type_in_sig::bip)` should be clean but is not
   |
   |
LL |     pub fn bip() -> f32 {


error: `hir_owner_nodes(fn_with_type_in_body::boop)` should be clean but is not
   |
   |
LL |     pub fn boop() -> f32 {


error: `hir_owner_nodes(call_fn_with_type_in_body::bip)` should be clean but is not
   |
   |
LL |     pub fn bip() -> f32 {


error: `hir_owner_nodes(make_origin)` should be clean but is not
   |
   |
LL |     pub fn make_origin(p: Point) -> Point {


error: `hir_owner_nodes(get_x)` should be clean but is not
   |
   |
LL |     pub fn get_x(p: Point) -> f32 {


error: `hir_owner_nodes(inc_x)` should be clean but is not
   |
   |
LL |     pub fn inc_x(p: &mut Point) {

error: aborting due to 7 previous errors
------------------------------------------



---- [incremental] src/test/incremental/hashes/enum_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(EnumVisibility)` should be clean but is not
   |
   |
LL | enum     EnumVisibility { A }


error: `hir_owner_nodes(EnumAddMustUse)` should be clean but is not
   |
   |
LL | / enum EnumAddMustUse {
LL | |     Variant1,
LL | |     Variant2,
LL | | }


error: `hir_owner_nodes(EnumAddReprC)` should be clean but is not
   |
   |
LL | / enum EnumAddReprC {
LL | |     Variant1,
LL | |     Variant2,
LL | | }

error: aborting due to 3 previous errors
------------------------------------------



---- [incremental] src/test/incremental/dirty_clean.rs stdout ----

error in revision `cfail2`: /checkout/src/test/incremental/dirty_clean.rs:46: unexpected error: '46:5: 46:15: `hir_owner_nodes(z)` should be clean but is not'

error in revision `cfail2`: /checkout/src/test/incremental/dirty_clean.rs:32: expected error not found: `hir_owner_nodes(y)` should be dirty but is not

error in revision `cfail2`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
    Error {
        line_num: 46,
        kind: Some(
            Error,
            Error,
        ),
        msg: "46:5: 46:15: `hir_owner_nodes(z)` should be clean but is not",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 32,
        kind: Some(
            Error,
        ),
        msg: "`hir_owner_nodes(y)` should be dirty but is not",
]

thread '[incremental] src/test/incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1392:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [incremental] src/test/incremental/hashes/function_interfaces.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(inline)` should be clean but is not
   |
LL | pub fn inline() {}
   | ^^^^^^^^^^^^^^^


error: `hir_owner_nodes(inline_never)` should be clean but is not
   |
LL | pub fn inline_never() {}
   | ^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(no_mangle)` should be clean but is not
   |
LL | pub fn no_mangle() {}
   | ^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(linkage)` should be clean but is not
   |
LL | pub fn linkage() {}
   | ^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(change_return_impl_trait)` should be clean but is not
   |
   |
LL | pub fn change_return_impl_trait() -> impl Clone {

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] src/test/incremental/hashes/inherent_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //--------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //--------------------------------------------------------------------------------------
...  |
LL | |     }
LL | | }
   | |_^

error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------
LL | |     //---------------
LL | |     //---------------
LL | |     //------------------------------------------------------------
...  |
LL | |     }
LL | | }
   | |_^

error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //----------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn method_privacy() { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn method_selfmutness(&    self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn add_method_parameter(&self        ) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn change_method_parameter_name(&self, a: i64) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn change_method_return_type(&self) -> u16 { 0 }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //--------------------------
...  |
LL | |     pub fn make_method_inline(&self) -> u8 { 0 }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub fn change_method_parameter_order(&self, a: i64, b: i64) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //------------------------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub        fn make_method_unsafe(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //----------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //----------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub            fn make_method_extern(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     //----------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     //--------------------------
LL | |     //----------------------------------------------------------------------------
LL | |     //--------------------------
LL | |     pub extern "C"      fn change_method_calling_convention(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | |     // -----------------------------------------------------
LL | |     // ---------------------------------------------------------
LL | |     // ---------------------------------------------------------
LL | |     // ----------------------------------------------------------
...  |
LL | |     pub fn add_lifetime_parameter_to_method    (&self) { }
---

warning: function is never used: `baz`
  --> /checkout/src/test/incremental/ich_method_call_trait_scope.rs:35:8
   |
LL |     fn baz() {


error: `hir_owner_nodes(bar)` should be clean but is not
   |
LL |     fn bar() {
   |     ^^^^^^^^


error: `hir_owner_nodes(baz)` should be clean but is not
   |
   |
LL |     fn baz() {

error: aborting due to 2 previous errors; 3 warnings emitted
------------------------------------------



---- [incremental] src/test/incremental/hello_world.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hello_world.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/hello_world.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(yyyy)` should be clean but is not
   |
   |
LL |     pub fn yyyy() {


error: `hir_owner_nodes(z)` should be clean but is not
   |
   |
LL |     pub fn z() {

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] src/test/incremental/ich_resolve_results.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function is never used: `test`
   |
   |
LL | fn test<T>() { }
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Foo`
---

warning: struct is never constructed: `Foo`
  --> /checkout/src/test/incremental/ich_resolve_results.rs:16:16
   |
LL |     pub struct Foo(pub i64);

warning: function is never used: `in_expr`
  --> /checkout/src/test/incremental/ich_resolve_results.rs:34:8
   |
---
   |
LL |     fn in_type() {
   |        ^^^^^^^

error: `hir_owner_nodes(in_expr)` should be clean but is not
   |
LL |     fn in_expr() {
   |     ^^^^^^^^^^^^


error: `hir_owner_nodes(in_type)` should be clean but is not
   |
LL |     fn in_type() {
   |     ^^^^^^^^^^^^


error: aborting due to 2 previous errors; 5 warnings emitted
------------------------------------------


---- [incremental] src/test/incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(unchanged_fn)` should be clean but is not
  --> /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:34:1
LL | pub fn unchanged_fn() {
   | ^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [incremental] src/test/incremental/issue-42602.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(bar)` should be clean but is not
   |
LL |     pub fn bar() {
   |     ^^^^^^^^^^^^


error: aborting due to previous error
------------------------------------------


---- [incremental] src/test/incremental/source_loc_macros.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/source_loc_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/source_loc_macros.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(line_same)` should be clean but is not
   |
LL | fn line_same() {
   | ^^^^^^^^^^^^^^


error: `hir_owner_nodes(col_same)` should be clean but is not
   |
   |
LL | fn col_same() {


error: `hir_owner_nodes(file_same)` should be clean but is not
   |
LL | fn file_same() {
   | ^^^^^^^^^^^^^^


error: aborting due to 3 previous errors
------------------------------------------


---- [incremental] src/test/incremental/span_hash_stable/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/span_hash_stable/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/a" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(SomeType)` should be clean but is not
  --> /checkout/src/test/incremental/span_hash_stable/auxiliary/sub1.rs:2:1
LL | / pub struct SomeType {
LL | |     pub x: u32,
LL | |     pub x: u32,
LL | |     pub y: i64,
LL | | }


error: `hir_owner_nodes(SomeOtherType)` should be clean but is not
  --> /checkout/src/test/incremental/span_hash_stable/auxiliary/sub2.rs:2:1
   |
LL | / pub struct SomeOtherType {
LL | |     pub a: i32,
LL | |     pub b: u64,
LL | | }

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] src/test/incremental/string_constant.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/string_constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/string_constant.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner_nodes(y)` should be clean but is not
   |
   |
LL |     pub fn y() {


error: `hir_owner_nodes(z)` should be clean but is not
   |
   |
LL |     pub fn z() {

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] src/test/incremental/struct_add_field.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_add_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/struct_add_field.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X(x: X) -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: `hir_owner_nodes(use_X)` should be clean but is not
   |
   |
LL | pub fn use_X(x: X) -> u32 {


error: `hir_owner_nodes(use_EmbedX)` should be clean but is not
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {


error: `hir_owner_nodes(use_Y)` should be clean but is not
   |
   |
LL | pub fn use_Y() {

error: aborting due to 3 previous errors; 4 warnings emitted
------------------------------------------



---- [incremental] src/test/incremental/struct_change_nothing.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_nothing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/struct_change_nothing.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |                   ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> /checkout/src/test/incremental/struct_change_nothing.rs:41:9
  --> /checkout/src/test/incremental/struct_change_nothing.rs:41:9
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: field is never read: `x`
  --> /checkout/src/test/incremental/struct_change_nothing.rs:20:5
   |
LL |     x: X
LL |     x: X
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: `hir_owner_nodes(use_X)` should be clean but is not
   |
   |
LL | pub fn use_X() -> u32 {


error: `hir_owner_nodes(use_EmbedX)` should be clean but is not
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {


error: `hir_owner_nodes(use_Y)` should be clean but is not
   |
   |
LL | pub fn use_Y() {

error: aborting due to 3 previous errors; 6 warnings emitted
------------------------------------------



---- [incremental] src/test/incremental/struct_change_field_type.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/struct_change_field_type.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |                   ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> /checkout/src/test/incremental/struct_change_field_type.rs:41:9
  --> /checkout/src/test/incremental/struct_change_field_type.rs:41:9
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: field is never read: `x`
  --> /checkout/src/test/incremental/struct_change_field_type.rs:20:5
   |
LL |     x: X
LL |     x: X
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: `hir_owner_nodes(use_X)` should be clean but is not
   |
   |
LL | pub fn use_X() -> u32 {


error: `hir_owner_nodes(use_EmbedX)` should be clean but is not
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {


error: `hir_owner_nodes(use_Y)` should be clean but is not
   |
   |
LL | pub fn use_Y() {

error: aborting due to 3 previous errors; 6 warnings emitted
------------------------------------------



---- [incremental] src/test/incremental/struct_remove_field.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_remove_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/struct_remove_field.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X(x: X) -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: `hir_owner_nodes(use_X)` should be clean but is not
   |
   |
LL | pub fn use_X(x: X) -> u32 {


error: `hir_owner_nodes(use_EmbedX)` should be clean but is not
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {


error: `hir_owner_nodes(use_Y)` should be clean but is not
   |
   |
LL | pub fn use_Y() {

error: aborting due to 3 previous errors; 4 warnings emitted
------------------------------------------

