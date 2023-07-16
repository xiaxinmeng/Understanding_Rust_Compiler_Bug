plain
65 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:71:42
+   --> $DIR/ub-wide-ptr.rs:71:50
67    |
68 LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─allocN─╼ ff ff ff 7f                         │ ╾──╼....
-            }
+    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:74:1
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/ub-wide-ptr.rs:74:42
+   --> $DIR/ub-wide-ptr.rs:74:42
78    |
79 LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
80    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
81 
81 
- <<<<<<< HEAD
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:77:1
- =======
86 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:74:48
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:77:48
89    |
90 LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
91    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (going beyond the bounds of its allocation)
92 
92 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:80:1
- =======
97 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:77:50
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:80:50
100    |
101 LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
102    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
103 
103 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:84:1
- =======
108 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:81:51
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:84:51
111    |
112 LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
113    |                                                   ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
114 
114 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:90:1
-    |
- LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.0: encountered 0x03, but expected a boolean
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                ╾allocN─╼                                     │ ╾──╼
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:93:1
-    |
-    |
- LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.1[0]: encountered 0x03, but expected a boolean
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                ╾allocN─╼                                     │ ╾──╼
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:100:1
- =======
- =======
141 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:87:60
+   --> $DIR/ub-wide-ptr.rs:90:60
143    |
144 LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
145    |                                                            ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
146 
147 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:90:67
+   --> $DIR/ub-wide-ptr.rs:93:67
+   --> $DIR/ub-wide-ptr.rs:93:67
149    |
150 LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
151    |                                                                   ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
152 
153 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:99:5
-   --> $DIR/ub-wide-ptr.rs:99:5
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:102:5
156    |
157 LL |     mem::transmute((42, uninit_len))
158    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
159 
159 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:108:1
- =======
164 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:105:58
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:108:58
167    |
168 LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
169    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered too small vtable
170 
170 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:111:1
- =======
175 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:108:58
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:111:58
178    |
179 LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
180    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered too small vtable
181 
181 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:114:1
- =======
186 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:111:54
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:114:54
189    |
190 LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
191    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered dangling vtable pointer in wide pointer
192 
192 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:116:1
- =======
197 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:113:57
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:116:57
200    |
201 LL | const TRAIT_OBJ_UNALIGNED_VTABLE: &dyn Trait = unsafe { mem::transmute((&92u8, &[0u8; 128])) };
202    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned vtable pointer in wide pointer
203 
203 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:118:1
- =======
208 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:115:57
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:118:57
211    |
212 LL | const TRAIT_OBJ_BAD_DROP_FN_NULL: &dyn Trait = unsafe { mem::transmute((&92u8, &[0usize; 8])) };
213    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
214 
214 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:120:1
- =======
219 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:117:56
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:120:56
222    |
223 LL | const TRAIT_OBJ_BAD_DROP_FN_INT: &dyn Trait = unsafe { mem::transmute((&92u8, &[1usize; 8])) };
224    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
225 
225 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:122:1
- =======
230 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:119:66
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:123:14
233    |
- LL | const TRAIT_OBJ_BAD_DROP_FN_NOT_FN_PTR: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
-    |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid drop function pointer in vtable (not pointing to a function)
+ LL |     unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid drop function pointer in vtable (not pointing to a function)
237 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:126:1
+   --> $DIR/ub-wide-ptr.rs:127:1
239    |
239    |
240 LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
241    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.<dyn-downcast>: encountered 0x03, but expected a boolean

245                ╾allocN─╼ ╾allocN─╼                         │ ╾──╼╾──╼
247 
247 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:130:1
- =======
252 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:127:62
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:131:62
255    |
256 LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { mem::transmute((&92u8, 0usize)) };
257    |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer
258 
258 
- <<<<<<< HEAD
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-wide-ptr.rs:132:1
- =======
263 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-wide-ptr.rs:129:65
- >>>>>>> 6273289316a (Try to turn on validation in CTFE for unsafe code)
+   --> $DIR/ub-wide-ptr.rs:133:65
266    |
267 LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { mem::transmute((&92u8, &3u64)) };
268    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable
269 
270 error[E0080]: could not evaluate static initializer
-   --> $DIR/ub-wide-ptr.rs:138:5
+   --> $DIR/ub-wide-ptr.rs:140:5
+   --> $DIR/ub-wide-ptr.rs:140:5
272    |
273 LL |     mem::transmute::<_, &dyn Trait>((&92u8, 0usize))
274    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer
275 
276 error[E0080]: could not evaluate static initializer
-   --> $DIR/ub-wide-ptr.rs:142:5
+   --> $DIR/ub-wide-ptr.rs:144:5
+   --> $DIR/ub-wide-ptr.rs:144:5
278    |
279 LL |     mem::transmute::<_, &dyn Trait>((&92u8, &3u64))
280    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable

The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/ub-wide-ptr.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-wide-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:40:53
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:43:39
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:46:44
   |
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:48:47
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:52:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:55:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:55:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:64:5
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:64:5
   |
LL |     mem::transmute((42, uninit_len))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized reference
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:68:40
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:71:50
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:74:42
   |
   |
LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:77:48
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (going beyond the bounds of its allocation)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:80:50
   |
   |
LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:84:51
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   |                                                   ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:60
   |
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
   |                                                            ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:93:67
   |
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
   |                                                                   ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:102:5
   |
   |
LL |     mem::transmute((42, uninit_len))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:108:58
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
   |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered too small vtable
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:111:58
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
   |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered too small vtable
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:114:54
   |
   |
LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
   |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered dangling vtable pointer in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:116:57
   |
   |
LL | const TRAIT_OBJ_UNALIGNED_VTABLE: &dyn Trait = unsafe { mem::transmute((&92u8, &[0u8; 128])) };
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned vtable pointer in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:118:57
   |
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_NULL: &dyn Trait = unsafe { mem::transmute((&92u8, &[0usize; 8])) };
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:120:56
   |
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_INT: &dyn Trait = unsafe { mem::transmute((&92u8, &[1usize; 8])) };
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:123:14
   |
   |
LL |     unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid drop function pointer in vtable (not pointing to a function)
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:127:1
   |
   |
LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.<dyn-downcast>: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc175─╼ ╾alloc178─╼                         │ ╾──╼╾──╼

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:131:62
   |
   |
LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { mem::transmute((&92u8, 0usize)) };
   |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:133:65
   |
   |
LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { mem::transmute((&92u8, &3u64)) };
   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:140:5
   |
   |
LL |     mem::transmute::<_, &dyn Trait>((&92u8, 0usize))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:144:5
   |
   |
LL |     mem::transmute::<_, &dyn Trait>((&92u8, &3u64))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable
error: aborting due to 29 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
