plain
diff of stderr:

24    |                                                   ++++
25 
26 error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:26:37
28    |
28    |
- LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined here...
-   --> $DIR/missing-lifetimes-in-signature.rs:26:26
-    |
- LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |                          ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:32:5: 34:6]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:26:37
-    |
- LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |                                     ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
-    |
- LL | fn bar<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
- 
- 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:30:1
- LL | / {
- LL | |
- LL | |     move || {
+ LL | /     move || {
+ LL | /     move || {
53 LL | |         *dest = g.get();
54 LL | |     }
- LL | | }
+    | |_____^
57    |
57    |
58 note: the parameter type `G` must be valid for the anonymous lifetime defined here...

60    |
60    |
61 LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
62    |                          ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:32:5: 34:6]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:30:1
- LL | / {
- LL | |
- LL | |     move || {
- LL | |     move || {
- LL | |         *dest = g.get();
- LL | |     }
- LL | | }
-    | |_^
- help: consider introducing an explicit lifetime bound
-    |
- LL ~ fn bar<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
- LL |
- LL | where
- LL |     G: Get<T>,
- LL | {
- LL |
-  ...
82 
83 error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:49:45
85    |
85    |
- LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined here...
-   --> $DIR/missing-lifetimes-in-signature.rs:49:34
-    |
- LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |                                  ^^^^^^
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:55:5: 57:6]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:49:45
-    |
- LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |                                             ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
-    |
- LL | fn qux<'b, 'a, G: 'b + 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'b
-    |        +++     ~~~~~~~                                                  ++++
- 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:53:1
- LL | / {
- LL | |
- LL | |     move || {
+ LL | /     move || {
+ LL | /     move || {
110 LL | |         *dest = g.get();
111 LL | |     }
- LL | | }
+    | |_____^
114    |
114    |
115 note: the parameter type `G` must be valid for the anonymous lifetime defined here...

117    |
117    |
118 LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
119    |                                  ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:55:5: 57:6]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:53:1
- LL | / {
- LL | |
- LL | |     move || {
- LL | |     move || {
- LL | |         *dest = g.get();
- LL | |     }
- LL | | }
-    | |_^
- help: consider introducing an explicit lifetime bound
-    |
- LL ~ fn qux<'b, 'a, G: 'b + 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
- LL |
- LL | where
- LL |     G: Get<T>,
- LL | {
- LL |
-  ...
139 
140 error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:62:58
142    |
142    |
- LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
+ LL | /         move || {
+ LL | /         move || {
+ LL | |             *dest = g.get();
+ LL | |         }
145    |
145    |
146 note: the parameter type `G` must be valid for the anonymous lifetime defined here...

148    |
148    |
149 LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
150    |                                               ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:65:9: 67:10]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:62:58
-    |
- LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
-    |                                                          ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
-    |
- LL |     fn qux<'c, 'b, G: 'c + Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'c {
-    |            +++     ~~~~~~~                                                           ++++
160 
161 error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:62:77
163    |
163    |
- LL |       fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
- LL | |
- LL | |
- LL | |         move || {
- LL | |         move || {
- LL | |             *dest = g.get();
- LL | |         }
+ LL | /     move || {
+ LL | |         *dest = g.get();
171 LL | |     }
173    |


174 note: the parameter type `G` must be valid for the anonymous lifetime defined here...
-   --> $DIR/missing-lifetimes-in-signature.rs:62:47
176    |
176    |
- LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
-    |                                               ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:65:9: 67:10]` will meet its required lifetime bounds
-   --> $DIR/missing-lifetimes-in-signature.rs:62:77
-    |
- LL |       fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
- LL | |
- LL | |
- LL | |         move || {
- LL | |         move || {
- LL | |             *dest = g.get();
- LL | |         }
- LL | |     }
-    | |_____^
- help: consider introducing an explicit lifetime bound
-    |
- LL ~     fn qux<'c, 'b, G: 'c + Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
- LL |
- LL |
- LL |         move || {
- LL |             *dest = g.get();
- LL |         }
-  ...
+ LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
200 
200 
201 error[E0621]: explicit lifetime required in the type of `dest`
-   --> $DIR/missing-lifetimes-in-signature.rs:72:45
203    |
203    |
- LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
-    |                                  ------     ^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'a` required
-    |                                  |
-    |                                  help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
+ LL |   fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
+    |                                    ------ help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
+ LL | /     move || {
+ LL | /     move || {
+ LL | |         *dest = g.get();
+ LL | |     }
+    | |_____^ lifetime `'a` required
208 
209 error[E0309]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:83:44
211    |
211    |
- LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
-    |            -                               ^^^^^^^^^^^^^^^^^^ ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:89:5: 91:6]` will meet its required lifetime bounds
-    |            |
-    |            help: consider adding an explicit lifetime bound...: `G: 'a`
- 
- error[E0309]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:87:1
-    |
- LL |   fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
-    |              - help: consider adding an explicit lifetime bound...: `G: 'a`
- LL | / {
- LL | |
- LL | |     move || {
+ LL | /     move || {
+ LL | /     move || {
226 LL | |         *dest = g.get();
227 LL | |     }
- LL | | }
-    | |_^ ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:89:5: 91:6]` will meet its required lifetime bounds
+    |
+    |
+    = help: consider adding an explicit lifetime bound `G: 'a`...
- error: aborting due to 11 previous errors
+ error: aborting due to 8 previous errors
232 
233 Some errors have detailed explanations: E0261, E0309, E0621, E0700.
233 Some errors have detailed explanations: E0261, E0309, E0621, E0700.
234 For more information about an error, try `rustc --explain E0261`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll/missing-lifetimes-in-signature.nll.stderr
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0261]: use of undeclared lifetime name `'a`
   |
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`

error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL |   fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                            ------ hidden type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:19:5: 22:6]` captures the anonymous lifetime defined here
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR hidden type for `impl Trait` captures lifetime
LL | |         *dest = g.get();
LL | |     }
   |
   |
help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /         move || {
LL | /         move || {
LL | |             *dest = g.get();
LL | |         }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {


error[E0311]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         *dest = g.get();
LL | |     }
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a


error[E0621]: explicit lifetime required in the type of `dest`
   |
   |
LL |   fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                    ------ help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
LL | /     move || {
LL | /     move || {
LL | |         *dest = g.get();
LL | |     }
   | |_____^ lifetime `'a` required

error[E0309]: the parameter type `G` may not live long enough
   |
LL | /     move || {
LL | /     move || {
LL | |         *dest = g.get();
LL | |     }
   |
   |
   = help: consider adding an explicit lifetime bound `G: 'a`...
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0261, E0309, E0621, E0700.
For more information about an error, try `rustc --explain E0261`.
