plain
---- [ui] src/test/ui/destructure-trait-ref.rs stdout ----
diff of stderr:

30    |
31 LL -     let &&x = &1isize as &dyn T;
32 LL +     let &x = &1isize as &dyn T;
+    |
34 
35 error[E0308]: mismatched types
36   --> $DIR/destructure-trait-ref.rs:36:11
36   --> $DIR/destructure-trait-ref.rs:36:11

46    |
47 LL -     let &&&x = &(&1isize as &dyn T);
48 LL +     let &&x = &(&1isize as &dyn T);
+    |
50 
51 error[E0308]: mismatched types
52   --> $DIR/destructure-trait-ref.rs:40:13
---
To only update this specific test, also pass `--test-args destructure-trait-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructure-trait-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0033]: type `&dyn T` cannot be dereferenced
   |
   |
LL |     let &x = &1isize as &dyn T;      //~ ERROR type `&dyn T` cannot be dereferenced
   |         ^^ type `&dyn T` cannot be dereferenced

error[E0033]: type `&dyn T` cannot be dereferenced
   |
   |
LL |     let &&x = &(&1isize as &dyn T);  //~ ERROR type `&dyn T` cannot be dereferenced
   |          ^^ type `&dyn T` cannot be dereferenced

error[E0033]: type `Box<dyn T>` cannot be dereferenced
   |
   |
LL |     let box x = Box::new(1isize) as Box<dyn T>;
   |         ^^^^^ type `Box<dyn T>` cannot be dereferenced
error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:32:10
   |
   |
LL |     let &&x = &1isize as &dyn T;
   |          ^^   ----------------- this expression has type `&dyn T`
   |          expected trait object `dyn T`, found reference
   |
   = note: expected trait object `dyn T`
                 found reference `&_`
                 found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let &&x = &1isize as &dyn T;
LL +     let &x = &1isize as &dyn T;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:36:11
   |
   |
LL |     let &&&x = &(&1isize as &dyn T);
   |           ^^   -------------------- this expression has type `&&dyn T`
   |           expected trait object `dyn T`, found reference
   |
   = note: expected trait object `dyn T`
                 found reference `&_`
                 found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let &&&x = &(&1isize as &dyn T);
LL +     let &&x = &(&1isize as &dyn T);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:40:13
   |
   |
LL |     let box box x = Box::new(1isize) as Box<dyn T>;
   |             ^^^^^   ------------------------------ this expression has type `Box<dyn T>`
   |             |
   |             expected trait object `dyn T`, found struct `Box`
   = note: expected trait object `dyn T`
   = note: expected trait object `dyn T`
                    found struct `Box<_>`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0033, E0308.
For more information about an error, try `rustc --explain E0033`.
For more information about an error, try `rustc --explain E0033`.
------------------------------------------


---- [ui] src/test/ui/mismatched_types/issue-38371.rs stdout ----
diff of stderr:

12    |
13 LL - fn foo(&_a: Foo) {}
14 LL + fn foo(_a: &Foo) {}
+    |
16 
17 error[E0308]: mismatched types
18   --> $DIR/issue-38371.rs:16:9
18   --> $DIR/issue-38371.rs:16:9

28    |
29 LL - fn agh(&&_a: &u32) {}
30 LL + fn agh(&_a: &u32) {}
+    |
32 
33 error: aborting due to 2 previous errors
34 
---
To only update this specific test, also pass `--test-args mismatched_types/issue-38371.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-38371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-38371" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-38371/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/issue-38371.rs:8:8
   |
   |
LL | fn foo(&_a: Foo) {} //~ ERROR mismatched types
   |        ^^^  --- expected due to this
   |        expected struct `Foo`, found reference
   |
   = note: expected struct `Foo`
           found reference `&_`
           found reference `&_`
help: to take parameter `_a` by reference, move `&` to the type
   |
LL - fn foo(&_a: Foo) {} //~ ERROR mismatched types
LL + fn foo(_a: &Foo) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/issue-38371.rs:16:9
   |
   |
LL | fn agh(&&_a: &u32) {} //~ ERROR mismatched types
   |         ^^^  ---- expected due to this
   |         expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL - fn agh(&&_a: &u32) {} //~ ERROR mismatched types
LL + fn agh(&_a: &u32) {} //~ ERROR mismatched types

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/mismatched_types/ref-pat-suggestions.rs stdout ----
diff of stderr:

12    |
13 LL - fn _f0(&_a: u32) {}
14 LL + fn _f0(_a: &u32) {}
+    |
16 
17 error[E0308]: mismatched types
18   --> $DIR/ref-pat-suggestions.rs:4:8
18   --> $DIR/ref-pat-suggestions.rs:4:8

28    |
29 LL - fn _f1(&mut _a: u32) {}
30 LL + fn _f1(_a: &mut u32) {}
+    |
32 
33 error[E0308]: mismatched types
34   --> $DIR/ref-pat-suggestions.rs:5:9
34   --> $DIR/ref-pat-suggestions.rs:5:9

44    |
45 LL - fn _f2(&&_a: &u32) {}
46 LL + fn _f2(&_a: &u32) {}
+    |
48 
49 error[E0308]: mismatched types
50   --> $DIR/ref-pat-suggestions.rs:6:13
50   --> $DIR/ref-pat-suggestions.rs:6:13

60    |
61 LL - fn _f3(&mut &_a: &mut u32) {}
62 LL + fn _f3(&mut _a: &mut u32) {}
+    |
64 
65 error[E0308]: mismatched types
66   --> $DIR/ref-pat-suggestions.rs:7:9
66   --> $DIR/ref-pat-suggestions.rs:7:9

76    |
77 LL - fn _f4(&&mut _a: &u32) {}
78 LL + fn _f4(&_a: &u32) {}
+    |
80 
81 error[E0308]: mismatched types
82   --> $DIR/ref-pat-suggestions.rs:8:13
82   --> $DIR/ref-pat-suggestions.rs:8:13

92    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
93 LL - fn _f5(&mut &mut _a: &mut u32) {}
94 LL + fn _f5(&mut _a: &mut u32) {}
+    |
96 
97 error[E0308]: mismatched types
98   --> $DIR/ref-pat-suggestions.rs:11:23
98   --> $DIR/ref-pat-suggestions.rs:11:23

109    |
110 LL -     let _: fn(u32) = |&_a| ();
111 LL +     let _: fn(u32) = |_a| ();
+    |
113 
114 error[E0308]: mismatched types
115   --> $DIR/ref-pat-suggestions.rs:12:23
115   --> $DIR/ref-pat-suggestions.rs:12:23

126    |
127 LL -     let _: fn(u32) = |&mut _a| ();
128 LL +     let _: fn(u32) = |_a| ();
+    |
130 
131 error[E0308]: mismatched types
132   --> $DIR/ref-pat-suggestions.rs:13:25
132   --> $DIR/ref-pat-suggestions.rs:13:25

143    |
144 LL -     let _: fn(&u32) = |&&_a| ();
145 LL +     let _: fn(&u32) = |&_a| ();
+    |
147 
148 error[E0308]: mismatched types
149   --> $DIR/ref-pat-suggestions.rs:14:33
149   --> $DIR/ref-pat-suggestions.rs:14:33

160    |
161 LL -     let _: fn(&mut u32) = |&mut &_a| ();
162 LL +     let _: fn(&mut u32) = |&mut _a| ();
+    |
164 
165 error[E0308]: mismatched types
166   --> $DIR/ref-pat-suggestions.rs:15:25
166   --> $DIR/ref-pat-suggestions.rs:15:25

177    |
178 LL -     let _: fn(&u32) = |&&mut _a| ();
179 LL +     let _: fn(&u32) = |&_a| ();
+    |
181 
182 error[E0308]: mismatched types
183   --> $DIR/ref-pat-suggestions.rs:16:33
183   --> $DIR/ref-pat-suggestions.rs:16:33

194    |
195 LL -     let _: fn(&mut u32) = |&mut &mut _a| ();
196 LL +     let _: fn(&mut u32) = |&mut _a| ();
+    |
198 
199 error[E0308]: mismatched types
200   --> $DIR/ref-pat-suggestions.rs:18:14
200   --> $DIR/ref-pat-suggestions.rs:18:14

210    |
211 LL -     let _ = |&_a: u32| ();
212 LL +     let _ = |_a: &u32| ();
+    |
214 
215 error[E0308]: mismatched types
216   --> $DIR/ref-pat-suggestions.rs:19:14
216   --> $DIR/ref-pat-suggestions.rs:19:14

226    |
227 LL -     let _ = |&mut _a: u32| ();
228 LL +     let _ = |_a: &mut u32| ();
+    |
230 
231 error[E0308]: mismatched types
232   --> $DIR/ref-pat-suggestions.rs:20:15
232   --> $DIR/ref-pat-suggestions.rs:20:15

242    |
243 LL -     let _ = |&&_a: &u32| ();
244 LL +     let _ = |&_a: &u32| ();
+    |
246 
247 error[E0308]: mismatched types
248   --> $DIR/ref-pat-suggestions.rs:21:19
248   --> $DIR/ref-pat-suggestions.rs:21:19

258    |
259 LL -     let _ = |&mut &_a: &mut u32| ();
260 LL +     let _ = |&mut _a: &mut u32| ();
+    |
262 
263 error[E0308]: mismatched types
264   --> $DIR/ref-pat-suggestions.rs:22:15
264   --> $DIR/ref-pat-suggestions.rs:22:15

274    |
275 LL -     let _ = |&&mut _a: &u32| ();
276 LL +     let _ = |&_a: &u32| ();
+    |
278 
279 error[E0308]: mismatched types
280   --> $DIR/ref-pat-suggestions.rs:23:19
280   --> $DIR/ref-pat-suggestions.rs:23:19

290    |
291 LL -     let _ = |&mut &mut _a: &mut u32| ();
292 LL +     let _ = |&mut _a: &mut u32| ();
+    |
294 
295 error: aborting due to 18 previous errors
296 
---
To only update this specific test, also pass `--test-args mismatched_types/ref-pat-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/ref-pat-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/ref-pat-suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:3:8
   |
   |
LL | fn _f0(&_a: u32) {} //~ ERROR mismatched types
   |        ^^^  --- expected due to this
   |        expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: to take parameter `_a` by reference, move `&` to the type
   |
LL - fn _f0(&_a: u32) {} //~ ERROR mismatched types
LL + fn _f0(_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:4:8
   |
   |
LL | fn _f1(&mut _a: u32) {} //~ ERROR mismatched types
   |        ^^^^^^^  --- expected due to this
   |        expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: to take parameter `_a` by reference, move `&mut` to the type
   |
LL - fn _f1(&mut _a: u32) {} //~ ERROR mismatched types
LL + fn _f1(_a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:5:9
   |
   |
LL | fn _f2(&&_a: &u32) {} //~ ERROR mismatched types
   |         ^^^  ---- expected due to this
   |         expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL - fn _f2(&&_a: &u32) {} //~ ERROR mismatched types
LL + fn _f2(&_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:6:13
   |
   |
LL | fn _f3(&mut &_a: &mut u32) {} //~ ERROR mismatched types
   |             ^^^  -------- expected due to this
   |             expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL - fn _f3(&mut &_a: &mut u32) {} //~ ERROR mismatched types
LL + fn _f3(&mut _a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:7:9
   |
   |
LL | fn _f4(&&mut _a: &u32) {} //~ ERROR mismatched types
   |         ^^^^^^^  ---- expected due to this
   |         expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL - fn _f4(&&mut _a: &u32) {} //~ ERROR mismatched types
LL + fn _f4(&_a: &u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:8:13
   |
   |
LL | fn _f5(&mut &mut _a: &mut u32) {} //~ ERROR mismatched types
   |             ^^^^^^^  -------- expected due to this
   |             expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL - fn _f5(&mut &mut _a: &mut u32) {} //~ ERROR mismatched types
LL + fn _f5(&mut _a: &mut u32) {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:11:23
   |
   |
LL |     let _: fn(u32) = |&_a| (); //~ ERROR mismatched types
   |                       ^--
   |                       |expected due to this
   |                       expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(u32) = |&_a| (); //~ ERROR mismatched types
LL +     let _: fn(u32) = |_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:12:23
   |
   |
LL |     let _: fn(u32) = |&mut _a| (); //~ ERROR mismatched types
   |                       |    |
   |                       |    expected due to this
   |                       expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(u32) = |&mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(u32) = |_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:13:25
   |
   |
LL |     let _: fn(&u32) = |&&_a| (); //~ ERROR mismatched types
   |                         ^--
   |                         |expected due to this
   |                         expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(&u32) = |&&_a| (); //~ ERROR mismatched types
LL +     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:14:33
   |
   |
LL |     let _: fn(&mut u32) = |&mut &_a| (); //~ ERROR mismatched types
   |                                 ^--
   |                                 |expected due to this
   |                                 expected `u32`, found reference
   |
   = note:   expected type `u32`
   = note:   expected type `u32`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _: fn(&mut u32) = |&mut &_a| (); //~ ERROR mismatched types
LL +     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:15:25
   |
   |
LL |     let _: fn(&u32) = |&&mut _a| (); //~ ERROR mismatched types
   |                         |    |
   |                         |    expected due to this
   |                         expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(&u32) = |&&mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(&u32) = |&_a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:16:33
   |
   |
LL |     let _: fn(&mut u32) = |&mut &mut _a| (); //~ ERROR mismatched types
   |                                 |    |
   |                                 |    expected due to this
   |                                 expected `u32`, found `&mut _`
   |
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _: fn(&mut u32) = |&mut &mut _a| (); //~ ERROR mismatched types
LL +     let _: fn(&mut u32) = |&mut _a| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:18:14
   |
   |
LL |     let _ = |&_a: u32| (); //~ ERROR mismatched types
   |              ^^^  --- expected due to this
   |              expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: to take parameter `_a` by reference, move `&` to the type
   |
LL -     let _ = |&_a: u32| (); //~ ERROR mismatched types
LL +     let _ = |_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:19:14
   |
   |
LL |     let _ = |&mut _a: u32| (); //~ ERROR mismatched types
   |              ^^^^^^^  --- expected due to this
   |              expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: to take parameter `_a` by reference, move `&mut` to the type
   |
LL -     let _ = |&mut _a: u32| (); //~ ERROR mismatched types
LL +     let _ = |_a: &mut u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:20:15
   |
   |
LL |     let _ = |&&_a: &u32| (); //~ ERROR mismatched types
   |               ^^^  ---- expected due to this
   |               expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _ = |&&_a: &u32| (); //~ ERROR mismatched types
LL +     let _ = |&_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:21:19
   |
   |
LL |     let _ = |&mut &_a: &mut u32| (); //~ ERROR mismatched types
   |                   ^^^  -------- expected due to this
   |                   expected `u32`, found reference
   |
   = note:   expected type `u32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     let _ = |&mut &_a: &mut u32| (); //~ ERROR mismatched types
LL +     let _ = |&mut _a: &mut u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:22:15
   |
   |
LL |     let _ = |&&mut _a: &u32| (); //~ ERROR mismatched types
   |               ^^^^^^^  ---- expected due to this
   |               expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _ = |&&mut _a: &u32| (); //~ ERROR mismatched types
LL +     let _ = |&_a: &u32| (); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/ref-pat-suggestions.rs:23:19
   |
   |
LL |     let _ = |&mut &mut _a: &mut u32| (); //~ ERROR mismatched types
   |                   ^^^^^^^  -------- expected due to this
   |                   expected `u32`, found `&mut _`
   |
   = note:           expected type `u32`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     let _ = |&mut &mut _a: &mut u32| (); //~ ERROR mismatched types
LL +     let _ = |&mut _a: &mut u32| (); //~ ERROR mismatched types

error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/pattern/for-loop-bad-item.rs stdout ----
diff of stderr:

12    |
13 LL -     for ((_, _), (&mut c, _)) in &mut map {
14 LL +     for ((_, _), (c, _)) in &mut map {
+    |
16 
17 error[E0308]: mismatched types
18   --> $DIR/for-loop-bad-item.rs:14:14
---
To only update this specific test, also pass `--test-args pattern/for-loop-bad-item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/for-loop-bad-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/for-loop-bad-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/for-loop-bad-item/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/for-loop-bad-item.rs:7:19
   |
   |
LL |     for ((_, _), (&mut c, _)) in &mut map {
   |                   ^^^^^^         -------- this is an iterator with items of type `(&(char, char), &mut (char, char))`
   |                   expected `char`, found `&mut _`
   |
   = note:           expected type `char`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
help: consider removing `&mut` from the pattern
   |
LL -     for ((_, _), (&mut c, _)) in &mut map {
LL +     for ((_, _), (c, _)) in &mut map {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/for-loop-bad-item.rs:14:14
   |
   |
LL |     for Some(Qux(_)) | None in [Some(""), None] {
   |              ^^^^^^            ---------------- this is an iterator with items of type `Option<&str>`
   |              |
   |              expected `str`, found struct `Qux`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/match-ergonomics.rs stdout ----
diff of stderr:

12    |
13 LL -         [&v] => {},
14 LL +         [v] => {},
+    |
16 
16 
17 error[E0529]: expected an array or slice, found `Vec<i32>`

44    |
44    |
45 LL -         &v => {},
46 LL +         v => {},
+    |
48 
49 error[E0308]: mismatched types
50   --> $DIR/match-ergonomics.rs:40:13
50   --> $DIR/match-ergonomics.rs:40:13

60    |
61 LL -     if let [&v] = &x[..] {}
62 LL +     if let [v] = &x[..] {}
+    |
64 
65 error: aborting due to 5 previous errors
66 
---
To only update this specific test, also pass `--test-args suggestions/match-ergonomics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-ergonomics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-ergonomics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-ergonomics/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:4:10
   |
   |
LL |     match &x[..] {
   |           ------ this expression has type `&[i32]`
LL |         [&v] => {}, //~ ERROR mismatched types
   |          ^^ expected `i32`, found reference
   = note:   expected type `i32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -         [&v] => {}, //~ ERROR mismatched types
LL +         [v] => {}, //~ ERROR mismatched types


error[E0529]: expected an array or slice, found `Vec<i32>`
   |
LL |     match x {
LL |     match x {
   |           - help: consider slicing here: `x[..]`
LL |         [&v] => {}, //~ ERROR expected an array or slice
   |         ^^^^ pattern cannot match with input type `Vec<i32>`

error[E0529]: expected an array or slice, found `Vec<i32>`
   |
LL |     match x {
LL |     match x {
   |           - help: consider slicing here: `x[..]`
LL |         [v] => {}, //~ ERROR expected an array or slice
   |         ^^^ pattern cannot match with input type `Vec<i32>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:29:9
   |
LL |     match y {
LL |     match y {
   |           - this expression has type `i32`
LL |         &v => {}, //~ ERROR mismatched types
   |         ^^ expected `i32`, found reference
   = note:   expected type `i32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -         &v => {}, //~ ERROR mismatched types
LL +         v => {}, //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:40:13
   |
   |
LL |     if let [&v] = &x[..] {} //~ ERROR mismatched types
   |             ^^    ------ this expression has type `&[i32]`
   |             expected `i32`, found reference
   |
   = note:   expected type `i32`
           found reference `&_`
           found reference `&_`
help: consider removing `&` from the pattern
   |
LL -     if let [&v] = &x[..] {} //~ ERROR mismatched types
LL +     if let [v] = &x[..] {} //~ ERROR mismatched types

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0308, E0529.
