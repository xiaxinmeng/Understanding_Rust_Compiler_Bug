plain
diff of stderr:

20   --> $DIR/not-on-struct.rs:8:22
21    |
22 LL | fn foo(_x: Box<Foo + Send>) { }
-    |                ---   ^^^^ ...because of this bound
+    |                ---   ^^^^ ...because of these bounds
25    |                expected this type to be a trait...
26 

40   --> $DIR/not-on-struct.rs:13:15
40   --> $DIR/not-on-struct.rs:13:15
41    |
42 LL | fn a() -> A + 'static {
-    |           -   ^^^^^^^ ...because of this bound
+    |           -   ^^^^^^^ ...because of these bounds
45    |           expected this type to be a trait...
46 help: if you meant to use a type and not a trait here, remove the bounds

59   --> $DIR/not-on-struct.rs:16:48
59   --> $DIR/not-on-struct.rs:16:48
60    |
61 LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) {
-    |                                  -----------   ^^ ...because of this bound
+    |                                  -----------   ^^ ...because of these bounds
64    |                                  expected this type to be a trait...
65 help: if you meant to use a type and not a trait here, remove the bounds


80 LL | fn c() -> 'static + A {
81    |           ^^^^^^^   - expected this type to be a trait...
-    |           ...because of this bound
+    |           ...because of these bounds
84 help: if you meant to use a type and not a trait here, remove the bounds
85    |
85    |
86 LL - fn c() -> 'static + A {

99 LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) {
100    |                                  ^^   ----------- expected this type to be a trait...
-    |                                  ...because of this bound
+    |                                  ...because of these bounds
103 help: if you meant to use a type and not a trait here, remove the bounds
104    |
104    |
105 LL - fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) {
156   --> $DIR/not-on-struct.rs:35:21
157    |
157    |
158 LL | fn g() -> Traitor + 'static {
-    |           -------   ^^^^^^^ ...because of this bound
+    |           -------   ^^^^^^^ ...because of these bounds
161    |           expected this type to be a trait...
162 help: if you meant to use a type and not a trait here, remove the bounds



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct/not-on-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/bound/not-on-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/bound/not-on-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/not-on-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0226]: only a single explicit lifetime bound is permitted
   |
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found


error[E0226]: only a single explicit lifetime bound is permitted
   |
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `Foo`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:8:16
   |
   |
LL | fn foo(_x: Box<Foo + Send>) { } //~ ERROR expected trait, found struct `Foo`
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn foo(_x: Box<Foo + Send>) { } //~ ERROR expected trait, found struct `Foo`
   |                ---   ^^^^ ...because of these bounds
   |                expected this type to be a trait...

error[E0404]: expected trait, found struct `Vec`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:10:29
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:10:29
   |
LL | type TypeAlias<T> = Box<dyn Vec<T>>; //~ ERROR expected trait, found struct `Vec`

error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:13:11
   |
   |
LL | fn a() -> A + 'static { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn a() -> A + 'static { //~ ERROR expected trait, found
   |           -   ^^^^^^^ ...because of these bounds
   |           expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn a() -> A + 'static { //~ ERROR expected trait, found
LL + fn a() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:16:34
   |
   |
LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                  -----------   ^^ ...because of these bounds
   |                                  expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn b<'a,T,E>(iter: Iterator<Item=Result<T,E> + 'a>) { //~ ERROR expected trait, found
LL + fn b<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:19:21
   |
   |
LL | fn c() -> 'static + A { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn c() -> 'static + A { //~ ERROR expected trait, found
   |           ^^^^^^^   - expected this type to be a trait...
   |           ...because of these bounds
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn c() -> 'static + A { //~ ERROR expected trait, found
LL + fn c() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:22:39
   |
   |
LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) { //~ ERROR expected trait, found
   |                                  ^^   ----------- expected this type to be a trait...
   |                                  ...because of these bounds
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn d<'a,T,E>(iter: Iterator<Item='a + Result<T,E>>) { //~ ERROR expected trait, found
LL + fn d<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `A`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:25:21
   |
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn e() -> 'static + A + 'static { //~ ERROR expected trait, found
   |           ^^^^^^^   -   ^^^^^^^ ...because of these bounds
   |                     expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn e() -> 'static + A + 'static { //~ ERROR expected trait, found
LL + fn e() -> A { //~ ERROR expected trait, found

error[E0404]: expected trait, found enum `Result`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:29:39
   |
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found
   |                                  ^^   -----------   ^^ ...because of these bounds
   |                                       expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn f<'a,T,E>(iter: Iterator<Item='a + Result<T,E> + 'a>) { //~ ERROR expected trait, found
LL + fn f<'a,T,E>(iter: Iterator<Item=Result<T,E>>) { //~ ERROR expected trait, found

error[E0404]: expected trait, found struct `Traitor`
  --> /checkout/src/test/ui/traits/bound/not-on-struct.rs:35:11
   |
   |
LL | trait Trait {}
   | ----------- similarly named trait `Trait` defined here
LL | fn g() -> Traitor + 'static { //~ ERROR expected trait, found struct `Traitor`
   |
   |
help: `+` is used to constrain a "trait object" type with lifetimes or auto-traits; structs and enums can't be bound in that way
   |
   |
LL | fn g() -> Traitor + 'static { //~ ERROR expected trait, found struct `Traitor`
   |           -------   ^^^^^^^ ...because of these bounds
   |           expected this type to be a trait...
help: if you meant to use a type and not a trait here, remove the bounds
   |
   |
LL - fn g() -> Traitor + 'static { //~ ERROR expected trait, found struct `Traitor`
LL + fn g() -> Traitor { //~ ERROR expected trait, found struct `Traitor`
help: a trait with a similar name exists
   |
   |
LL | fn g() -> Trait + 'static { //~ ERROR expected trait, found struct `Traitor`

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0226, E0404.
