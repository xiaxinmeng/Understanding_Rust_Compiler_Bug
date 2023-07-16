plain
.................................................................................................... 10100/11424
.................................................................................................... 10200/11424
.................................................................................................... 10300/11424
.............i...................................................................................... 10400/11424
.................F.F...F..................2021-02-08T22:34:56.552550Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/bound/auxiliary/trait_bounds_on_structs_and_enums_xc.rs` source not found"
F2021-02-08T22:34:56.552887Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/bound/auxiliary/trait_bounds_on_structs_and_enums_xc.rs` source not found"
FF.2021-02-08T22:34:56.567963Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/bound/auxiliary/crate_a1.rs` source not found"
F....FFFF.FF.FF.F.........2021-02-08T22:34:56.979539Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/default-method/auxiliary/trait_default_method_xc_aux.rs` source not found"
F2021-02-08T22:34:56.980924Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/default-method/auxiliary/trait_default_method_xc_aux.rs` source not found"
F..F.......F.FFFF.......F.2021-02-08T22:34:57.257656Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_auto_xc_aux.rs` source not found"
2021-02-08T22:34:57.257936Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_auto_xc_2_aux.rs` source not found"
F 10500/11424
FFF...2021-02-08T22:34:57.344928Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_xc_call_aux.rs` source not found"
FFFF........2021-02-08T22:34:57.520731Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_overloading_xc.rs` source not found"
F..............................F....F....F.F.F..F....F...........F.........FF.FFF. 10600/11424
.F.F.......F..FFF....F...F..F.FFFF.FFF.F.FF......................................................... 10700/11424
.................................................................................................... 10900/11424
.................................................................................................... 11000/11424
.................................................................................................... 11100/11424
.................................................................................................... 11200/11424
.................................................................................................... 11200/11424
.................................................................................................... 11300/11424
..............i.i................................................................................... 11400/11424
........................
failures:

---- [ui] ui/traits/alias/trait-alias-import-cross-crate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/trait-alias-import-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/trait-alias-import-cross-crate/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/trait-alias-import-cross-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved imports `trait_alias::Greet`, `trait_alias::Hi`
   |
   |
LL | use trait_alias::{Greet, Hi};
   |                   ^^^^^  ^^ no `Hi` in the root
   |                   |
   |                   no `Greet` in the root
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.


------------------------------------------


---- [ui] ui/traits/as-struct-constructor.rs stdout ----


1 error[E0574]: expected struct, variant or union type, found trait `TraitNotAStruct`
-   --> $DIR/trait-as-struct-constructor.rs:4:5
+   --> $DIR/as-struct-constructor.rs:4:5
3    |
4 LL |     TraitNotAStruct{ value: 0 };
5    |     ^^^^^^^^^^^^^^^ not a struct, variant or union type

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/as-struct-constructor/as-struct-constructor.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/as-struct-constructor/as-struct-constructor.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/as-struct-constructor.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/as-struct-constructor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/as-struct-constructor" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/as-struct-constructor/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0574]: expected struct, variant or union type, found trait `TraitNotAStruct`
  --> /checkout/src/test/ui/traits/as-struct-constructor.rs:4:5
   |
LL |     TraitNotAStruct{ value: 0 };
   |     ^^^^^^^^^^^^^^^ not a struct, variant or union type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0574`.


------------------------------------------


---- [ui] ui/traits/assoc-type-in-superbad.rs stdout ----


1 error[E0271]: type mismatch resolving `<std::vec::IntoIter<i32> as Iterator>::Item == u32`
-   --> $DIR/traits-assoc-type-in-supertrait-bad.rs:12:16
+   --> $DIR/assoc-type-in-superbad.rs:12:16
4 LL |     type Key = u32;
4 LL |     type Key = u32;
5    |                ^^^ expected `i32`, found `u32`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad/assoc-type-in-superbad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/assoc-type-in-superbad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/assoc-type-in-superbad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<std::vec::IntoIter<i32> as Iterator>::Item == u32`
  --> /checkout/src/test/ui/traits/assoc-type-in-superbad.rs:12:16
   |
LL |     type Key = u32; //~ ERROR type mismatch
   |                ^^^ expected `i32`, found `u32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/traits/bound/on-structs-and-enums-xc.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/bound/auxiliary/trait_bounds_on_structs_and_enums_xc.rs` source not found
thread '[ui] ui/traits/bound/on-structs-and-enums-xc.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/bound/on-structs-and-enums-xc1.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/bound/auxiliary/trait_bounds_on_structs_and_enums_xc.rs` source not found
thread '[ui] ui/traits/bound/on-structs-and-enums-xc1.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/bound/not-on-struct.rs stdout ----


1 error[E0226]: only a single explicit lifetime bound is permitted
-   --> $DIR/trait-bounds-not-on-struct.rs:25:25
3    |
3    |
4 LL | fn e() -> 'static + A + 'static {

6 
6 
7 error[E0226]: only a single explicit lifetime bound is permitted
-   --> $DIR/trait-bounds-not-on-struct.rs:29:53
9    |
9    |
10 LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) {

12 
13 error[E0404]: expected trait, found struct `Foo`
-   --> $DIR/trait-bounds-not-on-struct.rs:8:16
-   --> $DIR/trait-bounds-not-on-struct.rs:8:16
+   --> $DIR/not-on-struct.rs:8:16
15    |
16 LL | fn foo(_x: Box<Foo + Send>) { }
17    |                ^^^ not a trait
18    |
18    |
19 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:8:22
21    |
21    |
22 LL | fn foo(_x: Box<Foo + Send>) { }
23    |                ---   ^^^^ ...because of this bound
25    |                expected this type to be a trait...
26 
26 
27 error[E0404]: expected trait, found struct `Vec`
-   --> $DIR/trait-bounds-not-on-struct.rs:10:29
29    |
29    |
30 LL | type TypeAlias<T> = Box<dyn Vec<T>>;
31    |                             ^^^^^^ not a trait
32 
33 error[E0404]: expected trait, found struct `A`
-   --> $DIR/trait-bounds-not-on-struct.rs:13:11
+   --> $DIR/not-on-struct.rs:13:11
+   --> $DIR/not-on-struct.rs:13:11
35    |
36 LL | fn a() -> A + 'static {
37    |           ^ not a trait
38    |
38    |
39 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:13:15
41    |
41    |
42 LL | fn a() -> A + 'static {
43    |           -   ^^^^^^^ ...because of this bound
49    |           --
50 
51 error[E0404]: expected trait, found enum `Result`
-   --> $DIR/trait-bounds-not-on-struct.rs:16:34
-   --> $DIR/trait-bounds-not-on-struct.rs:16:34
+   --> $DIR/not-on-struct.rs:16:34
53    |
54 LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) {
55    |                                  ^^^^^^^^^^^ not a trait
56    |
56    |
57 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:16:48
59    |
59    |
60 LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) {
61    |                                  -----------   ^^ ...because of this bound
67    |                                            --
68 
69 error[E0404]: expected trait, found struct `A`
-   --> $DIR/trait-bounds-not-on-struct.rs:19:21
-   --> $DIR/trait-bounds-not-on-struct.rs:19:21
+   --> $DIR/not-on-struct.rs:19:21
71    |
72 LL | fn c() -> 'static + A {
73    |                     ^ not a trait
74    |
74    |
75 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:19:11
77    |
77    |
78 LL | fn c() -> 'static + A {
79    |           ^^^^^^^   - expected this type to be a trait...
85    |          --
86 
87 error[E0404]: expected trait, found enum `Result`
-   --> $DIR/trait-bounds-not-on-struct.rs:22:39
-   --> $DIR/trait-bounds-not-on-struct.rs:22:39
+   --> $DIR/not-on-struct.rs:22:39
89    |
90 LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) {
91    |                                       ^^^^^^^^^^^ not a trait
92    |
92    |
93 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:22:34
95    |
95    |
96 LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) {
97    |                                  ^^   ----------- expected this type to be a trait...
103    |                                 --
104 
105 error[E0404]: expected trait, found struct `A`
-   --> $DIR/trait-bounds-not-on-struct.rs:25:21
-   --> $DIR/trait-bounds-not-on-struct.rs:25:21
+   --> $DIR/not-on-struct.rs:25:21
107    |
108 LL | fn e() -> 'static + A + 'static {
109    |                     ^ not a trait
110    |
110    |
111 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:25:11
113    |
113    |
114 LL | fn e() -> 'static + A + 'static {
115    |           ^^^^^^^   -   ^^^^^^^ ...because of these bounds
121    |          ---
122 
123 error[E0404]: expected trait, found enum `Result`
-   --> $DIR/trait-bounds-not-on-struct.rs:29:39
-   --> $DIR/trait-bounds-not-on-struct.rs:29:39
+   --> $DIR/not-on-struct.rs:29:39
125    |
126 LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) {
127    |                                       ^^^^^^^^^^^ not a trait
128    |
128    |
129 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:29:34
131    |
131    |
132 LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) {
133    |                                  ^^   -----------   ^^ ...because of these bounds
139    |                                 --         --
140 
140 
141 error[E0404]: expected trait, found struct `Traitor`
-   --> $DIR/trait-bounds-not-on-struct.rs:35:11
143    |
143    |
144 LL | trait Trait {}
145    | ----------- similarly named trait `Trait` defined here
147    |           ^^^^^^^ not a trait
148    |
148    |
149 help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
-   --> $DIR/trait-bounds-not-on-struct.rs:35:21
151    |
151    |
152 LL | fn g() -> Traitor + 'static {
153    |           -------   ^^^^^^^ ...because of this bound

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct/not-on-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/bound/not-on-struct.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/bound/not-on-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0226]: only a single explicit lifetime bound is permitted
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:25:25
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found


error[E0226]: only a single explicit lifetime bound is permitted
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:29:53
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `Foo`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:8:16
   |
   |
LL | fn foo(_x: Box<Foo + Send>) { } //~ ERROR expected trait, found struct `Foo`
   |                ^^^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:8:22
   |
LL | fn foo(_x: Box<Foo + Send>) { } //~ ERROR expected trait, found struct `Foo`
   |                ---   ^^^^ ...because of this bound
   |                expected this type to be a trait...


error[E0404]: expected trait, found struct `Vec`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:10:29
   |
LL | type TypeAlias<T> = Box<dyn Vec<T>>; //~ ERROR expected trait, found struct `Vec`
   |                             ^^^^^^ not a trait
error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:13:11
   |
   |
LL | fn a() -> A + 'static { //~ ERROR expected trait, found
   |           ^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:13:15
   |
LL | fn a() -> A + 'static { //~ ERROR expected trait, found
   |           -   ^^^^^^^ ...because of this bound
   |           expected this type to be a trait...
   |           expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn a() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:16:34
   |
   |
LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                  ^^^^^^^^^^^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:16:48
   |
LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                  -----------   ^^ ...because of this bound
   |                                  expected this type to be a trait...
   |                                  expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:19:21
   |
   |
LL | fn c() -> 'static + A { //~ ERROR expected trait, found
   |                     ^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:19:11
   |
LL | fn c() -> 'static + A { //~ ERROR expected trait, found
   |           ^^^^^^^   - expected this type to be a trait...
   |           |
   |           ...because of this bound
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn c() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:22:39
   |
   |
LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) { //~ ERROR expected trait, found
   |                                       ^^^^^^^^^^^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:22:34
   |
LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) { //~ ERROR expected trait, found
   |                                  ^^   ----------- expected this type to be a trait...
   |                                  |
   |                                  ...because of this bound
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn d<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:25:21
   |
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found
   |                     ^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:25:11
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found
   |           ^^^^^^^   -   ^^^^^^^ ...because of these bounds
   |                     expected this type to be a trait...
   |                     expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn e() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:29:39
   |
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                       ^^^^^^^^^^^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:29:34
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                  ^^   -----------   ^^ ...because of these bounds
   |                                       expected this type to be a trait...
   |                                       expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn f<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found


error[E0404]: expected trait, found struct `Traitor`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:35:11
   |
LL | trait Trait {}
   | ----------- similarly named trait `Trait` defined here
LL | fn g() -> Traitor + 'static { //~ ERROR expected trait, found struct `Traitor`
   |           ^^^^^^^ not a trait
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:35:21
   |
LL | fn g() -> Traitor + 'static { //~ ERROR expected trait, found struct `Traitor`
   |           -------   ^^^^^^^ ...because of this bound
   |           expected this type to be a trait...
   |           expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
LL | fn g() -> Traitor { //~ ERROR expected trait, found struct `Traitor`
help: a trait with a similar name exists
   |
   |
LL | fn g() -> Trait + 'static { //~ ERROR expected trait, found struct `Traitor`

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0226, E0404.
Some errors have detailed explanations: E0226, E0404.
For more information about an error, try `rustc --explain E0226`.

------------------------------------------


---- [ui] ui/traits/bound/same-crate-name.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/bound/auxiliary/crate_a1.rs` source not found
thread '[ui] ui/traits/bound/same-crate-name.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/bound/not-on-bare-trait.rs stdout ----

1 warning: trait objects without an explicit `dyn` are deprecated
-   --> $DIR/trait-bounds-not-on-bare-trait.rs:7:12
+   --> $DIR/not-on-bare-trait.rs:7:12
+   --> $DIR/not-on-bare-trait.rs:7:12
3    |
4 LL | fn foo(_x: Foo + Send) {
---
1 error[E0573]: expected type, found module `a`
-   --> $DIR/trait-impl-for-module.rs:7:12
+   --> $DIR/impl-for-module.rs:7:12
3    |
4 LL | trait A {
5    | ------- similarly named trait `A` defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-for-module/impl-for-module.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-for-module/impl-for-module.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-for-module.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-for-module.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-for-module" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-for-module/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0573]: expected type, found module `a`
  --> /checkout/src/test/ui/traits/impl-for-module.rs:7:12
   |
LL | trait A {
   | ------- similarly named trait `A` defined here
...
LL | impl A for a { //~ ERROR expected type, found module
   |            ^ help: a trait with a similar name exists: `A`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0573`.


------------------------------------------


---- [ui] ui/traits/impl-different-num-params.rs stdout ----
diff of stderr:

1 error[E0050]: method `bar` has 1 parameter but the declaration in trait `Foo::bar` has 2
-   --> $DIR/trait-impl-different-num-params.rs:5:12
3    |
3    |
4 LL |     fn bar(&self, x: usize) -> Self;
5    |            --------------- trait requires 2 parameters

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-different-num-params/impl-different-num-params.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-different-num-params/impl-different-num-params.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-different-num-params.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-different-num-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-different-num-params" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-different-num-params/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0050]: method `bar` has 1 parameter but the declaration in trait `Foo::bar` has 2
   |
   |
LL |     fn bar(&self, x: usize) -> Self;
   |            --------------- trait requires 2 parameters
LL |     fn bar(&self) -> isize {
   |            ^^^^^ expected 2 parameters, found 1

error: aborting due to previous error
---

---- [ui] ui/traits/impl-method-mismatch.rs stdout ----
diff of stderr:

1 error[E0053]: method `jumbo` has an incompatible type for trait
-   --> $DIR/trait-impl-method-mismatch.rs:7:5
+   --> $DIR/impl-method-mismatch.rs:7:5
3    |
4 LL |     fn jumbo(&self, x: &usize) -> usize;
5    |     ------------------------------------ type in trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-method-mismatch/impl-method-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-method-mismatch/impl-method-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-method-mismatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-method-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-method-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-method-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0053]: method `jumbo` has an incompatible type for trait
  --> /checkout/src/test/ui/traits/impl-method-mismatch.rs:7:5
   |
LL |     fn jumbo(&self, x: &usize) -> usize;
   |     ------------------------------------ type in trait
...
LL |     unsafe fn jumbo(&self, x: &usize) { *self + *x; }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected normal fn, found unsafe fn
   |
   = note: expected fn pointer `fn(&usize, &usize) -> usize`
              found fn pointer `unsafe fn(&usize, &usize)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.


------------------------------------------


---- [ui] ui/traits/impl-1.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `foo` found for reference `&i32` in the current scope
-   --> $DIR/trait-impl-1.rs:15:7
3    |
3    |
4 LL |     x.foo();
5    |       ^^^ method not found in `&i32`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-1/impl-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `foo` found for reference `&i32` in the current scope
   |
   |
LL |     x.foo(); //~ERROR: no method named `foo` found
   |       ^^^ method not found in `&i32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs stdout ----
diff of stderr:

1 error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
-   --> $DIR/trait-impl-of-supertrait-has-wrong-lifetime-parameters.rs:24:13
3    |
3    |
4 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {

6    |
6    |
7 note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 24:6...
-   --> $DIR/trait-impl-of-supertrait-has-wrong-lifetime-parameters.rs:24:6
9    |
9    |
10 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


12 note: ...but the lifetime must also be valid for the lifetime `'b` as defined on the impl at 24:9...
-   --> $DIR/trait-impl-of-supertrait-has-wrong-lifetime-parameters.rs:24:9
14    |
14    |
15 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


17 note: ...so that the types are compatible
-   --> $DIR/trait-impl-of-supertrait-has-wrong-lifetime-parameters.rs:24:13
19    |
19    |
20 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/impl-of-supertrait-has-wrong-lifetime-parameters.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/impl-of-supertrait-has-wrong-lifetime-parameters.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 24:6...
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |      ^^
note: ...but the lifetime must also be valid for the lifetime `'b` as defined on the impl at 24:9...
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |         ^^
note: ...so that the types are compatible
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |             ^^^^^^^^^^
   = note: expected `T1<'a>`
              found `T1<'_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/traits/inheritance/auto-xc.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_auto_xc_aux.rs` source not found
thread '[ui] ui/traits/inheritance/auto-xc.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/inheritance/auto-xc-2.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_auto_xc_2_aux.rs` source not found
thread '[ui] ui/traits/inheritance/auto-xc-2.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/inductive-overflow/lifetime.rs stdout ----


1 error[E0275]: overflow evaluating the requirement `Box<X<C<'_>>>: NotAuto`
-   --> $DIR/traits-inductive-overflow-lifetime.rs:27:5
3    |
3    |
4 LL | fn is_send<S: NotAuto>() {}
5    |               ------- required by this bound in `is_send`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/lifetime.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/lifetime.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Box<X<C<'_>>>: NotAuto`
  --> /checkout/src/test/ui/traits/inductive-overflow/lifetime.rs:27:5
   |
LL | fn is_send<S: NotAuto>() {}
   |               ------- required by this bound in `is_send`
...
LL |     is_send::<X<C<'static>>>();
   |
   |
   = note: required because of the requirements on the impl of `NotAuto` for `X<C<'static>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/traits/inductive-overflow/simultaneous.rs stdout ----


1 error[E0275]: overflow evaluating the requirement `{integer}: Tweedledum`
-   --> $DIR/traits-inductive-overflow-simultaneous.rs:18:5
+   --> $DIR/simultaneous.rs:18:5
3    |
4 LL | fn is_ee<T: Combo>(t: T) {
5    |             ----- required by this bound in `is_ee`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous/simultaneous.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/simultaneous.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `{integer}: Tweedledum`
  --> /checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs:18:5
   |
LL | fn is_ee<T: Combo>(t: T) {
   |             ----- required by this bound in `is_ee`
LL |     is_ee(4);
   |     ^^^^^
   |
   |
   = note: required because of the requirements on the impl of `Combo` for `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/traits/inheritance/cross-trait-call-xc.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_xc_call_aux.rs` source not found
thread '[ui] ui/traits/inheritance/cross-trait-call-xc.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/traits/inductive-overflow/supertrait.rs stdout ----


1 error[E0275]: overflow evaluating the requirement `NoClone: Magic`
-   --> $DIR/traits-inductive-overflow-supertrait.rs:13:18
3    |
3    |
4 LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
5    |            ----- required by this bound in `copy`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait/supertrait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait/supertrait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/supertrait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `NoClone: Magic`
  --> /checkout/src/test/ui/traits/inductive-overflow/supertrait.rs:13:18
   |
LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
   |            ----- required by this bound in `copy`
...
LL |     let (a, b) = copy(NoClone); //~ ERROR E0275
   |
   |
   = note: required because of the requirements on the impl of `Magic` for `NoClone`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/traits/inductive-overflow/superauto-trait.rs stdout ----

1 error[E0568]: auto traits cannot have super traits
-   --> $DIR/traits-inductive-overflow-supertrait-auto-trait.rs:8:19
+   --> $DIR/superauto-trait.rs:8:19
+   --> $DIR/superauto-trait.rs:8:19
3    |
4 LL | auto trait Magic: Copy {}
5    |            -----  ^^^^ help: remove the super traits
7    |            auto trait cannot have super traits
8 
8 
9 error[E0277]: the trait bound `NoClone: Copy` is not satisfied
-   --> $DIR/traits-inductive-overflow-supertrait-auto-trait.rs:16:23
+   --> $DIR/superauto-trait.rs:16:23
11    |
12 LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
13    |            ----- required by this bound in `copy`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait/superauto-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/superauto-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0568]: auto traits cannot have super traits
  --> /checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs:8:19
   |
LL | auto trait Magic: Copy {} //~ ERROR E0568
   |            -----  ^^^^ help: remove the super traits
   |            auto trait cannot have super traits


error[E0277]: the trait bound `NoClone: Copy` is not satisfied
  --> /checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs:16:23
   |
LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
   |            ----- required by this bound in `copy`
...
LL |     let (a, b) = copy(NoClone); //~ ERROR
   |                       ^^^^^^^ the trait `Copy` is not implemented for `NoClone`
   |
   = note: required because of the requirements on the impl of `Magic` for `NoClone`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0568.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/traits/inductive-overflow/two-traits.rs stdout ----


1 error[E0277]: `T` cannot be shared between threads safely
-   --> $DIR/traits-inductive-overflow-two-traits.rs:11:5
3    |
3    |
4 LL |     type X: Trait;
5    |             ----- required by this bound in `Magic::X`
13    |               ^^^^^^
14 
14 
15 error[E0275]: overflow evaluating the requirement `*mut (): Magic`
-   --> $DIR/traits-inductive-overflow-two-traits.rs:20:5
17    |
17    |
18 LL | fn wizard<T: Magic>() { check::<<T as Magic>::X>(); }
19    |              ----- required by this bound in `wizard`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/two-traits.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `T` cannot be shared between threads safely
  --> /checkout/src/test/ui/traits/inductive-overflow/two-traits.rs:11:5
   |
LL |     type X: Trait;
   |             ----- required by this bound in `Magic::X`
LL |     type X = Self;
LL |     type X = Self;
   |     ^^^^^^^^^^^^^^ `T` cannot be shared between threads safely
help: consider further restricting this bound
   |
   |
LL | impl<T: Magic + Sync> Magic for T {


error[E0275]: overflow evaluating the requirement `*mut (): Magic`
  --> /checkout/src/test/ui/traits/inductive-overflow/two-traits.rs:20:5
   |
LL | fn wizard<T: Magic>() { check::<<T as Magic>::X>(); }
   |              ----- required by this bound in `wizard`
...
LL |     wizard::<*mut ()>(); //~ ERROR E0275

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0275, E0277.
Some errors have detailed explanations: E0275, E0277.
For more information about an error, try `rustc --explain E0275`.

------------------------------------------


---- [ui] ui/traits/inheritance/overloading-xc-exe.rs stdout ----

error: aux-build `/checkout/src/test/ui/traits/inheritance/auxiliary/trait_inheritance_overloading_xc.rs` source not found
thread '[ui] ui/traits/inheritance/overloading-xc-exe.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9
---- [ui] ui/traits/issue-71136.rs stdout ----
diff of stderr:


1 error[E0277]: the trait bound `Foo: Clone` is not satisfied
-   --> $DIR/traits-issue-71136.rs:5:5
3    |
3    |
4 LL |     the_foos: Vec<Foo>,
5    |     ^^^^^^^^^^^^^^^^^^ expected an implementor of trait `Clone`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136/issue-71136.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136/issue-71136.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-71136.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-71136.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-71136/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Foo: Clone` is not satisfied
   |
   |
LL |     the_foos: Vec<Foo>, //~ERROR Clone
   |     ^^^^^^^^^^^^^^^^^^ expected an implementor of trait `Clone`
   |
   = note: required because of the requirements on the impl of `Clone` for `Vec<Foo>`
   = note: required by `clone`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/issue-65284-suggest-generic-bound.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `foo` found for type parameter `T` in the current scope
-   --> $DIR/issue-65284-suggest-generic-trait-bound.rs:8:7
3    |
3    |
4 LL |     t.foo()
5    |       ^^^ method not found in `T`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/issue-65284-suggest-generic-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/issue-65284-suggest-generic-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-65284-suggest-generic-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-65284-suggest-generic-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/auxiliary"
------------------------------------------

------------------------------------------
---
1 error[E0308]: mismatched types
-   --> $DIR/traits-multidispatch-bad.rs:19:17
+   --> $DIR/multidispatch-bad.rs:19:17
3    |
4 LL |     test(22i32, 44i32);
5    |                 ^^^^^ expected `u32`, found `i32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-bad/multidispatch-bad.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-bad/multidispatch-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/multidispatch-bad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/multidispatch-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/traits/multidispatch-bad.rs:19:17
   |
LL |     test(22i32, 44i32); //~ ERROR mismatched types
   |                 ^^^^^ expected `u32`, found `i32`
   |
help: change the type of the numeric literal from `i32` to `u32`
   |
LL |     test(22i32, 44u32); //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
1 error[E0282]: type annotations needed
-   --> $DIR/traits-multidispatch-convert-ambig-dest.rs:26:5
+   --> $DIR/multidispatch-convert-ambig-dest.rs:26:5
3    |
4 LL |     test(22, std::default::Default::default());
5    |     ^^^^ cannot infer type for type parameter `U` declared on the function `test`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest/multidispatch-convert-ambig-dest.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest/multidispatch-convert-ambig-dest.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/multidispatch-convert-ambig-dest.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26:5
   |
LL |     test(22, std::default::Default::default());
   |     ^^^^ cannot infer type for type parameter `U` declared on the function `test`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.

---
1 error[E0308]: method not compatible with trait
-   --> $DIR/trait-matching-lifetimes.rs:14:5
+   --> $DIR/matching-lifetimes.rs:14:5
3    |
4 LL |     fn foo(x: Foo<'b,'a>) {
5    |     ^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch

7    = note: expected fn pointer `fn(Foo<'a, 'b>)`
8               found fn pointer `fn(Foo<'b, 'a>)`
9 note: the lifetime `'b` as defined on the impl at 13:9...
-   --> $DIR/trait-matching-lifetimes.rs:13:9
11    |
11    |
12 LL | impl<'a,'b> Tr for Foo<'a,'b> {


14 note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 13:6
-   --> $DIR/trait-matching-lifetimes.rs:13:6
16    |
16    |
17 LL | impl<'a,'b> Tr for Foo<'a,'b> {

19 
20 error[E0308]: method not compatible with trait
-   --> $DIR/trait-matching-lifetimes.rs:14:5
-   --> $DIR/trait-matching-lifetimes.rs:14:5
+   --> $DIR/matching-lifetimes.rs:14:5
22    |
23 LL |     fn foo(x: Foo<'b,'a>) {
24    |     ^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch

26    = note: expected fn pointer `fn(Foo<'a, 'b>)`
27               found fn pointer `fn(Foo<'b, 'a>)`
28 note: the lifetime `'a` as defined on the impl at 13:6...
-   --> $DIR/trait-matching-lifetimes.rs:13:6
30    |
30    |
31 LL | impl<'a,'b> Tr for Foo<'a,'b> {


33 note: ...does not necessarily outlive the lifetime `'b` as defined on the impl at 13:9
-   --> $DIR/trait-matching-lifetimes.rs:13:9
35    |
35    |
36 LL | impl<'a,'b> Tr for Foo<'a,'b> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/matching-lifetimes/matching-lifetimes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/matching-lifetimes/matching-lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/matching-lifetimes.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/matching-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/matching-lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/matching-lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/traits/matching-lifetimes.rs:14:5
   |
LL |     fn foo(x: Foo<'b,'a>) {
   |     ^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected fn pointer `fn(Foo<'a, 'b>)`
              found fn pointer `fn(Foo<'b, 'a>)`
note: the lifetime `'b` as defined on the impl at 13:9...
   |
   |
LL | impl<'a,'b> Tr for Foo<'a,'b> {
   |         ^^
note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 13:6
   |
   |
LL | impl<'a,'b> Tr for Foo<'a,'b> {

error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/traits/matching-lifetimes.rs:14:5
   |
   |
LL |     fn foo(x: Foo<'b,'a>) {
   |     ^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected fn pointer `fn(Foo<'a, 'b>)`
              found fn pointer `fn(Foo<'b, 'a>)`
note: the lifetime `'a` as defined on the impl at 13:6...
   |
   |
LL | impl<'a,'b> Tr for Foo<'a,'b> {
   |      ^^
note: ...does not necessarily outlive the lifetime `'b` as defined on the impl at 13:9
   |
   |
LL | impl<'a,'b> Tr for Foo<'a,'b> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/traits/method-private.rs stdout ----
diff of stderr:

1 error[E0624]: associated function `method` is private
-   --> $DIR/trait-method-private.rs:19:9
3    |
3    |
4 LL |     foo.method();
5    |         ^^^^^^ private associated function

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/method-private/method-private.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/method-private/method-private.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/method-private.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/method-private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/method-private" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/method-private/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0624]: associated function `method` is private
   |
   |
LL |     foo.method(); //~ ERROR is private
   |         ^^^^^^ private associated function
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use inner::Bar;
---

---- [ui] ui/traits/item-privacy.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `a` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:67:7
3    |
4 LL | struct S;
4 LL | struct S;
5    | --------- method `a` not found for this
9    |
10    = help: items from traits can only be used if the trait is implemented and in scope
10    = help: items from traits can only be used if the trait is implemented and in scope
11 note: `method::A` defines an item `a`, perhaps you need to implement it
-   --> $DIR/trait-item-privacy.rs:6:5
13    |
14 LL |     trait A {
15    |     ^^^^^^^


16 
17 error[E0599]: no method named `b` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:68:7
19    |
20 LL | struct S;
20 LL | struct S;
21    | --------- method `b` not found for this
30    |
31 
31 
32 error[E0624]: associated function `a` is private
-   --> $DIR/trait-item-privacy.rs:72:7
34    |
35 LL |     c.a();
36    |       ^ private associated function


37 
38 error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:78:8
40    |
41 LL | struct S;
41 LL | struct S;
42    | --------- function or associated item `a` not found for this
46    |
47    = help: items from traits can only be used if the trait is implemented and in scope
47    = help: items from traits can only be used if the trait is implemented and in scope
48 note: `method::A` defines an item `a`, perhaps you need to implement it
-   --> $DIR/trait-item-privacy.rs:6:5
50    |
51 LL |     trait A {
52    |     ^^^^^^^


53 
54 error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:80:8
56    |
57 LL | struct S;
57 LL | struct S;
58    | --------- function or associated item `b` not found for this
67    |
68 
68 
69 error[E0624]: associated function `a` is private
-   --> $DIR/trait-item-privacy.rs:84:8
71    |
71    |
72 LL |     C::a(&S);
73    |        ^ private associated function
74 
74 
75 error[E0599]: no associated item named `A` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:97:8
77    |
78 LL | struct S;
78 LL | struct S;
79    | --------- associated item `A` not found for this
83    |
84    = help: items from traits can only be used if the trait is implemented and in scope
84    = help: items from traits can only be used if the trait is implemented and in scope
85 note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
-   --> $DIR/trait-item-privacy.rs:24:5
87    |
88 LL |     trait A {
89    |     ^^^^^^^


90 
91 error[E0599]: no associated item named `B` found for struct `S` in the current scope
-   --> $DIR/trait-item-privacy.rs:98:8
93    |
94 LL | struct S;
94 LL | struct S;
95    | --------- associated item `B` not found for this
104    |
105 
105 
106 error[E0624]: associated constant `A` is private
-   --> $DIR/trait-item-privacy.rs:101:8
108    |
109 LL |     C::A;
110    |        ^ private associated constant


111 
112 error[E0038]: the trait `assoc_const::C` cannot be made into an object
-   --> $DIR/trait-item-privacy.rs:101:5
114    |
115 LL |     C::A;
115 LL |     C::A;
116    |     ^^^^ `assoc_const::C` cannot be made into an object

119    = help: consider moving `B` to another trait
120    = help: consider moving `A` to another trait
121 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-item-privacy.rs:25:15
123    |
123    |
124 LL |         const A: u8 = 0;
125    |               ^ ...because it contains this associated `const`

133    |               ^ ...because it contains this associated `const`
135 error[E0223]: ambiguous associated type
-   --> $DIR/trait-item-privacy.rs:115:12
+   --> $DIR/item-privacy.rs:115:12
137    |
137    |
138 LL |     let _: S::A;
139    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
140 
141 error[E0223]: ambiguous associated type
-   --> $DIR/trait-item-privacy.rs:116:12
+   --> $DIR/item-privacy.rs:116:12
+   --> $DIR/item-privacy.rs:116:12
143    |
144 LL |     let _: S::B;
145    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
146 
147 error[E0223]: ambiguous associated type
-   --> $DIR/trait-item-privacy.rs:117:12
+   --> $DIR/item-privacy.rs:117:12
+   --> $DIR/item-privacy.rs:117:12
149    |
150 LL |     let _: S::C;
151    |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`
152 
152 
153 error: associated type `A` is private
-   --> $DIR/trait-item-privacy.rs:119:12
155    |
155    |
156 LL |     let _: T::A;
157    |            ^^^^ private associated type
158 
158 
159 error: associated type `A` is private
-   --> $DIR/trait-item-privacy.rs:128:9
161    |
162 LL |         A = u8,
163    |         ^^^^^^ private associated type



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/item-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/item-privacy.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `a` not found for this
...
LL |     S.a(); //~ ERROR no method named `a` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no method named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `b` not found for this
...
LL |     S.b(); //~ ERROR no method named `b` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     c.a(); //~ ERROR associated function `a` is private
   |       ^ private associated function

error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `a` not found for this
...
LL |     S::a(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `b` not found for this
...
LL |     S::b(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     C::a(&S); //~ ERROR associated function `a` is private
   |        ^ private associated function

error[E0599]: no associated item named `A` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `A` not found for this
...
LL |     S::A; //~ ERROR no associated item named `A` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no associated item named `B` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `B` not found for this
...
LL |     S::B; //~ ERROR no associated item named `B` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use assoc_const::B;
LL | use assoc_const::B;
   |

error[E0624]: associated constant `A` is private
   |
   |
LL |     C::A; //~ ERROR associated constant `A` is private
   |        ^ private associated constant

error[E0038]: the trait `assoc_const::C` cannot be made into an object
   |
   |
LL |     C::A; //~ ERROR associated constant `A` is private
   |     ^^^^ `assoc_const::C` cannot be made into an object
   |
   = help: consider moving `C` to another trait
   = help: consider moving `B` to another trait
   = help: consider moving `A` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL |         const A: u8 = 0;
   |               ^ ...because it contains this associated `const`
...
LL |         const B: u8 = 0;
   |               ^ ...because it contains this associated `const`
...
LL |     pub trait C: A + B {
   |               - this trait cannot be made into an object...
LL |         const C: u8 = 0;
   |               ^ ...because it contains this associated `const`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:115:12
   |
   |
LL |     let _: S::A; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:116:12
   |
   |
LL |     let _: S::B; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:117:12
   |
   |
LL |     let _: S::C; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`

error: associated type `A` is private
   |
   |
LL |     let _: T::A; //~ ERROR associated type `A` is private
   |            ^^^^ private associated type

error: associated type `A` is private
   |
   |
LL |         A = u8, //~ ERROR associated type `A` is private
   |         ^^^^^^ private associated type
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0038, E0223, E0599, E0624.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/object/auto-dedup-in-impl.rs stdout ----


1 error[E0592]: duplicate definitions with name `test`
-   --> $DIR/trait-object-auto-dedup-in-impl.rs:14:5
3    |
3    |
4 LL |     fn test(&self) { println!("one"); }
5    |     ^^^^^^^^^^^^^^ duplicate definitions for `test`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/auto-dedup-in-impl/auto-dedup-in-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/auto-dedup-in-impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/auto-dedup-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/auto-dedup-in-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/auto-dedup-in-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0592]: duplicate definitions with name `test`
  --> /checkout/src/test/ui/traits/object/auto-dedup-in-impl.rs:14:5
   |
LL |     fn test(&self) { println!("one"); } //~ ERROR duplicate definitions with name `test`
   |     ^^^^^^^^^^^^^^ duplicate definitions for `test`
...
LL |     fn test(&self) { println!("two"); }
   |     -------------- other definition for `test`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0592`.


------------------------------------------


---- [ui] ui/traits/object/vs-lifetime-2.rs stdout ----

1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-vs-lifetime-2.rs:7:5
+   --> $DIR/vs-lifetime-2.rs:7:5
+   --> $DIR/vs-lifetime-2.rs:7:5
3    |
4 LL |     dyn 'static +: 'static + Copy,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime-2/vs-lifetime-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/vs-lifetime-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/vs-lifetime-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/vs-lifetime-2.rs:7:5
   |
LL |     dyn 'static +: 'static + Copy,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0224`.
---
1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-macro-matcher.rs:11:8
+   --> $DIR/macro-matcher.rs:11:8
3    |
4 LL |     m!(dyn 'static +);

6 
6 
7 error[E0038]: the trait `Copy` cannot be made into an object
-   --> $DIR/trait-object-macro-matcher.rs:8:8
9    |
9    |
10 LL |     m!(dyn Copy + Send + 'static);
11    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ `Copy` cannot be made into an object

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/macro-matcher.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/macro-matcher.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/macro-matcher.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/macro-matcher.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/macro-matcher.rs:11:8
   |
LL |     m!(dyn 'static +); //~ ERROR at least one trait is required for an object type


error[E0038]: the trait `Copy` cannot be made into an object
   |
   |
LL |     m!(dyn Copy + Send + 'static);
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ `Copy` cannot be made into an object
   |
   = note: the trait cannot be made into an object because it requires `Self: Sized`
   = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0038, E0224.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/object/safety.rs stdout ----


1 error[E0038]: the trait `Tr` cannot be made into an object
-   --> $DIR/trait-object-safety.rs:15:22
3    |
3    |
4 LL |     let _: &dyn Tr = &St;
5    |                      ^^^ `Tr` cannot be made into an object
6    |
6    |
7 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-object-safety.rs:4:8
9    |
9    |
10 LL | trait Tr {
11    |       -- this trait cannot be made into an object...
23    |              ^^^^^^^^^^^^^^^^^
24 
24 
25 error[E0038]: the trait `Tr` cannot be made into an object
-   --> $DIR/trait-object-safety.rs:15:12
27    |
27    |
28 LL |     let _: &dyn Tr = &St;
29    |            ^^^^^^^ `Tr` cannot be made into an object
30    |
30    |
31 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-object-safety.rs:4:8
33    |
33    |
34 LL | trait Tr {
35    |       -- this trait cannot be made into an object...

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/safety.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/safety.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/safety.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/safety.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0038]: the trait `Tr` cannot be made into an object
  --> /checkout/src/test/ui/traits/object/safety.rs:15:22
   |
LL |     let _: &dyn Tr = &St; //~ ERROR E0038
   |                      ^^^ `Tr` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/object/safety.rs:4:8
   |
LL | trait Tr {
   |       -- this trait cannot be made into an object...
LL |     fn foo();
   |        ^^^ ...because associated function `foo` has no `self` parameter
   = note: required because of the requirements on the impl of `CoerceUnsized<&dyn Tr>` for `&St`
   = note: required by cast to type `&dyn Tr`
help: consider turning `foo` into a method by giving it a `&self` argument
   |
LL |     fn foo(&self);
   |            ^^^^^
help: alternatively, consider constraining `foo` so it does not apply to trait objects
   |
LL |     fn foo() where Self: Sized;


error[E0038]: the trait `Tr` cannot be made into an object
  --> /checkout/src/test/ui/traits/object/safety.rs:15:12
   |
LL |     let _: &dyn Tr = &St; //~ ERROR E0038
   |            ^^^^^^^ `Tr` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/object/safety.rs:4:8
   |
LL | trait Tr {
   |       -- this trait cannot be made into an object...
LL |     fn foo();
   |        ^^^ ...because associated function `foo` has no `self` parameter
help: consider turning `foo` into a method by giving it a `&self` argument
   |
LL |     fn foo(&self);
   |            ^^^^^
help: alternatively, consider constraining `foo` so it does not apply to trait objects
   |
LL |     fn foo() where Self: Sized;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0038`.
For more information about this error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/object/vs-lifetime.rs stdout ----

1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-vs-lifetime.rs:9:23
+   --> $DIR/vs-lifetime.rs:9:23
+   --> $DIR/vs-lifetime.rs:9:23
3    |
4 LL |     let _: S<'static, dyn 'static +>;

6 
6 
7 error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
-   --> $DIR/trait-object-vs-lifetime.rs:11:12
+   --> $DIR/vs-lifetime.rs:11:12
9    |
10 LL |     let _: S<'static, 'static>;
11    |            ^        --------- help: remove this lifetime argument
13    |            expected 1 lifetime argument
14    |
14    |
15 note: struct defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/trait-object-vs-lifetime.rs:4:8
+   --> $DIR/vs-lifetime.rs:4:8
17    |
18 LL | struct S<'a, T>(&'a u8, T);
19    |        ^ --
20 
20 
21 error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
-   --> $DIR/trait-object-vs-lifetime.rs:11:12
+   --> $DIR/vs-lifetime.rs:11:12
23    |
24 LL |     let _: S<'static, 'static>;
25    |            ^ expected 1 type argument
26    |
26    |
27 note: struct defined here, with 1 type parameter: `T`
-   --> $DIR/trait-object-vs-lifetime.rs:4:8
+   --> $DIR/vs-lifetime.rs:4:8
29    |
30 LL | struct S<'a, T>(&'a u8, T);
31    |        ^     -
35    |                              ^^^
36 
37 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
+   --> $DIR/vs-lifetime.rs:14:14
39    |
40 LL |     let _: S<dyn 'static +, 'static>;

42 
42 
43 error[E0747]: type provided when a lifetime was expected
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
+   --> $DIR/vs-lifetime.rs:14:14
45    |
46 LL |     let _: S<dyn 'static +, 'static>;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime/vs-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/vs-lifetime.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/vs-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:9:23
   |
LL |     let _: S<'static, dyn 'static +>;


error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:11:12
   |
LL |     let _: S<'static, 'static>;
   |            ^        --------- help: remove this lifetime argument
   |            expected 1 lifetime argument
   |
   |
note: struct defined here, with 1 lifetime parameter: `'a`
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:4:8
   |
LL | struct S<'a, T>(&'a u8, T);
   |        ^ --

error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:11:12
   |
LL |     let _: S<'static, 'static>;
   |            ^ expected 1 type argument
   |
note: struct defined here, with 1 type parameter: `T`
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:4:8
   |
LL | struct S<'a, T>(&'a u8, T);
   |        ^     -
help: add missing type argument
   |
LL |     let _: S<'static, 'static, T>;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:14:14
   |
   |
LL |     let _: S<dyn 'static +, 'static>;


error[E0747]: type provided when a lifetime was expected
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:14:14
   |
LL |     let _: S<dyn 'static +, 'static>;

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0107, E0224, E0747.
Some errors have detailed explanations: E0107, E0224, E0747.
For more information about an error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/traits/or-new-type-instead.rs stdout ----


1 error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
-   --> $DIR/trait-or-new-type-instead.rs:1:1
+   --> $DIR/or-new-type-instead.rs:1:1
3    |
4 LL | / impl<T> Option<T> {
5 LL | |

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/or-new-type-instead/or-new-type-instead.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/or-new-type-instead.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/or-new-type-instead.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/or-new-type-instead" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/or-new-type-instead/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
  --> /checkout/src/test/ui/traits/or-new-type-instead.rs:1:1
   |
LL | / impl<T> Option<T> {
LL | | //~^ ERROR cannot define inherent `impl` for a type outside of the crate where the type is defined
LL | |     pub fn foo(&self) { }
LL | | }
   | |_^ impl for type defined outside of crate.
   |
   = note: define and implement a trait or new type instead
error: aborting due to previous error

For more information about this error, try `rustc --explain E0116`.


------------------------------------------


---- [ui] ui/traits/object/with-self-in-projection-output-bad.rs stdout ----
diff of stderr:

1 error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
-   --> $DIR/trait-object-with-self-in-projection-output-bad.rs:45:21
3    |
4 LL |     type Output;
4 LL |     type Output;
5    |     ------------ `Output` defined here

8    |                     ^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Helper<Target=i32, Output = Type>`
9 
10 error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
-   --> $DIR/trait-object-with-self-in-projection-output-bad.rs:48:21
12    |
13 LL |     type Output;
13 LL |     type Output;
14    |     ------------ `Output` defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/with-self-in-projection-output-bad.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/with-self-in-projection-output-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/with-self-in-projection-output-bad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/with-self-in-projection-output-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
   |
LL |     type Output;
LL |     type Output;
   |     ------------ `Output` defined here
...
LL |     let _x: Box<dyn Helper<Target=i32>> = Box::new(2u32);
   |                     ^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Helper<Target=i32, Output = Type>`

error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
   |
LL |     type Output;
LL |     type Output;
   |     ------------ `Output` defined here
...
LL |     let _y: Box<dyn NormalizableHelper<Target=i32>> = Box::new(2u32);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `NormalizableHelper<Target=i32, Output = Type>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0191`.


------------------------------------------


---- [ui] ui/traits/param-without-lifetime-constraint.rs stdout ----
diff of stderr:

1 error: `impl` item signature doesn't match `trait` item signature
-   --> $DIR/trait-param-without-lifetime-constraint.rs:14:5
3    |
4 LL |     fn get_relation(&self) -> To;
4 LL |     fn get_relation(&self) -> To;
5    |     ----------------------------- expected `fn(&Article) -> &ProofReader`

10    = note: expected `fn(&Article) -> &ProofReader`
11               found `fn(&Article) -> &ProofReader`
12 help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
-   --> $DIR/trait-param-without-lifetime-constraint.rs:10:31
14    |
15 LL |     fn get_relation(&self) -> To;
15 LL |     fn get_relation(&self) -> To;
16    |                               ^^ consider borrowing this type parameter in the trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/param-without-lifetime-constraint/param-without-lifetime-constraint.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/param-without-lifetime-constraint/param-without-lifetime-constraint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/param-without-lifetime-constraint.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/param-without-lifetime-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/param-without-lifetime-constraint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/param-without-lifetime-constraint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `impl` item signature doesn't match `trait` item signature
   |
LL |     fn get_relation(&self) -> To;
LL |     fn get_relation(&self) -> To;
   |     ----------------------------- expected `fn(&Article) -> &ProofReader`
...
LL |     fn get_relation(&self) -> &ProofReader {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&Article) -> &ProofReader`
   |
   = note: expected `fn(&Article) -> &ProofReader`
              found `fn(&Article) -> &ProofReader`
help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
   |
LL |     fn get_relation(&self) -> To;
LL |     fn get_relation(&self) -> To;
   |                               ^^ consider borrowing this type parameter in the trait
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/traits/resolution-in-overloaded-op.rs stdout ----
diff of stderr:

1 error[E0369]: cannot multiply `&T` by `f64`
-   --> $DIR/trait-resolution-in-overloaded-op.rs:8:7
3    |
4 LL |     a * b
4 LL |     a * b
5    |     - ^ - f64

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op/resolution-in-overloaded-op.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op/resolution-in-overloaded-op.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/resolution-in-overloaded-op.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/resolution-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: cannot multiply `&T` by `f64`
   |
   |
LL |     a * b //~ ERROR cannot multiply `&T` by `f64`
   |     - ^ - f64
   |     &T
   |
help: consider further restricting this bound
   |
   |
LL | fn foo<T: MyMul<f64, f64> + std::ops::Mul<Output = f64>>(a: &T, b: f64) -> f64 {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
For more information about this error, try `rustc --explain E0369`.

------------------------------------------


---- [ui] ui/traits/repeated-superambig.rs stdout ----


1 error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
-   --> $DIR/traits-repeated-supertrait-ambig.rs:26:7
+   --> $DIR/repeated-superambig.rs:26:7
3    |
4 LL |     c.same_as(22)
5    |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`
---
1 error[E0283]: type annotations needed
-   --> $DIR/trait-static-method-generic-inference.rs:24:25
+   --> $DIR/static-method-generic-inference.rs:24:25
3    |
4 LL |         fn new() -> T;
5    |         -------------- required by `HasNew::new`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/static-method-generic-inference.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/static-method-generic-inference.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/static-method-generic-inference.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/static-method-generic-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/static-method-generic-inference.rs:24:25
   |
LL |         fn new() -> T;
   |         -------------- required by `HasNew::new`
...
LL |     let _f: base::Foo = base::HasNew::new();
   |
   |
   = note: cannot satisfy `_: HasNew<Foo>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.


------------------------------------------


---- [ui] ui/traits/test.rs stdout ----
diff of stderr:

1 error[E0404]: expected trait, found builtin type `isize`
-   --> $DIR/trait-test.rs:4:6
3    |
3    |
4 LL | impl isize for usize { fn foo(&self) {} }
5    |      ^^^^^ not a trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test/test.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test/test.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/test.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found builtin type `isize`
   |
   |
LL | impl isize for usize { fn foo(&self) {} } //~ ERROR trait
   |      ^^^^^ not a trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0404`.


------------------------------------------


---- [ui] ui/traits/test-2.rs stdout ----
diff of stderr:

1 error[E0107]: this associated function takes 0 type arguments but 1 type argument was supplied
-   --> $DIR/trait-test-2.rs:9:8
3    |
3    |
4 LL |     10.dup::<i32>();
5    |        ^^^------- help: remove these generics
7    |        expected 0 type arguments
8    |
8    |
9 note: associated function defined here, with 0 type parameters
-   --> $DIR/trait-test-2.rs:4:16
11    |
11    |
12 LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }

14 
14 
15 error[E0107]: this associated function takes 1 type argument but 2 type arguments were supplied
-   --> $DIR/trait-test-2.rs:11:8
17    |
17    |
18 LL |     10.blah::<i32, i32>();
19    |        ^^^^      ----- help: remove this type argument
21    |        expected 1 type argument
22    |
22    |
23 note: associated function defined here, with 1 type parameter: `X`
-   --> $DIR/trait-test-2.rs:4:39
25    |
25    |
26 LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }

28 
28 
29 error[E0038]: the trait `bar` cannot be made into an object
-   --> $DIR/trait-test-2.rs:13:16
31    |
31    |
32 LL |     (box 10 as Box<dyn bar>).dup();
33    |                ^^^^^^^^^^^^ `bar` cannot be made into an object

35    = help: consider moving `dup` to another trait
36    = help: consider moving `blah` to another trait
37 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-test-2.rs:4:30
39    |
39    |
40 LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
41    |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters

44    |       this trait cannot be made into an object...
45 
46 error[E0038]: the trait `bar` cannot be made into an object
-   --> $DIR/trait-test-2.rs:13:6
48    |
48    |
49 LL |     (box 10 as Box<dyn bar>).dup();
50    |      ^^^^^^ `bar` cannot be made into an object

52    = help: consider moving `dup` to another trait
53    = help: consider moving `blah` to another trait
54 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-test-2.rs:4:30
56    |
56    |
57 LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
58    |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2/test-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/test-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/test-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this associated function takes 0 type arguments but 1 type argument was supplied
   |
   |
LL |     10.dup::<i32>();
   |        ^^^------- help: remove these generics
   |        expected 0 type arguments
   |
   |
note: associated function defined here, with 0 type parameters
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }


error[E0107]: this associated function takes 1 type argument but 2 type arguments were supplied
   |
   |
LL |     10.blah::<i32, i32>();
   |        ^^^^      ----- help: remove this type argument
   |        expected 1 type argument
   |
   |
note: associated function defined here, with 1 type parameter: `X`
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }


error[E0038]: the trait `bar` cannot be made into an object
   |
   |
LL |     (box 10 as Box<dyn bar>).dup();
   |                ^^^^^^^^^^^^ `bar` cannot be made into an object
   |
   = help: consider moving `dup` to another trait
   = help: consider moving `blah` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
   |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters
   |       |                      |
   |       |                      ...because method `dup` references the `Self` type in its return type
   |       this trait cannot be made into an object...

error[E0038]: the trait `bar` cannot be made into an object
   |
   |
LL |     (box 10 as Box<dyn bar>).dup();
   |      ^^^^^^ `bar` cannot be made into an object
   |
   = help: consider moving `dup` to another trait
   = help: consider moving `blah` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
   |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters
   |       |                      |
   |       |                      ...because method `dup` references the `Self` type in its return type
   |       this trait cannot be made into an object...
   = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn bar>>` for `Box<{integer}>`
   = note: required by cast to type `Box<dyn bar>`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0038, E0107.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/suggest-deferences/issue-39029.rs stdout ----


1 error[E0277]: the trait bound `NoToSocketAddrs: ToSocketAddrs` is not satisfied
-   --> $DIR/trait-suggest-deferences-issue-39029.rs:16:37
3    |
3    |
4 LL |     let _errors = TcpListener::bind(&bad);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/issue-39029.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/issue-39029.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/issue-39029.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/issue-39029.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `NoToSocketAddrs: ToSocketAddrs` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/issue-39029.rs:16:37
   |
LL |     let _errors = TcpListener::bind(&bad);
   |                                     |
   |                                     |
   |                                     the trait `ToSocketAddrs` is not implemented for `NoToSocketAddrs`
   |                                     help: consider adding dereference here: `&*bad`
  ::: /checkout/library/std/src/net/tcp.rs:696:20
   |
   |
LL |     pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
   |                    ------------- required by this bound in `TcpListener::bind`
   |
   = note: required because of the requirements on the impl of `ToSocketAddrs` for `&NoToSocketAddrs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/safety-trait-impl-cc.rs stdout ----


1 error[E0200]: the trait `Foo` requires an `unsafe impl` declaration
-   --> $DIR/trait-safety-trait-impl-cc.rs:9:1
3    |
3    |
4 LL | / impl lib::Foo for Bar {
5 LL | |     fn foo(&self) -> isize {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-trait-impl-cc/safety-trait-impl-cc.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-trait-impl-cc/safety-trait-impl-cc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/safety-trait-impl-cc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/safety-trait-impl-cc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-trait-impl-cc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-trait-impl-cc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0200]: the trait `Foo` requires an `unsafe impl` declaration
  --> /checkout/src/test/ui/traits/safety-trait-impl-cc.rs:9:1
   |
LL | / impl lib::Foo for Bar { //~ ERROR requires an `unsafe impl` declaration
LL | |     fn foo(&self) -> isize {
LL | |         panic!();
LL | |     }
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0200`.
For more information about this error, try `rustc --explain E0200`.

------------------------------------------


---- [ui] ui/traits/suggest-deferences/issue-62530.rs stdout ----


1 error[E0277]: the trait bound `&String: SomeTrait` is not satisfied
-   --> $DIR/trait-suggest-deferences-issue-62530.rs:13:26
3    |
3    |
4 LL | fn takes_type_parameter<T>(_x: T) where T: SomeTrait {}
5    |                                            --------- required by this bound in `takes_type_parameter`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/issue-62530.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/issue-62530.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/issue-62530.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/issue-62530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&String: SomeTrait` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/issue-62530.rs:13:26
   |
LL | fn takes_type_parameter<T>(_x: T) where T: SomeTrait {}
   |                                            --------- required by this bound in `takes_type_parameter`
...
LL |     takes_type_parameter(&string);  // Error
   |                          |
   |                          |
   |                          the trait `SomeTrait` is not implemented for `&String`
   |                          help: consider adding dereference here: `&*string`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/suggest-deferences/multiple-1.rs stdout ----


1 error[E0277]: the trait bound `&mut Baz: Happy` is not satisfied
-   --> $DIR/trait-suggest-deferences-multiple-1.rs:52:9
3    |
3    |
4 LL | fn foo<T>(_: T) where T: Happy {}
5    |                          ----- required by this bound in `foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1/multiple-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/multiple-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/multiple-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&mut Baz: Happy` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/multiple-1.rs:52:9
   |
LL | fn foo<T>(_: T) where T: Happy {}
   |                          ----- required by this bound in `foo`
...
LL |     foo(&mut baz);
   |         ^^^^^^^^ the trait `Happy` is not implemented for `&mut Baz`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/suggest-deferences/multiple-0.rs stdout ----


1 error[E0277]: the trait bound `&Baz: Happy` is not satisfied
-   --> $DIR/trait-suggest-deferences-multiple-0.rs:34:9
3    |
3    |
4 LL | fn foo<T>(_: T) where T: Happy {}
5    |                          ----- required by this bound in `foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0/multiple-0.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/multiple-0.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/multiple-0.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&Baz: Happy` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/multiple-0.rs:34:9
   |
LL | fn foo<T>(_: T) where T: Happy {}
   |                          ----- required by this bound in `foo`
...
LL |     foo(&baz);
   |         |
   |         |
   |         the trait `Happy` is not implemented for `&Baz`
   |         help: consider adding dereference here: `&***baz`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/wf-object/maybe-bound.rs stdout ----


1 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:5:15
3    |
3    |
4 LL | type _0 = dyn ?Sized + Foo;

6 
6 
7 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:8:21
9    |
9    |
10 LL | type _1 = dyn Foo + ?Sized;

12 
12 
13 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:11:21
15    |
15    |
16 LL | type _2 = dyn Foo + ?Sized + ?Sized;

18 
18 
19 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:11:30
21    |
21    |
22 LL | type _2 = dyn Foo + ?Sized + ?Sized;

24 
24 
25 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:15:15
27    |
27    |
28 LL | type _3 = dyn ?Sized + Foo;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound/maybe-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound/maybe-bound.stderr
---
    [ui] ui/traits/impl-different-num-params.rs
    [ui] ui/traits/impl-for-module.rs
    [ui] ui/traits/impl-method-mismatch.rs
    [ui] ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs
    [ui] ui/traits/inductive-overflow/lifetime.rs
    [ui] ui/traits/inductive-overflow/simultaneous.rs
    [ui] ui/traits/inductive-overflow/superauto-trait.rs
    [ui] ui/traits/inductive-overflow/supertrait.rs
    [ui] ui/traits/inductive-overflow/two-traits.rs
    [ui] ui/traits/inheritance/auto-xc-2.rs
    [ui] ui/traits/inheritance/auto-xc.rs
    [ui] ui/traits/inheritance/overloading-xc-exe.rs
    [ui] ui/traits/issue-65284-suggest-generic-bound.rs
    [ui] ui/traits/issue-71136.rs
    [ui] ui/traits/item-privacy.rs
---
test result: FAILED. 11266 passed; 66 failed; 92 ignored; 0 measured; 0 filtered out; finished in 133.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:43
