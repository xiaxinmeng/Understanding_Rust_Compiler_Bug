plain
.................................................................................................... 4100/12114
.................................................................................................... 4200/12114
.................................................................................................... 4300/12114
.................................................................................................... 4400/12114
...F..............F.............................F................................................... 4500/12114
.......................................................................i............................ 4700/12114
.................................................................................................... 4800/12114
.................................................................................................... 4900/12114
.................................................................................................... 5000/12114
---
---- [ui] ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs stdout ----
diff of stderr:

97    |
98 LL | fn bal() -> Box<dyn Trait> {
99 LL |     if true {
- LL |         return Box::new(Struct);
+ LL |         return box (Struct);
- LL |     Box::new(42)
+ LL |     box (42)
103    |
104 
104 
105 error[E0308]: `if` and `else` have incompatible types
128    |
128    |
129 LL | fn bax() -> Box<dyn Trait> {
130 LL |     if true {
- LL |         Box::new(Struct)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |         box (Struct)
- LL |         Box::new(42)
+ LL |         box (42)
134    |
135 
---
To only update this specific test, also pass `--test-args impl-trait/dyn-trait-return-should-be-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:7:35
   |
LL | fn fuz() -> (usize, Trait) { (42, Struct) }
   |                                   ^^^^^^ expected trait object `dyn Trait`, found struct `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
   |
   |
LL | fn fuz() -> (usize, Trait) { (42, Struct) }
   |             ^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
   |             doesn't have a size known at compile-time
   |
   |
   = help: within `(usize, (dyn Trait + 'static))`, the trait `Sized` is not implemented for `(dyn Trait + 'static)`
   = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
   = note: the return type of a function must have a statically known size
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:10:39
   |
   |
LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
   |                                       ^^^^^^ expected trait object `dyn Trait`, found struct `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
   |
   |
LL | fn bar() -> (usize, dyn Trait) { (42, Struct) }
   |             ^^^^^^^^^^^^^^^^^^   ------------ this returned value is of type `(usize, (dyn Trait + 'static))`
   |             doesn't have a size known at compile-time
   |
   |
   = help: within `(usize, (dyn Trait + 'static))`, the trait `Sized` is not implemented for `(dyn Trait + 'static)`
   = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
   = note: the return type of a function must have a statically known size

error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bap() -> Trait { Struct }
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
   |
LL | fn bap() -> impl Trait { Struct }


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn ban() -> dyn Trait { Struct }
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `Struct`, which implements `Trait`
   |
LL | fn ban() -> impl Trait { Struct }


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0746
   |
   |
help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
   |
LL | fn bak() -> T { unimplemented!() } //~ ERROR E0746
   |             ^
help: use `impl Trait` as the return type if all return paths have the same type but you want to expose only the trait in the signature
   |
LL | fn bak() -> impl Trait { unimplemented!() } //~ ERROR E0746
   |             ^^^^^^^^^^
help: use a boxed trait object if all return paths implement trait `Trait`
   |
LL | fn bak() -> Box<dyn Trait> { unimplemented!() } //~ ERROR E0746


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bal() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl Trait` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn bal() -> Box<dyn Trait> { //~ ERROR E0746
LL |     if true {
LL |         return box (Struct);
LL |     box (42)
   |


error[E0308]: `if` and `else` have incompatible types
   |
LL | /     if true {
LL | |         Struct
   | |         ------ expected because of this
   | |         ------ expected because of this
LL | |     } else {
LL | |         42 //~ ERROR `if` and `else` have incompatible types
   | |         ^^ expected struct `Struct`, found integer
LL | |     }
   | |_____- `if` and `else` have incompatible types

error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bax() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl Trait` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn bax() -> Box<dyn Trait> { //~ ERROR E0746
LL |         box (Struct)
LL |     } else {
LL |     } else {
LL |         box (42) //~ ERROR `if` and `else` have incompatible types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:34:16
   |
   |
LL | fn bam() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         return Struct; //~ ERROR mismatched types
   |                ^^^^^^ expected struct `Box`, found struct `Struct`
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
              found struct `Struct`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         return Box::new(Struct); //~ ERROR mismatched types
   |                ^^^^^^^^^      ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:36:5
   |
   |
LL | fn bam() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |     42 //~ ERROR mismatched types
   |     ^^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |     Box::new(42) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:40:16
   |
   |
LL | fn baq() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         return 0; //~ ERROR mismatched types
   |                ^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         return Box::new(0); //~ ERROR mismatched types
   |                ^^^^^^^^^ ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:42:5
   |
   |
LL | fn baq() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |     42 //~ ERROR mismatched types
   |     ^^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |     Box::new(42) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:46:9
   |
   |
LL | fn baz() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         Struct //~ ERROR mismatched types
   |         ^^^^^^ expected struct `Box`, found struct `Struct`
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
              found struct `Struct`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         Box::new(Struct) //~ ERROR mismatched types
   |         ^^^^^^^^^      ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:48:9
   |
   |
LL | fn baz() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |         42 //~ ERROR mismatched types
   |         ^^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         Box::new(42) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:53:9
   |
   |
LL | fn baw() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
LL |     if true {
LL |         0 //~ ERROR mismatched types
   |         ^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         Box::new(0) //~ ERROR mismatched types
   |         ^^^^^^^^^ ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/dyn-trait-return-should-be-impl-trait.rs:55:9
   |
   |
LL | fn baw() -> Box<dyn Trait> {
   |             -------------- expected `Box<(dyn Trait + 'static)>` because of return type
...
LL |         42 //~ ERROR mismatched types
   |         ^^ expected struct `Box`, found integer
   |
   = note: expected struct `Box<(dyn Trait + 'static)>`
                found type `{integer}`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
LL |         Box::new(42) //~ ERROR mismatched types


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bat() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
   |
LL | fn bat() -> impl Trait { //~ ERROR E0746


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn bay() -> dyn Trait { //~ ERROR E0746
   |
   |
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Trait` as the return type, as all return paths are of type `{integer}`, which implements `Trait`
   |
LL | fn bay() -> impl Trait { //~ ERROR E0746

error: aborting due to 20 previous errors

Some errors have detailed explanations: E0277, E0308, E0746.
---

---- [ui] ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait.rs stdout ----
diff of stderr:

38    |             ^^^^^^^           ^
39 help: if you change the return type to expect trait objects, box the returned expressions
40    |
- LL |         return Box::new(A);
+ LL |         return box (A);
- LL |     Box::new(B)
+ LL |     box (B)
44    |
45 
---
To only update this specific test, also pass `--test-args impl-trait/object-unsafe-trait-in-return-position-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait.rs:36:5
   |
LL | fn can() -> impl NotObjectSafe {
   |             ------------------ expected because this return type...
LL |         return A;
LL |         return A;
   |                - ...is found to be `A` here
LL |     }
LL |     B //~ ERROR mismatched types
   |     ^ expected struct `A`, found struct `B`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = help: if the trait `NotObjectSafe` were object safe, you could return a boxed trait object
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/object-unsafe-trait-in-return-position-impl-trait.rs:43:5
   |
   |
LL | fn cat() -> impl ObjectSafe {
   |             --------------- expected because this return type...
LL |         return A;
LL |         return A;
   |                - ...is found to be `A` here
LL |     }
LL |     B //~ ERROR mismatched types
   |     ^ expected struct `A`, found struct `B`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
   |
LL | fn cat() -> Box<dyn ObjectSafe> {
   |             ^^^^^^^           ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         return box (A);
LL |     }
LL |     box (B) //~ ERROR mismatched types

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs stdout ----
diff of stderr:

20    |             ^^^^^^^                  ^
21 help: if you change the return type to expect trait objects, box the returned expressions
22    |
- LL |         return Box::new(0i32);
+ LL |         return box (0i32);
- LL |     Box::new(1u32)
+ LL |     box (1u32)
26    |
27 
27 
28 error[E0308]: mismatched types

47    |             ^^^^^^^                  ^
48 help: if you change the return type to expect trait objects, box the returned expressions
49    |
- LL |         return Box::new(0i32);
+ LL |         return box (0i32);
51 LL |     } else {
- LL |         return Box::new(1u32);
+ LL |         return box (1u32);
54 
55 error[E0308]: mismatched types

74    |             ^^^^^^^                  ^
74    |             ^^^^^^^                  ^
75 help: if you change the return type to expect trait objects, box the returned expressions
76    |
- LL |         return Box::new(0i32);
+ LL |         return box (0i32);
- LL |         Box::new(1u32)
+ LL |         box (1u32)
80    |
81 
81 
82 error[E0308]: `if` and `else` have incompatible types
97    |             ^^^^^^^                  ^
97    |             ^^^^^^^                  ^
98 help: if you change the return type to expect trait objects, box the returned expressions
- LL |         Box::new(0i32)
+ LL |         box (0i32)
101 LL |     } else {
- LL |         Box::new(1u32)
- LL |         Box::new(1u32)
+ LL |         box (1u32)
103    |
104 
105 error[E0308]: mismatched types

123    |             ^^^^^^^                  ^
124 help: if you change the return type to expect trait objects, box the returned expressions
125    |
- LL |         0 => return Box::new(0i32),
- LL |         _ => Box::new(1u32),
+ LL |         0 => return box (0i32),
+ LL |         _ => box (1u32),
129 
130 error[E0308]: mismatched types

150    |             ^^^^^^^                  ^
150    |             ^^^^^^^                  ^
151 help: if you change the return type to expect trait objects, box the returned expressions
- LL |     Box::new(match 13 {
- LL |     Box::new(match 13 {
- LL |         0 => return Box::new(0i32),
+ LL |     box (match 13 {
+ LL |         0 => return box (0i32),
155 LL |         1 => 1u32,
156 LL |         _ => 2u32,

179    |             ^^^^^^^                  ^
179    |             ^^^^^^^                  ^
180 help: if you change the return type to expect trait objects, box the returned expressions
181    |
- LL |             return Box::new(0i32);
+ LL |             return box (0i32);
184 LL |         _ => {
- LL |             Box::new(1u32)
+ LL |             box (1u32)
186    |
186    |
187 
188 error[E0308]: `match` arms have incompatible types
203    |             ^^^^^^^                  ^
203    |             ^^^^^^^                  ^
204 help: if you change the return type to expect trait objects, box the returned expressions
205    |
- LL |         0 => Box::new(0i32),
- LL |         1 => Box::new(1u32),
+ LL |         0 => box (0i32),
+ LL |         1 => box (1u32),
209 
209 
210 error[E0308]: `if` and `else` have incompatible types
225    |             ^^^^^^^                  ^
225    |             ^^^^^^^                  ^
226 help: if you change the return type to expect trait objects, box the returned expressions
- LL |         Box::new(0i32)
+ LL |         box (0i32)
229 LL |     } else {
- LL |         Box::new(1u32)
- LL |         Box::new(1u32)
+ LL |         box (1u32)
231    |
232 
233 error[E0746]: return type cannot have an unboxed trait object

245 LL | fn hat() -> Box<dyn std::fmt::Display> {
246 LL |     match 13 {
247 LL |         0 => {
- LL |             return Box::new(0i32);
+ LL |             return box (0i32);
250 LL |         _ => {
251  ...

276    |
276    |
277 LL | fn pug() -> Box<dyn std::fmt::Display> {
278 LL |     match 13 {
- LL |         0 => Box::new(0i32),
- LL |         1 => Box::new(1u32),
- LL |         _ => Box::new(2u32),
+ LL |         0 => box (0i32),
+ LL |         1 => box (1u32),
+ LL |         _ => box (2u32),
283 
283 
284 error[E0308]: `if` and `else` have incompatible types
307    |
307    |
308 LL | fn man() -> Box<dyn std::fmt::Display> {
309 LL |     if false {
- LL |         Box::new(0i32)
+ LL |         box (0i32)
- LL |         Box::new(1u32)
+ LL |         box (1u32)
313    |
314 
---
To only update this specific test, also pass `--test-args impl-trait/point-to-type-err-cause-on-impl-trait-return.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:5:5
   |
LL | fn foo() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     if false {
LL |         return 0i32;
   |                ---- ...is found to be `i32` here
LL |     }
LL |     1u32 //~ ERROR mismatched types
   |     ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
LL | fn foo() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         return box (0i32);
LL |     }
LL |     box (1u32) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:12:16
   |
   |
LL | fn bar() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     if false {
LL |         return 0i32;
   |                ---- ...is found to be `i32` here
LL |     } else {
LL |         return 1u32; //~ ERROR mismatched types
   |                ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
LL | fn bar() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         return box (0i32);
LL |     } else {
LL |         return box (1u32); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:20:9
   |
   |
LL | fn baz() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     if false {
LL |         return 0i32;
   |                ---- ...is found to be `i32` here
LL |     } else {
LL |         1u32 //~ ERROR mismatched types
   |         ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
   |
LL | fn baz() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         return box (0i32);
LL |     } else {
LL |         box (1u32) //~ ERROR mismatched types


error[E0308]: `if` and `else` have incompatible types
   |
LL | /     if false {
LL | |         0i32
   | |         ---- expected because of this
   | |         ---- expected because of this
LL | |     } else {
LL | |         1u32 //~ ERROR `if` and `else` have incompatible types
   | |         ^^^^ expected `i32`, found `u32`
LL | |     }
   | |_____- `if` and `else` have incompatible types
   |
help: you could change the return type to be a boxed trait object
   |
LL | fn qux() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
LL |         box (0i32)
LL |     } else {
LL |     } else {
LL |         box (1u32) //~ ERROR `if` and `else` have incompatible types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:35:14
   |
   |
LL | fn bat() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     match 13 {
LL |         0 => return 0i32,
   |                     ---- ...is found to be `i32` here
LL |         _ => 1u32, //~ ERROR mismatched types
   |              ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
   |
LL | fn bat() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         0 => return box (0i32),
LL |         _ => box (1u32), //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:40:5
   |
   |
LL |   fn can() -> impl std::fmt::Display {
   |               ---------------------- expected because this return type...
LL | /     match 13 { //~ ERROR mismatched types
LL | |         0 => return 0i32,
   | |                     ---- ...is found to be `i32` here
LL | |         1 => 1u32,
LL | |         _ => 2u32,
LL | |     }
   | |_____^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
   |
LL | fn can() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |     box (match 13 { //~ ERROR mismatched types
LL |         0 => return box (0i32),
LL |         1 => 1u32,
LL |         _ => 2u32,
   |

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:53:13
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:53:13
   |
LL | fn cat() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |             return 0i32;
LL |             return 0i32;
   |                    ---- ...is found to be `i32` here
...
LL |             1u32 //~ ERROR mismatched types
   |             ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
   |
LL | fn cat() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |             return box (0i32);
LL |         _ => {
LL |         _ => {
LL |             box (1u32) //~ ERROR mismatched types


error[E0308]: `match` arms have incompatible types
   |
LL | /     match 13 {
LL | |         0 => 0i32,
   | |              ---- this is found to be of type `i32`
   | |              ---- this is found to be of type `i32`
LL | |         1 => 1u32, //~ ERROR `match` arms have incompatible types
   | |              ^^^^ expected `i32`, found `u32`
LL | |         _ => 2u32,
LL | |     }
   | |_____- `match` arms have incompatible types
   |
help: you could change the return type to be a boxed trait object
   |
LL | fn dog() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         0 => box (0i32),
LL |         1 => box (1u32), //~ ERROR `match` arms have incompatible types


error[E0308]: `if` and `else` have incompatible types
   |
   |
LL | /     if let Some(42) = Some(42) {
LL | |         0i32
LL | |     } else {
LL | |     } else {
LL | |         1u32 //~ ERROR `if` and `else` have incompatible types
   | |         ^^^^ expected `i32`, found `u32`
LL | |     }
   | |_____- `if` and `else` have incompatible types
   |
help: you could change the return type to be a boxed trait object
   |
LL | fn apt() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
LL |         box (0i32)
LL |     } else {
LL |     } else {
LL |         box (1u32) //~ ERROR `if` and `else` have incompatible types


error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn hat() -> dyn std::fmt::Display { //~ ERROR return type cannot have an unboxed trait object
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl std::fmt::Display` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn hat() -> Box<dyn std::fmt::Display> { //~ ERROR return type cannot have an unboxed trait object
LL |     match 13 {
LL |         0 => {
LL |             return box (0i32);
LL |         _ => {
 ...


error[E0308]: `match` arms have incompatible types
   |
LL | /     match 13 {
LL | |         0 => 0i32,
   | |              ---- this is found to be of type `i32`
   | |              ---- this is found to be of type `i32`
LL | |         1 => 1u32, //~ ERROR `match` arms have incompatible types
   | |              ^^^^ expected `i32`, found `u32`
LL | |         _ => 2u32,
LL | |     }
   | |_____- `match` arms have incompatible types

error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn pug() -> dyn std::fmt::Display { //~ ERROR return type cannot have an unboxed trait object
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl std::fmt::Display` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn pug() -> Box<dyn std::fmt::Display> { //~ ERROR return type cannot have an unboxed trait object
LL |     match 13 {
LL |         0 => box (0i32),
LL |         1 => box (1u32), //~ ERROR `match` arms have incompatible types
LL |         _ => box (2u32),


error[E0308]: `if` and `else` have incompatible types
   |
LL | /     if false {
LL | |         0i32
   | |         ---- expected because of this
   | |         ---- expected because of this
LL | |     } else {
LL | |         1u32 //~ ERROR `if` and `else` have incompatible types
   | |         ^^^^ expected `i32`, found `u32`
LL | |     }
   | |_____- `if` and `else` have incompatible types

error[E0746]: return type cannot have an unboxed trait object
   |
   |
LL | fn man() -> dyn std::fmt::Display { //~ ERROR return type cannot have an unboxed trait object
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl std::fmt::Display` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn man() -> Box<dyn std::fmt::Display> { //~ ERROR return type cannot have an unboxed trait object
LL |     if false {
LL |         box (0i32)
LL |     } else {
LL |         box (1u32) //~ ERROR `if` and `else` have incompatible types

error: aborting due to 14 previous errors

Some errors have detailed explanations: E0308, E0746.
---
---- [ui] ui/suggestions/match-prev-arm-needing-semi.rs stdout ----
diff of stderr:

61    |                                ^^^^^^
62 help: consider removing this semicolon and boxing the expressions
63    |
- LL |             Box::new(async_dummy())
+ LL |             box (async_dummy())
66 LL |         }
66 LL |         }
- LL |         false => Box::new(async_dummy2()),
+ LL |         false => box (async_dummy2()),
69 
69 
70 error[E0308]: `match` arms have incompatible types

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/match-prev-arm-needing-semi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy(), //~ ERROR `match` arms have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn async_dummy() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                        ^ checked the `Output` of this `async fn`, found opaque type
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy().await, //~ ERROR `match` arms have incompatible types
help: consider removing this semicolon
   |
   |
LL |             async_dummy() //~ NOTE this is found to be


error[E0308]: `match` arms have incompatible types
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn async_dummy2() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                         ^ checked the `Output` of this `async fn`, found opaque type
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types
   |                                ^^^^^^
help: consider removing this semicolon and boxing the expressions
   |
LL |             box (async_dummy()) //~ NOTE this is found to be
LL |             //~^ HELP consider removing this semicolon
LL |         }
LL |         false => box (async_dummy2()), //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
   |  _____________-
LL | |         true => async_dummy(), //~ NOTE this is found to be
   | |                 ------------- this is found to be of type `impl Future`
LL | |         //~| HELP consider `await`ing on both `Future`s
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
...  |
LL | |         //~| NOTE distinct uses of `impl Trait` result in different opaque types
LL | |     };
   | |_____- `match` arms have incompatible types
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn async_dummy2() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                         ^ checked the `Output` of this `async fn`, found opaque type
   = note:     expected type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:16:24>)
           found opaque type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:19:25>)
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
LL |         true => async_dummy().await, //~ NOTE this is found to be
LL |         //~| HELP consider `await`ing on both `Future`s
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             dummy(); //~ NOTE this is found to be
   | |             |      |
   | |             |      help: consider removing this semicolon
   | |             this is found to be of type `()`
   | |             this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => dummy(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^ expected `()`, found `i32`
LL | |         //~^ NOTE expected `()`, found `i32`
LL | |     };
   | |_____- `match` arms have incompatible types
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12007 passed; 4 failed; 103 ignored; 0 measured; 0 filtered out; finished in 126.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:01
