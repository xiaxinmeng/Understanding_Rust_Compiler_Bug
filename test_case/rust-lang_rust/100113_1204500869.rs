plain
- help: consider restricting type parameter `T`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ help: consider further restricting type parameter `T`
93    |
- LL | fn duplicate_custom_1<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) where {
-    |                        ++++++++++++++
+ LL | fn duplicate_custom_1<T>(t: S<T>) -> (S<T>, S<T>) where, T: Trait, T: Copy {
96 
97 error[E0382]: use of moved value: `t`
98   --> $DIR/use_of_moved_value_copy_suggestions.rs:52:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/use_of_moved_value_copy_suggestions.stderr
diff of fixed:

39 trait B {}
40 
41 // Test where bounds are added with different bound placements
- fn duplicate_custom_1<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) where {
+ fn duplicate_custom_1<T>(t: S<T>) -> (S<T>, S<T>) where, T: Trait, T: Copy {
43     //~^ HELP consider restricting type parameter `T`
44     (t, t) //~ use of moved value: `t`


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/use_of_moved_value_copy_suggestions.fixed
To only update this specific test, also pass `--test-args moves/use_of_moved_value_copy_suggestions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `t`
   |
   |
LL | fn duplicate_t<T>(t: T) -> (T, T) {
   |                   - move occurs because `t` has type `T`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameter `T`
   |
   |
LL | fn duplicate_t<T: Copy>(t: T) -> (T, T) {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:11:9
   |
   |
LL | fn duplicate_opt<T>(t: Option<T>) -> (Option<T>, Option<T>) {
   |                     - move occurs because `t` has type `Option<T>`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameter `T`
   |
   |
LL | fn duplicate_opt<T: Copy>(t: Option<T>) -> (Option<T>, Option<T>) {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:16:9
   |
   |
LL | fn duplicate_tup1<T>(t: (T,)) -> ((T,), (T,)) {
   |                      - move occurs because `t` has type `(T,)`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameter `T`
   |
   |
LL | fn duplicate_tup1<T: Copy>(t: (T,)) -> ((T,), (T,)) {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:21:9
   |
   |
LL | fn duplicate_tup2<A, B>(t: (A, B)) -> ((A, B), (A, B)) {
   |                         - move occurs because `t` has type `(A, B)`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameters
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameters
   |
   |
LL | fn duplicate_tup2<A: Copy, B: Copy>(t: (A, B)) -> ((A, B), (A, B)) {
   |                    ++++++   ++++++
error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:26:9
   |
   |
LL | fn duplicate_custom<T>(t: S<T>) -> (S<T>, S<T>) {
   |                        - move occurs because `t` has type `S<T>`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameter `T`
   |
   |
LL | fn duplicate_custom<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:44:9
   |
   |
LL | fn duplicate_custom_1<T>(t: S<T>) -> (S<T>, S<T>) where {
   |                          - move occurs because `t` has type `S<T>`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider further restricting type parameter `T`
   |
   |
LL | fn duplicate_custom_1<T>(t: S<T>) -> (S<T>, S<T>) where, T: Trait, T: Copy {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:52:9
   |
   |
LL | fn duplicate_custom_2<T>(t: S<T>) -> (S<T>, S<T>)
   |                          - move occurs because `t` has type `S<T>`, which does not implement the `Copy` trait
...
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider further restricting this bound
   |
   |
LL |     T: A + Copy + Trait,

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:61:9
   |
   |
LL | fn duplicate_custom_3<T>(t: S<T>) -> (S<T>, S<T>)
   |                          - move occurs because `t` has type `S<T>`, which does not implement the `Copy` trait
...
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider further restricting this bound
   |
   |
LL |     T: A + Copy + Trait,

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:69:9
   |
   |
LL | fn duplicate_custom_4<T: A>(t: S<T>) -> (S<T>, S<T>)
   |                             - move occurs because `t` has type `S<T>`, which does not implement the `Copy` trait
...
LL |     (t, t) //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider further restricting this bound
   |
   |
LL | fn duplicate_custom_4<T: A + Copy + Trait>(t: S<T>) -> (S<T>, S<T>)

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:75:9
   |
   |
LL | fn existing_colon<T:>(t: T) {
   |                       - move occurs because `t` has type `T`, which does not implement the `Copy` trait
LL |     //~^ HELP consider restricting type parameter `T`
LL |     [t, t]; //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider restricting type parameter `T`
   |
   |
LL | fn existing_colon<T: Copy>(t: T) {

error[E0382]: use of moved value: `t`
  --> /checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs:83:9
   |
   |
LL | fn existing_colon_in_where<T>(t: T)
   |                               - move occurs because `t` has type `T`, which does not implement the `Copy` trait
...
LL |     [t, t]; //~ use of moved value: `t`
   |      -  ^ value used here after move
   |      value moved here
   |
help: consider further restricting type parameter `T`
   |
   |
LL |     T:, T: Copy

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] src/test/ui/partialeq_help.rs stdout ----
diff of stderr:

19    = help: the trait `PartialEq<T>` is not implemented for `&T`
20 help: consider extending the `where` clause, but there might be an alternative better way to express this requirement
21    |
- LL | fn foo2<T: PartialEq>(a: &T, b: T) where &T: PartialEq<T> {
-    |                                          ++++++++++++++++
+ LL | fn foo2<T: PartialEq>(a: &T, b: T) where, &T: PartialEq<T> {
24 
25 error: aborting due to 2 previous errors
26 

---
To only update this specific test, also pass `--test-args partialeq_help.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/partialeq_help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/partialeq_help" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/partialeq_help/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `&T` with `T`
   |
   |
LL |     a == b; //~ ERROR E0277
   |       ^^ no implementation for `&T == T`
   |
   = help: the trait `PartialEq<T>` is not implemented for `&T`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo<T: PartialEq>(a: &T, b: T) where &T: PartialEq<T> {


error[E0277]: can't compare `&T` with `T`
   |
   |
LL |     a == b; //~ ERROR E0277
   |       ^^ no implementation for `&T == T`
   |
   = help: the trait `PartialEq<T>` is not implemented for `&T`
help: consider extending the `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo2<T: PartialEq>(a: &T, b: T) where, &T: PartialEq<T> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

10 help: introduce a type parameter with a trait bound instead of using `impl Trait`
11    |
12 LL ~ pub fn print_values<I: IntoIterator>(values: &I)
- LL ~ where <I as IntoIterator>::Item: std::fmt::Display {
+ LL ~ where, <I as IntoIterator>::Item: std::fmt::Display {
15 
16 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760/issue-97760.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-97760.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-97760.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `<impl IntoIterator as IntoIterator>::Item` doesn't implement `std::fmt::Display`
   |
   |
LL |         println!("{x}");
   |                    ^ `<impl IntoIterator as IntoIterator>::Item` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `<impl IntoIterator as IntoIterator>::Item`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
   |
LL ~ pub fn print_values<I: IntoIterator>(values: &I)
LL ~ where, <I as IntoIterator>::Item: std::fmt::Display {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
