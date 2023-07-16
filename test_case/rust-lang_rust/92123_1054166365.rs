plain

13 error[E0106]: missing lifetime specifiers
14   --> $DIR/missing-lifetime-specifier.rs:18:44
15    |
- LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
-    |                                            ^^^ expected 2 lifetime parameters
+ LL | / thread_local! {
+ LL | |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
+    | |                                            ^^^ expected 2 lifetime parameters
+ LL | |
+ LL | |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | | }
18    |
18    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static a: RefCell<HashMap<i32, Vec<Vec<Foo<'static, 'static>>>>> = RefCell::new(HashMap::new());
-    |                                            ~~~~~~~~~~~~~~~~~~~~~
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from
25 error[E0106]: missing lifetime specifier
26   --> $DIR/missing-lifetime-specifier.rs:23:44

49 error[E0106]: missing lifetime specifier
49 error[E0106]: missing lifetime specifier
50   --> $DIR/missing-lifetime-specifier.rs:23:44
51    |
- LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
-    |                                            ^ expected named lifetime parameter
+ LL | / thread_local! {
+ LL | |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    | |                                            ^ expected named lifetime parameter
+ LL | |
+ LL | |
+ LL | |
+ LL | |
+ LL | | }
54    |
54    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static b: RefCell<HashMap<i32, Vec<Vec<&'static Bar>>>> = RefCell::new(HashMap::new());
-    |                                            ~~~~~~~~
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
61 error[E0106]: missing lifetime specifiers
62   --> $DIR/missing-lifetime-specifier.rs:23:45

63    |
63    |
- LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
-    |                                             ^^^ expected 2 lifetime parameters
+ LL | / thread_local! {
+ LL | |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
+    | |                                             ^^^ expected 2 lifetime parameters
+ LL | |
+ LL | |
+ LL | |
+ LL | |
+ LL | | }
66    |
66    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar<'static, 'static>>>>> = RefCell::new(HashMap::new());
-    |                                             ~~~~~~~~~~~~~~~~~~~~~
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
73 error[E0106]: missing lifetime specifiers
74   --> $DIR/missing-lifetime-specifier.rs:30:48

85 error[E0106]: missing lifetime specifiers
85 error[E0106]: missing lifetime specifiers
86   --> $DIR/missing-lifetime-specifier.rs:30:48
87    |
- LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
-    |                                                ^ expected 2 lifetime parameters
+ LL | / thread_local! {
+ LL | |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
+    | |                                                ^ expected 2 lifetime parameters
+ LL | |
+ LL | |
+ LL | | }
90    |
90    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
-    |                                                +++++++++++++++++
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from
97 error[E0106]: missing lifetime specifier
98   --> $DIR/missing-lifetime-specifier.rs:35:44

121 error[E0106]: missing lifetime specifier
121 error[E0106]: missing lifetime specifier
122   --> $DIR/missing-lifetime-specifier.rs:35:44
123    |
- LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
-    |                                            ^ expected named lifetime parameter
+ LL | / thread_local! {
+ LL | |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    | |                                            ^ expected named lifetime parameter
+ LL | |
+ LL | |
+ LL | |
+ LL | |
+ LL | | }
126    |
126    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static d: RefCell<HashMap<i32, Vec<Vec<&'static Tar<i32>>>>> = RefCell::new(HashMap::new());
-    |                                            ~~~~~~~~
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
133 error[E0106]: missing lifetime specifiers
134   --> $DIR/missing-lifetime-specifier.rs:35:49

135    |
135    |
- LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
-    |                                                 ^ expected 2 lifetime parameters
+ LL | / thread_local! {
+ LL | |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
+    | |                                                 ^ expected 2 lifetime parameters
+ LL | |
+ LL | |
+ LL | |
+ LL | |
+ LL | | }
138    |
138    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
-    |
- LL |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'static, i32>>>>> = RefCell::new(HashMap::new());
-    |                                                 +++++++++++++++++
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
144 
145 error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied

211    |           ^^^ --  --
212 help: add missing lifetime argument
213    |
213    |
+ LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());
+ 
+ 
+ error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
+    |
+    |
+ LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
+    |                                            ^^^ ------- supplied 1 lifetime argument
+    |                                            expected 2 lifetime arguments
+    |
+    |
+ note: union defined here, with 2 lifetime parameters: `'t`, `'k`
+    |
+    |
+ LL | pub union Qux<'t, 'k, I> {
+ help: add missing lifetime argument
+    |
+    |
214 LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, '_, i32>>>>> = RefCell::new(HashMap::new());
216 

265 error[E0106]: missing lifetime specifier
266   --> $DIR/missing-lifetime-specifier.rs:50:44
266   --> $DIR/missing-lifetime-specifier.rs:50:44
267    |
+ LL | / thread_local! {
+ LL | |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
+    | |                                            ^ expected named lifetime parameter
+ LL | |
+ LL | |
+ LL | |
+ LL | | }
+    | |_-
+    |
+    |
+    = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from
+ 
+ error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
+    |
+    |
268 LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
-    |                                            ^ expected named lifetime parameter
+    |                                             ^^^ ------- supplied 1 lifetime argument
+    |                                             expected 2 lifetime arguments
270    |
270    |
-    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
- help: consider using the `'static` lifetime
+ note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
273    |
273    |
- LL |     static f: RefCell<HashMap<i32, Vec<Vec<&'static Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
-    |                                            ~~~~~~~~
+ LL | trait Tar<'t, 'k, I> {}
+ help: add missing lifetime argument
+    |
+    |
+ LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());
276 
276 
277 error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied


310 LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, '_, i32>>>>> = RefCell::new(HashMap::new());
312 
- error: aborting due to 22 previous errors
+ error: aborting due to 24 previous errors
314 
---
To only update this specific test, also pass `--test-args suggestions/missing-lifetime-specifier.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifiers
   |
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
LL | / thread_local! {
LL | |     static a: RefCell<HashMap<i32, Vec<Vec<Foo>>>> = RefCell::new(HashMap::new());
   | |                                            ^^^ expected 2 lifetime parameters
LL | |       //~^ ERROR missing lifetime specifiers
LL | |       //~| ERROR missing lifetime specifiers
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from
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
LL | / thread_local! {
LL | |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   | |                                            ^ expected named lifetime parameter
LL | |       //~^ ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23:45
   |
   |
LL | / thread_local! {
LL | |     static b: RefCell<HashMap<i32, Vec<Vec<&Bar>>>> = RefCell::new(HashMap::new());
   | |                                             ^^^ expected 2 lifetime parameters
LL | |       //~^ ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | |       //~| ERROR missing lifetime specifier
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
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
LL | / thread_local! {
LL | |     static c: RefCell<HashMap<i32, Vec<Vec<Qux<i32>>>>> = RefCell::new(HashMap::new());
   | |                                                ^ expected 2 lifetime parameters
LL | |     //~^ ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from
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
LL | / thread_local! {
LL | |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   | |                                            ^ expected named lifetime parameter
LL | |     //~^ ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from
error[E0106]: missing lifetime specifiers
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35:49
   |
   |
LL | / thread_local! {
LL | |     static d: RefCell<HashMap<i32, Vec<Vec<&Tar<i32>>>>> = RefCell::new(HashMap::new());
   | |                                                 ^ expected 2 lifetime parameters
LL | |     //~^ ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | |     //~| ERROR missing lifetime
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 4 lifetimes it is borrowed from

error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ ------- supplied 1 lifetime argument
   |                                            expected 2 lifetime arguments
   |
   |
note: union defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | pub union Qux<'t, 'k, I> {
help: add missing lifetime argument
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, '_, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ ------- supplied 1 lifetime argument
   |                                            expected 2 lifetime arguments
   |
   |
note: union defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | pub union Qux<'t, 'k, I> {
help: add missing lifetime argument
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ ------- supplied 1 lifetime argument
   |                                            expected 2 lifetime arguments
   |
   |
note: union defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | pub union Qux<'t, 'k, I> {
help: add missing lifetime argument
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ ------- supplied 1 lifetime argument
   |                                            expected 2 lifetime arguments
   |
   |
note: union defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | pub union Qux<'t, 'k, I> {
help: add missing lifetime argument
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this union takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                            ^^^ ------- supplied 1 lifetime argument
   |                                            expected 2 lifetime arguments
   |
   |
note: union defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | pub union Qux<'t, 'k, I> {
help: add missing lifetime argument
   |
   |
LL |     static e: RefCell<HashMap<i32, Vec<Vec<Qux<'static, '_, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ ------- supplied 1 lifetime argument
   |                                             expected 2 lifetime arguments
   |
   |
note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | trait Tar<'t, 'k, I> {}
help: add missing lifetime argument
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, '_, i32>>>>> = RefCell::new(HashMap::new());

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


error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ ------- supplied 1 lifetime argument
   |                                             expected 2 lifetime arguments
   |
   |
note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | trait Tar<'t, 'k, I> {}
help: add missing lifetime argument
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:50:44
   |
   |
LL | / thread_local! {
LL | |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   | |                                            ^ expected named lifetime parameter
LL | |     //~^ ERROR this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
LL | |     //~| ERROR this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
...  |
LL | |     //~| ERROR missing lifetime
LL | | }
   |
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `init`'s 3 lifetimes it is borrowed from

error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ ------- supplied 1 lifetime argument
   |                                             expected 2 lifetime arguments
   |
   |
note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | trait Tar<'t, 'k, I> {}
help: add missing lifetime argument
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ ------- supplied 1 lifetime argument
   |                                             expected 2 lifetime arguments
   |
   |
note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | trait Tar<'t, 'k, I> {}
help: add missing lifetime argument
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, 'k, i32>>>>> = RefCell::new(HashMap::new());


error[E0107]: this trait takes 2 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, i32>>>>> = RefCell::new(HashMap::new());
   |                                             ^^^ ------- supplied 1 lifetime argument
   |                                             expected 2 lifetime arguments
   |
   |
note: trait defined here, with 2 lifetime parameters: `'t`, `'k`
   |
   |
LL | trait Tar<'t, 'k, I> {}
help: add missing lifetime argument
   |
   |
LL |     static f: RefCell<HashMap<i32, Vec<Vec<&Tar<'static, '_, i32>>>>> = RefCell::new(HashMap::new());

error: aborting due to 24 previous errors

Some errors have detailed explanations: E0106, E0107.
