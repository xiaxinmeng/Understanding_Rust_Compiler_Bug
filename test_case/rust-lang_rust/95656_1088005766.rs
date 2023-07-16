plain
 finished in 0.525 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 153 tests
...........................................FF...F...F............................................... 100/153
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] incremental/hashes/if_expressions.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(change_condition)` should be clean but is not
   |
   |
LL | / pub fn change_condition(x: bool) -> u32 {
LL | |     if !x {
LL | |         return 1
LL | |     }
LL | |
LL | |     return 0
LL | | }


error: `hir_owner(add_else_branch)` should be clean but is not
   |
   |
LL | / pub fn add_else_branch(x: bool) -> u32 {
LL | |     let mut ret = 1;
LL | |
LL | |     if x {
LL | |     ret
LL | | }
   | |_^


error: `hir_owner(change_then_branch_if_let)` should be clean but is not
   |
   |
LL | / pub fn change_then_branch_if_let(x: Option<u32>) -> u32 {
LL | |     if let Some(x) = x {
LL | |         return x + 1
LL | |     }
LL | |
LL | |     0
LL | | }


error: `hir_owner(add_else_branch_if_let)` should be clean but is not
   |
   |
LL | / pub fn add_else_branch_if_let(x: Option<u32>) -> u32 {
LL | |     let mut ret = 1;
LL | |
LL | |     if let Some(x) = x {
LL | |     ret
LL | | }
   | |_^


error: aborting due to 4 previous errors
------------------------------------------


---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(add_lower_bound)` should be clean but is not
   |
   |
LL | / fn add_lower_bound(slice: &[u32]) -> &[u32] {
LL | |     &slice[3..4]
LL | | }


error: `hir_owner(add_upper_bound)` should be clean but is not
   |
   |
LL | / fn add_upper_bound(slice: &[u32]) -> &[u32] {
LL | |     &slice[3..7]
LL | | }


error: `hir_owner(exclusive_to_inclusive_range)` should be clean but is not
   |
   |
LL | / fn exclusive_to_inclusive_range(slice: &[u32]) -> &[u32] {
LL | |     &slice[3..=7]
LL | | }

error: aborting due to 3 previous errors
------------------------------------------



---- [incremental] incremental/hashes/match_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(add_arm)` should be clean but is not
   |
   |
LL | / pub fn add_arm(x: u32) -> u32 {
LL | |     match x {
LL | |         0 => 0,
LL | |         1 => 1,
LL | |     }
LL | | }
   | |_^


error: `hir_owner(add_guard_clause)` should be clean but is not
   |
   |
LL | / pub fn add_guard_clause(x: u32, y: bool) -> u32 {
LL | |     match x {
LL | |         0 => 0,
LL | |         1 if y => 1,
LL | |         _ => 100,
LL | |     }
LL | | }


error: `hir_owner(change_guard_clause)` should be clean but is not
   |
   |
LL | / pub fn change_guard_clause(x: u32, y: bool) -> u32 {
LL | |     match x {
LL | |         0 => 0,
LL | |         1 if !y => 1,
LL | |         _ => 100,
LL | |     }
LL | | }


error: `hir_owner(add_at_binding)` should be clean but is not
   |
   |
LL | / pub fn add_at_binding(x: u32) -> u32 {
LL | |     match x {
LL | |         0 => 0,
LL | |         1 => 1,
LL | |         x @ _ => x,
LL | |     }
LL | | }


error: `hir_owner(change_simple_name_to_pattern)` should be clean but is not
   |
   |
LL | / pub fn change_simple_name_to_pattern(x: u32) -> u32 {
LL | |     match (x, x & 1) {
LL | |         (0, 0) => 0,
LL | |         (x, y) => 1,
LL | |     }
LL | | }


error: `hir_owner(add_amp_to_binding_in_pattern)` should be clean but is not
   |
   |
LL | / pub fn add_amp_to_binding_in_pattern(x: u32) -> u32 {
LL | |     match (&x, x & 1) {
LL | |         (&a, 0) => 0,
LL | |         _ => 1,
LL | |     }
LL | | }


error: `hir_owner(add_alternative_to_arm)` should be clean but is not
   |
   |
LL | / pub fn add_alternative_to_arm(x: u32) -> u32 {
LL | |     match x {
LL | |         0 | 7 => 0,
LL | |         1 => 3,
LL | |         _ => 2,
LL | |     }
LL | | }

error: aborting due to 7 previous errors
------------------------------------------



---- [incremental] incremental/hashes/struct_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/struct_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(add_field_regular_struct)` should be clean but is not
   |
   |
LL | / pub fn add_field_regular_struct() -> RegularStruct {
LL | |     let struct1 = RegularStruct {
LL | |         x: 3,
LL | |         y: 4,
LL | |     }
LL | | }
   | |_^

