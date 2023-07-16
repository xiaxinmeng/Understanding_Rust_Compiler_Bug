plain

557    |                                     this code causes undefined behavior when executed
558    |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
559    |
-    = note: `WrapAroundRange` must be non-null
+    = note: `WrapAroundRange` must be initialized inside its custom valid range
562 error: the type `Result<i32, i32>` does not permit being left uninitialized
563   --> $DIR/invalid_value.rs:144:38


---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/invalid_value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/invalid_value" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/invalid_value/auxiliary"
stdout: none
--- stderr -------------------------------
error: the type `&T` does not permit zero-initialization
   |
   |
LL |         let _val: &'static T = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/invalid_value.rs:6:9
   |
LL | #![deny(invalid_value)]
   |         ^^^^^^^^^^^^^

error: the type `&T` does not permit being left uninitialized
   |
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `Wrap<&T>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<&'static T> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<&T>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<&'static T> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `!` does not permit zero-initialization
   |
   |
LL |         let _val: ! = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                       |
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value
error: the type `!` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:66:23
   |
   |
LL |         let _val: ! = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                       |
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit zero-initialization
   |
   |
LL |         let _val: (i32, !) = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                              |
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit being left uninitialized
   |
   |
LL |         let _val: (i32, !) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                              |
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: integers must not be uninitialized

error: the type `Void` does not permit zero-initialization
   |
   |
LL |         let _val: Void = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums with no inhabited variants have no valid value
   |
   |
LL | enum Void {}

error: the type `Void` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:72:26
   |
   |
LL |         let _val: Void = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums with no inhabited variants have no valid value
   |
   |
LL | enum Void {}


error: the type `&i32` does not permit zero-initialization
   |
   |
LL |         let _val: &'static i32 = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `&i32` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:75:34
   |
LL |         let _val: &'static i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `Ref` does not permit zero-initialization
   |
   |
LL |         let _val: Ref = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:14:12
   |
LL | struct Ref(&'static i32);

error: the type `Ref` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:78:25
   |
   |
LL |         let _val: Ref = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:14:12
   |
LL | struct Ref(&'static i32);


error: the type `fn()` does not permit zero-initialization
   |
   |
LL |         let _val: fn() = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `fn()` does not permit being left uninitialized
   |
   |
LL |         let _val: fn() = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `Wrap<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `WrapEnum<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this field of the only potentially inhabited enum variant)
  --> /checkout/src/test/ui/lint/invalid_value.rs:18:28
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `WrapEnum<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this field of the only potentially inhabited enum variant)
  --> /checkout/src/test/ui/lint/invalid_value.rs:18:28
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `Wrap<(RefPair, i32)>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:15:16
   |
LL | struct RefPair((&'static i32, i32));


error: the type `Wrap<(RefPair, i32)>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:15:16
   |
LL | struct RefPair((&'static i32, i32));


error: the type `NonNull<i32>` does not permit zero-initialization
   |
   |
LL |         let _val: NonNull<i32> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `NonNull<i32>` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:93:34
   |
LL |         let _val: NonNull<i32> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `(NonZeroU32, i32)` does not permit zero-initialization
   |
   |
LL |         let _val: (NonZeroU32, i32) = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                       |
   |                                       this code causes undefined behavior when executed
   |                                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::num::NonZeroU32` must be non-null

error: the type `(NonZeroU32, i32)` does not permit being left uninitialized
   |
   |
LL |         let _val: (NonZeroU32, i32) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                       |
   |                                       this code causes undefined behavior when executed
   |                                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::num::NonZeroU32` must be non-null

error: the type `*const dyn Send` does not permit zero-initialization
   |
   |
LL |         let _val: *const dyn Send = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `*const dyn Send` does not permit being left uninitialized
   |
   |
LL |         let _val: *const dyn Send = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `[fn(); 2]` does not permit zero-initialization
   |
   |
LL |         let _val: [fn(); 2] = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `[fn(); 2]` does not permit being left uninitialized
   |
   |
LL |         let _val: [fn(); 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `TwoUninhabited` does not permit zero-initialization
   |
   |
LL |         let _val: TwoUninhabited = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums with no inhabited variants have no valid value
   |
   |
LL | enum TwoUninhabited {


error: the type `TwoUninhabited` does not permit being left uninitialized
   |
   |
LL |         let _val: TwoUninhabited = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums with no inhabited variants have no valid value
   |
   |
LL | enum TwoUninhabited {


error: the type `OneFruitNonZero` does not permit zero-initialization
   |
   |
LL |         let _val: OneFruitNonZero = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: `std::num::NonZeroU32` must be non-null (in this field of the only potentially inhabited enum variant)
   |
   |
LL |     Banana(NonZeroU32),


error: the type `OneFruitNonZero` does not permit being left uninitialized
   |
   |
LL |         let _val: OneFruitNonZero = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: `std::num::NonZeroU32` must be non-null (in this field of the only potentially inhabited enum variant)
   |
   |
LL |     Banana(NonZeroU32),

error: the type `bool` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/invalid_value.rs:112:26
   |
   |
LL |         let _val: bool = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: booleans must be either `true` or `false`

error: the type `Wrap<char>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<char> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: characters must be a valid Unicode codepoint (in this struct field)
  --> /checkout/src/test/ui/lint/invalid_value.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `NonBig` does not permit being left uninitialized
   |
   |
LL |         let _val: NonBig = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                            |
   |                            this code causes undefined behavior when executed
   |                            help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `NonBig` must be initialized inside its custom valid range

error: the type `Fruit` does not permit being left uninitialized
   |
   |
LL |         let _val: Fruit = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                           |
   |                           this code causes undefined behavior when executed
   |                           help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums with multiple inhabited variants have to be initialized to a variant
  --> /checkout/src/test/ui/lint/invalid_value.rs:26:1
   |
LL | enum Fruit {
