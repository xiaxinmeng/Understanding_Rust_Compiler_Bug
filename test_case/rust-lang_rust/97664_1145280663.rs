plain

75    |
76 help: consider restricting type parameter `T`
77    |
- LL | fn duplicate_custom<T: Trait + Copy>(t: S<T>) -> (S<T>, S<T>) {
+ LL | fn duplicate_custom<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) {
80 
81 error[E0382]: use of moved value: `t`

91    |
91    |
92 help: consider restricting type parameter `T`
93    |
- LL | fn duplicate_custom_1<T: Trait + Copy>(t: S<T>) -> (S<T>, S<T>) where {
+ LL | fn duplicate_custom_1<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) where {
96 
97 error[E0382]: use of moved value: `t`

107    |
107    |
108 help: consider further restricting this bound
109    |
- LL |     T: A + Trait + Copy,
+ LL |     T: A + Copy + Trait,
112 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
113 error[E0382]: use of moved value: `t`


123    |
124 help: consider further restricting this bound
125    |
- LL |     T: A + Trait + Copy,
+ LL |     T: A + Copy + Trait,
128 
129 error[E0382]: use of moved value: `t`

139    |
139    |
140 help: consider further restricting this bound
141    |
- LL | fn duplicate_custom_4<T: A + Trait + Copy>(t: S<T>) -> (S<T>, S<T>)
+ LL | fn duplicate_custom_4<T: A + Copy + Trait>(t: S<T>) -> (S<T>, S<T>)
144 
145 error[E0382]: use of moved value: `t`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/use_of_moved_value_copy_suggestions.stderr
diff of fixed:

21     (t, t) //~ use of moved value: `t`
23 
23 
- fn duplicate_custom<T: Trait + Copy>(t: S<T>) -> (S<T>, S<T>) {
+ fn duplicate_custom<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) {
25     //~^ HELP consider restricting type parameter `T`
26     (t, t) //~ use of moved value: `t`

39 trait B {}
40 
40 
41 // Test where bounds are added with different bound placements
- fn duplicate_custom_1<T: Trait + Copy>(t: S<T>) -> (S<T>, S<T>) where {
+ fn duplicate_custom_1<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) where {
43     //~^ HELP consider restricting type parameter `T`
44     (t, t) //~ use of moved value: `t`

46 
46 
47 fn duplicate_custom_2<T>(t: S<T>) -> (S<T>, S<T>)
48 where
-     T: A + Trait + Copy,
+     T: A + Copy + Trait,
50     //~^ HELP consider further restricting this bound
51 {
52     (t, t) //~ use of moved value: `t`
54 
54 
55 fn duplicate_custom_3<T>(t: S<T>) -> (S<T>, S<T>)
56 where
-     T: A + Trait + Copy,
+     T: A + Copy + Trait,
58     //~^ HELP consider further restricting this bound
59     T: B,


61     (t, t) //~ use of moved value: `t`
63 
63 
- fn duplicate_custom_4<T: A + Trait + Copy>(t: S<T>) -> (S<T>, S<T>)
+ fn duplicate_custom_4<T: A + Copy + Trait>(t: S<T>) -> (S<T>, S<T>)
65 //~^ HELP consider further restricting this bound
67     T: B,


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/use_of_moved_value_copy_suggestions.fixed
To only update this specific test, also pass `--test-args moves/use_of_moved_value_copy_suggestions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/use_of_moved_value_copy_suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/use_of_moved_value_copy_suggestions/auxiliary"
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
help: consider restricting type parameter `T`
   |
   |
LL | fn duplicate_custom_1<T: Copy + Trait>(t: S<T>) -> (S<T>, S<T>) where {

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

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0382`.
