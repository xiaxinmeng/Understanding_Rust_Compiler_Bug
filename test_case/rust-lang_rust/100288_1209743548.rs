plain
diff of stderr:

2   --> $DIR/match-no-arms-unreachable-after.rs:8:5
3    |
4 LL |     match v { }
-    |           - any code following this expression is unreachable
+    |           - this expression has type `Void`, which is uninhabited
6 LL |     let x = 2;
7    |     ^^^^^^^^^^ unreachable statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/match-no-arms-unreachable-after.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/match-no-arms-unreachable-after.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/match-no-arms-unreachable-after.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-no-arms-unreachable-after.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/match/match-no-arms-unreachable-after.rs:8:5
   |
LL |     match v { }
LL |     match v { }
   |           - this expression has type `Void`, which is uninhabited
LL |     let x = 2; //~ ERROR unreachable
   |     ^^^^^^^^^^ unreachable statement
note: the lint level is defined here
  --> /checkout/src/test/ui/match/match-no-arms-unreachable-after.rs:2:9
   |
LL | #![deny(unreachable_code)]
---

---- [ui] src/test/ui/repr/repr-transparent.rs stdout ----
diff of stderr:

42 LL | enum Void {}
43    | --------- zero-variant enum
44 
+ error[E0731]: transparent enum needs exactly one variant, but has 0
+    |
+    |
+ LL | enum Void {}
+    | ^^^^^^^^^ needs exactly one variant, but has 0
+ 
45 error[E0690]: the variant of a transparent enum needs at most one non-zero-sized field, but has 2
47    |

52    |         |
53    |         this field is non-zero-sized
53    |         this field is non-zero-sized
54 
+ error[E0690]: the variant of a transparent enum needs at most one non-zero-sized field, but has 2
+    |
+    |
+ LL | enum TooManyFieldsEnum {
+    | ^^^^^^^^^^^^^^^^^^^^^^ needs at most one non-zero-sized field, but has 2
+ LL |     Foo(u32, String),
+    |         |
+    |         this field is non-zero-sized
+ 
+ 
55 error[E0731]: transparent enum needs exactly one variant, but has 2
57    |

62 LL |     Bar,
62 LL |     Bar,
63    |     --- too many variants in `MultipleVariants`
64 
+ error[E0731]: transparent enum needs exactly one variant, but has 2
+    |
+    |
+ LL | enum MultipleVariants {
+    | ^^^^^^^^^^^^^^^^^^^^^ needs exactly one variant, but has 2
+ LL |     Foo(String),
+ LL |     Bar,
+ LL |     Bar,
+    |     --- too many variants in `MultipleVariants`
+ 
65 error[E0691]: zero-sized field in transparent enum has alignment larger than 1
67    |

69    |              ^^^^^^^^ has alignment larger than 1
70 
70 
71 error[E0691]: zero-sized field in transparent enum has alignment larger than 1
+    |
+    |
+ LL |     Foo(u32, [u16; 0]),
+    |              ^^^^^^^^ has alignment larger than 1
+ 
+ error[E0691]: zero-sized field in transparent enum has alignment larger than 1
73    |
73    |
74 LL |     Foo { bar: ZstAlign32<T>, baz: u32 }
75    |           ^^^^^^^^^^^^^^^^^^ has alignment larger than 1
76 
76 
+ error[E0691]: zero-sized field in transparent enum has alignment larger than 1
+    |
+    |
+ LL |     Foo { bar: ZstAlign32<T>, baz: u32 }
+    |           ^^^^^^^^^^^^^^^^^^ has alignment larger than 1
77 error[E0690]: transparent union needs at most one non-zero-sized field, but has 2
78   --> $DIR/repr-transparent.rs:85:1
79    |

---
To only update this specific test, also pass `--test-args repr/repr-transparent.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-transparent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-transparent" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-transparent/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0690]: transparent struct needs at most one non-zero-sized field, but has 2
   |
   |
LL | struct MultipleNonZst(u8, u8); //~ ERROR needs at most one non-zero-sized field
   | |                     |
   | |                     this field is non-zero-sized
   | |                     this field is non-zero-sized
   | needs at most one non-zero-sized field, but has 2
error[E0690]: transparent struct needs at most one non-zero-sized field, but has 2
  --> /checkout/src/test/ui/repr/repr-transparent.rs:32:1
   |
   |
LL | pub struct StructWithProjection(f32, <f32 as Mirror>::It);
   | |                               |
   | |                               this field is non-zero-sized
   | |                               this field is non-zero-sized
   | needs at most one non-zero-sized field, but has 2

error[E0691]: zero-sized field in transparent struct has alignment larger than 1
   |
   |
LL | struct NontrivialAlignZst(u32, [u16; 0]); //~ ERROR alignment larger than 1
   |                                ^^^^^^^^ has alignment larger than 1

error[E0691]: zero-sized field in transparent struct has alignment larger than 1
   |
   |
LL | struct GenericAlign<T>(ZstAlign32<T>, u32); //~ ERROR alignment larger than 1
   |                        ^^^^^^^^^^^^^ has alignment larger than 1

error[E0731]: transparent enum needs exactly one variant, but has 0
   |
   |
LL | enum Void {} //~ ERROR transparent enum needs exactly one variant, but has 0
   | ^^^^^^^^^ needs exactly one variant, but has 0
error[E0084]: unsupported representation for zero-variant enum
  --> /checkout/src/test/ui/repr/repr-transparent.rs:44:1
   |
   |
LL | #[repr(transparent)] //~ ERROR unsupported representation for zero-variant enum
   | ^^^^^^^^^^^^^^^^^^^^
LL | enum Void {} //~ ERROR transparent enum needs exactly one variant, but has 0
   | --------- zero-variant enum

error[E0731]: transparent enum needs exactly one variant, but has 0
   |
   |
LL | enum Void {} //~ ERROR transparent enum needs exactly one variant, but has 0
   | ^^^^^^^^^ needs exactly one variant, but has 0

error[E0690]: the variant of a transparent enum needs at most one non-zero-sized field, but has 2
   |
   |
LL | enum TooManyFieldsEnum {
   | ^^^^^^^^^^^^^^^^^^^^^^ needs at most one non-zero-sized field, but has 2
LL |     Foo(u32, String),
   |         |
   |         this field is non-zero-sized


error[E0690]: the variant of a transparent enum needs at most one non-zero-sized field, but has 2
   |
   |
LL | enum TooManyFieldsEnum {
   | ^^^^^^^^^^^^^^^^^^^^^^ needs at most one non-zero-sized field, but has 2
LL |     Foo(u32, String),
   |         |
   |         this field is non-zero-sized


error[E0731]: transparent enum needs exactly one variant, but has 2
   |
   |
LL | enum MultipleVariants { //~ ERROR transparent enum needs exactly one variant, but has 2
   | ^^^^^^^^^^^^^^^^^^^^^ needs exactly one variant, but has 2
LL |     Foo(String),
LL |     Bar,
LL |     Bar,
   |     --- too many variants in `MultipleVariants`

error[E0731]: transparent enum needs exactly one variant, but has 2
   |
   |
LL | enum MultipleVariants { //~ ERROR transparent enum needs exactly one variant, but has 2
   | ^^^^^^^^^^^^^^^^^^^^^ needs exactly one variant, but has 2
LL |     Foo(String),
LL |     Bar,
LL |     Bar,
   |     --- too many variants in `MultipleVariants`

error[E0691]: zero-sized field in transparent enum has alignment larger than 1
   |
   |
LL |     Foo(u32, [u16; 0]), //~ ERROR alignment larger than 1
   |              ^^^^^^^^ has alignment larger than 1

error[E0691]: zero-sized field in transparent enum has alignment larger than 1
   |
   |
LL |     Foo(u32, [u16; 0]), //~ ERROR alignment larger than 1
   |              ^^^^^^^^ has alignment larger than 1

error[E0691]: zero-sized field in transparent enum has alignment larger than 1
   |
   |
LL |     Foo { bar: ZstAlign32<T>, baz: u32 } //~ ERROR alignment larger than 1
   |           ^^^^^^^^^^^^^^^^^^ has alignment larger than 1

error[E0691]: zero-sized field in transparent enum has alignment larger than 1
   |
   |
LL |     Foo { bar: ZstAlign32<T>, baz: u32 } //~ ERROR alignment larger than 1
   |           ^^^^^^^^^^^^^^^^^^ has alignment larger than 1
error[E0690]: transparent union needs at most one non-zero-sized field, but has 2
  --> /checkout/src/test/ui/repr/repr-transparent.rs:85:1
   |
   |
LL | union TooManyFields { //~ ERROR transparent union needs at most one non-zero-sized field, but has 2
   | ^^^^^^^^^^^^^^^^^^^ needs at most one non-zero-sized field, but has 2
LL |     u: u32,
LL |     s: i32
   |     ------ this field is non-zero-sized

error: aborting due to 16 previous errors
