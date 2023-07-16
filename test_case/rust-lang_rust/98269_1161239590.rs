plain

---- [ui] src/test/ui/error-codes/E0109.rs stdout ----
diff of stderr:

4 LL | type X = u32<i32>;
5    |          --- ^^^ type argument not allowed
-    |          not allowed on this type
+    |          not allowed on builtin type `u32`
8    |
8    |
9 help: primitive type `u32` doesn't have generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0109/E0109.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0109/E0109.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0109.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0109.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0109" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0109/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0109]: type arguments are not allowed on builtin type `u32`
   |
   |
LL | type X = u32<i32>; //~ ERROR E0109
   |          --- ^^^ type argument not allowed
   |          not allowed on builtin type `u32`
   |
   |
help: primitive type `u32` doesn't have generic parameters
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL - type X = u32<i32>; //~ ERROR E0109
LL + type X = u32; //~ ERROR E0109

error: aborting due to previous error

For more information about this error, try `rustc --explain E0109`.
For more information about this error, try `rustc --explain E0109`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0110.rs stdout ----
diff of stderr:

4 LL | type X = u32<'static>;
6    |          |
-    |          not allowed on this type
+    |          not allowed on builtin type `u32`
8    |
8    |
9 help: primitive type `u32` doesn't have generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0110/E0110.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0110/E0110.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0110.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0110.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0110" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0110/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0109]: lifetime arguments are not allowed on builtin type `u32`
   |
   |
LL | type X = u32<'static>; //~ ERROR E0109
   |          |
   |          not allowed on builtin type `u32`
   |
   |
help: primitive type `u32` doesn't have generic parameters
   |
LL - type X = u32<'static>; //~ ERROR E0109
LL + type X = u32; //~ ERROR E0109

error: aborting due to previous error

For more information about this error, try `rustc --explain E0109`.
For more information about this error, try `rustc --explain E0109`.
------------------------------------------


---- [ui] src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs stdout ----
diff of stderr:

278 LL |     Enum::<()>::TSVariant::<()>(());
279    |                 ---------   ^^ type argument not allowed
-    |                 not allowed on this type
-    |                 not allowed on this type
+    |                 not allowed on tuple variant `TSVariant`
283 error[E0109]: type arguments are not allowed on this type
284   --> $DIR/enum-variant-generic-args.rs:57:24


346 LL |     Enum::<()>::SVariant::<()> { v: () };
347    |                 --------   ^^ type argument not allowed
-    |                 not allowed on this type
-    |                 not allowed on this type
+    |                 not allowed on variant `SVariant`
351    = note: enum variants can't have type parameters
352 


444 LL |     Enum::<()>::UVariant::<()>;
445    |                 --------   ^^ type argument not allowed
-    |                 not allowed on this type
+    |                 not allowed on unit variant `UVariant`
448 
449 error[E0109]: type arguments are not allowed on this type
---
To only update this specific test, also pass `--test-args type-alias-enum-variants/enum-variant-generic-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/enum-variant-generic-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/enum-variant-generic-args/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:13:25
   |
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
LL |     fn ts_variant() {
LL |         Self::TSVariant(());
   |         --------------- ^^ expected type parameter `T`, found `()`
   |         arguments to this function are incorrect
   |
   = note: expected type parameter `T`
                   found unit type `()`
                   found unit type `()`
note: tuple variant defined here
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:7:16
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:15:27
   |
   |
LL |         Self::TSVariant::<()>(());
   |               ---------   ^^ type argument not allowed
   |               not allowed on this type

error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:17:16
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:17:16
   |
LL |         Self::<()>::TSVariant(());
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::TSVariant(());

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:17:31
   |
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
...
LL |         Self::<()>::TSVariant(());
   |         --------------------- ^^ expected type parameter `T`, found `()`
   |         arguments to this function are incorrect
   |
   = note: expected type parameter `T`
                   found unit type `()`
                   found unit type `()`
note: tuple variant defined here
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:7:16
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }

error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:20:16
   |
   |
LL |         Self::<()>::TSVariant::<()>(());
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::TSVariant::<()>(());

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:20:33
   |
   |
LL |         Self::<()>::TSVariant::<()>(());
   |                     ---------   ^^ type argument not allowed
   |                     not allowed on this type

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:26:29
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:26:29
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
...
LL |         Self::SVariant { v: () };
   |                             ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`

error[E0109]: type arguments are not allowed on this type
error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:28:26
   |
LL |         Self::SVariant::<()> { v: () };
   |               --------   ^^ type argument not allowed
   |               not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -         Self::SVariant::<()> { v: () };
LL +         Enum::<()>::SVariant { v: () };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:28:35
   |
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
...
LL |         Self::SVariant::<()> { v: () };
   |                                   ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`

error[E0109]: type arguments are not allowed on self type
error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:31:16
   |
LL |         Self::<()>::SVariant { v: () };
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::SVariant { v: () };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:31:35
   |
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
...
LL |         Self::<()>::SVariant { v: () };
   |                                   ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`

error[E0109]: type arguments are not allowed on self type
error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:34:16
   |
LL |         Self::<()>::SVariant::<()> { v: () };
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::SVariant::<()> { v: () };

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:34:32
   |
   |
LL |         Self::<()>::SVariant::<()> { v: () };
   |                     --------   ^^ type argument not allowed
   |                     not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -         Self::<()>::SVariant::<()> { v: () };
LL +         Enum::<()>::SVariant { v: () };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:34:41
   |
   |
LL | impl<T> Enum<T> {
   |      - this type parameter
...
LL |         Self::<()>::SVariant::<()> { v: () };
   |                                         ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`

error[E0109]: type arguments are not allowed on this type
---

error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:43:16
   |
LL |         Self::<()>::UVariant;
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::UVariant;

error[E0109]: type arguments are not allowed on self type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:45:16
   |
   |
LL |         Self::<()>::UVariant::<()>;
   |         ----   ^^ type argument not allowed
   |         not allowed on self type
   |
   |
note: `Self` is of type `Enum<T>`
   |
   |
LL | enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
   |      ^^^^ `Self` corresponds to this type
...
LL | impl<T> Enum<T> {
   | --------------- `Self` is on type `Enum` in this `impl`
help: the `Self` type doesn't accept type parameters, use the concrete type's name `Enum` instead if you want to specify its type parameters
   |
LL |         Enum::<()>::UVariant::<()>;

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:45:32
   |
   |
LL |         Self::<()>::UVariant::<()>;
   |                     --------   ^^ type argument not allowed
   |                     not allowed on this type


error[E0109]: type arguments are not allowed on tuple variant `TSVariant`
   |
   |
LL |     Enum::<()>::TSVariant::<()>(());
   |                 ---------   ^^ type argument not allowed
   |                 |
   |                 not allowed on tuple variant `TSVariant`
error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:57:24
   |
   |
LL |     Alias::TSVariant::<()>(());
   |            ---------   ^^ type argument not allowed
   |            not allowed on this type

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:59:30
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:59:30
   |
LL |     Alias::<()>::TSVariant::<()>(());
   |                  ---------   ^^ type argument not allowed
   |                  not allowed on this type

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:62:29
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:62:29
   |
LL |     AliasFixed::TSVariant::<()>(());
   |                 ---------   ^^ type argument not allowed
   |                 not allowed on this type


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::TSVariant(());
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::TSVariant::<()>(());
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:66:35
   |
   |
LL |     AliasFixed::<()>::TSVariant::<()>(());
   |                       ---------   ^^ type argument not allowed
   |                       not allowed on this type


error[E0109]: type arguments are not allowed on variant `SVariant`
   |
   |
LL |     Enum::<()>::SVariant::<()> { v: () };
   |                 --------   ^^ type argument not allowed
   |                 |
   |                 not allowed on variant `SVariant`
   = note: enum variants can't have type parameters

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:75:23
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:75:23
   |
LL |     Alias::SVariant::<()> { v: () };
   |            --------   ^^ type argument not allowed
   |            not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -     Alias::SVariant::<()> { v: () };
LL +     Alias::<()>::SVariant { v: () };

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:77:29
   |
   |
LL |     Alias::<()>::SVariant::<()> { v: () };
   |                  --------   ^^ type argument not allowed
   |                  not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -     Alias::<()>::SVariant::<()> { v: () };
LL +     Alias::<()>::SVariant { v: () };

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:80:28
   |
   |
LL |     AliasFixed::SVariant::<()> { v: () };
   |                 --------   ^^ type argument not allowed
   |                 not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -     AliasFixed::SVariant::<()> { v: () };
LL +     AliasFixed::<()>::SVariant { v: () };


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::SVariant { v: () };
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::SVariant::<()> { v: () };
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:84:34
   |
   |
LL |     AliasFixed::<()>::SVariant::<()> { v: () };
   |                       --------   ^^ type argument not allowed
   |                       not allowed on this type
   |
   = note: enum variants can't have type parameters
   = note: enum variants can't have type parameters
help: you might have meant to specity type parameters on enum `Enum`
   |
LL -     AliasFixed::<()>::SVariant::<()> { v: () };
LL +     AliasFixed::<()>::SVariant { v: () };


error[E0109]: type arguments are not allowed on unit variant `UVariant`
   |
   |
LL |     Enum::<()>::UVariant::<()>;
   |                 --------   ^^ type argument not allowed
   |                 not allowed on unit variant `UVariant`

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:93:23
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:93:23
   |
LL |     Alias::UVariant::<()>;
   |            --------   ^^ type argument not allowed
   |            not allowed on this type

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:95:29
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:95:29
   |
LL |     Alias::<()>::UVariant::<()>;
   |                  --------   ^^ type argument not allowed
   |                  not allowed on this type

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:98:28
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:98:28
   |
LL |     AliasFixed::UVariant::<()>;
   |                 --------   ^^ type argument not allowed
   |                 not allowed on this type


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::UVariant;
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;


error[E0107]: this type alias takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     AliasFixed::<()>::UVariant::<()>;
   |     ^^^^^^^^^^------ help: remove these generics
   |     expected 0 generic arguments
   |
   |
note: type alias defined here, with 0 generic parameters
   |
   |
LL | type AliasFixed = Enum<()>;

error[E0109]: type arguments are not allowed on this type
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-generic-args.rs:102:34
   |
   |
LL |     AliasFixed::<()>::UVariant::<()>;
   |                       --------   ^^ type argument not allowed
   |                       not allowed on this type

error: aborting due to 39 previous errors

---
43    |        |
-    |        not allowed on this type
+    |        not allowed on builtin type `u8`
45    |
46 help: primitive type `u8` doesn't have generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/issue-91268.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/issue-91268.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/issue-91268.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/issue-91268.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type/issue-91268.rs:9:12
   |
LL | fn main() {
   |           - unclosed delimiter
---
error[E0412]: cannot find type `ţ` in this scope
  --> /checkout/src/test/ui/type/issue-91268.rs:9:11
   |
LL |     0: u8(ţ
   |           ^ expecting a type here because of type ascription

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
LL |     0: u8(ţ
LL |     0: u8(ţ
   |        ^^^^ only `Fn` traits may use parentheses
help: use angle brackets instead
   |
LL |     0: u8<ţ>
   |          ~ +
---
   |        -- ^ type argument not allowed
   |        |
   |        not allowed on builtin type `u8`
   |
help: primitive type `u8` doesn't have generic parameters
LL -     0: u8(ţ
LL +     0: u8
   |


error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/issue-91268.rs:9:5
   |
LL | fn main() {
   |           - expected `()` because of default return type
   |     ^^^^^^^ expected `()`, found `u8`

error: aborting due to 6 previous errors

---

---- [ui] src/test/ui/typeck/prim-with-args.rs stdout ----
diff of stderr:

4 LL | let _x: isize<isize>;
5    |         ----- ^^^^^ type argument not allowed
-    |         not allowed on this type
-    |         not allowed on this type
+    |         not allowed on builtin type `isize`
8    |
9 help: primitive type `isize` doesn't have generic parameters


18 LL | let _x: i8<isize>;
19    |         -- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `i8`
22    |
22    |
23 help: primitive type `i8` doesn't have generic parameters


32 LL | let _x: i16<isize>;
33    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `i16`
36    |
36    |
37 help: primitive type `i16` doesn't have generic parameters


46 LL | let _x: i32<isize>;
47    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `i32`
50    |
50    |
51 help: primitive type `i32` doesn't have generic parameters


60 LL | let _x: i64<isize>;
61    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `i64`
64    |
64    |
65 help: primitive type `i64` doesn't have generic parameters


74 LL | let _x: usize<isize>;
75    |         ----- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `usize`
78    |
78    |
79 help: primitive type `usize` doesn't have generic parameters


88 LL | let _x: u8<isize>;
89    |         -- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `u8`
92    |
92    |
93 help: primitive type `u8` doesn't have generic parameters


102 LL | let _x: u16<isize>;
103    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `u16`
106    |
106    |
107 help: primitive type `u16` doesn't have generic parameters


116 LL | let _x: u32<isize>;
117    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `u32`
120    |
120    |
121 help: primitive type `u32` doesn't have generic parameters


130 LL | let _x: u64<isize>;
131    |         --- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `u64`
134    |
134    |
135 help: primitive type `u64` doesn't have generic parameters


144 LL | let _x: char<isize>;
145    |         ---- ^^^^^ type argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `char`
148    |
148    |
149 help: primitive type `char` doesn't have generic parameters


158 LL | let _x: isize<'static>;
160    |         |
-    |         not allowed on this type
-    |         not allowed on this type
+    |         not allowed on builtin type `isize`
162    |
163 help: primitive type `isize` doesn't have generic parameters


172 LL | let _x: i8<'static>;
173    |         -- ^^^^^^^ lifetime argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `i8`
176    |
176    |
177 help: primitive type `i8` doesn't have generic parameters


186 LL | let _x: i16<'static>;
188    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `i16`
190    |
190    |
191 help: primitive type `i16` doesn't have generic parameters


200 LL | let _x: i32<'static>;
202    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `i32`
204    |
204    |
205 help: primitive type `i32` doesn't have generic parameters


214 LL | let _x: i64<'static>;
216    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `i64`
218    |
218    |
219 help: primitive type `i64` doesn't have generic parameters


228 LL | let _x: usize<'static>;
230    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `usize`
232    |
232    |
233 help: primitive type `usize` doesn't have generic parameters


242 LL | let _x: u8<'static>;
243    |         -- ^^^^^^^ lifetime argument not allowed
-    |         not allowed on this type
+    |         not allowed on builtin type `u8`
246    |
246    |
247 help: primitive type `u8` doesn't have generic parameters


256 LL | let _x: u16<'static>;
258    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `u16`
260    |
260    |
261 help: primitive type `u16` doesn't have generic parameters


270 LL | let _x: u32<'static>;
272    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `u32`
274    |
274    |
275 help: primitive type `u32` doesn't have generic parameters


284 LL | let _x: u64<'static>;
286    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `u64`
288    |
288    |
289 help: primitive type `u64` doesn't have generic parameters


298 LL | let _x: char<'static>;
300    |         |
-    |         not allowed on this type
+    |         not allowed on builtin type `char`
302    |
302    |
303 help: primitive type `char` doesn't have generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/prim-with-args/prim-with-args.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/prim-with-args/prim-with-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/prim-with-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/prim-with-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/prim-with-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/prim-with-args/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0109]: type arguments are not allowed on builtin type `isize`
   |
   |
LL | let _x: isize<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         ----- ^^^^^ type argument not allowed
   |         |
   |         not allowed on builtin type `isize`
   |
help: primitive type `isize` doesn't have generic parameters
   |
LL - let _x: isize<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: isize; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `i8`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:5:12
   |
   |
LL | let _x: i8<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         -- ^^^^^ type argument not allowed
   |         not allowed on builtin type `i8`
   |
   |
help: primitive type `i8` doesn't have generic parameters
   |
LL - let _x: i8<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: i8; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `i16`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:6:13
   |
   |
LL | let _x: i16<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `i16`
   |
   |
help: primitive type `i16` doesn't have generic parameters
   |
LL - let _x: i16<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: i16; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `i32`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:7:13
   |
   |
LL | let _x: i32<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `i32`
   |
   |
help: primitive type `i32` doesn't have generic parameters
   |
LL - let _x: i32<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: i32; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `i64`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:8:13
   |
   |
LL | let _x: i64<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `i64`
   |
   |
help: primitive type `i64` doesn't have generic parameters
   |
LL - let _x: i64<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: i64; //~ ERROR type arguments are not allowed on builtin type


error[E0109]: type arguments are not allowed on builtin type `usize`
   |
   |
LL | let _x: usize<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         ----- ^^^^^ type argument not allowed
   |         not allowed on builtin type `usize`
   |
   |
help: primitive type `usize` doesn't have generic parameters
   |
LL - let _x: usize<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: usize; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `u8`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:10:12
   |
   |
LL | let _x: u8<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         -- ^^^^^ type argument not allowed
   |         not allowed on builtin type `u8`
   |
   |
help: primitive type `u8` doesn't have generic parameters
   |
LL - let _x: u8<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: u8; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `u16`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:11:13
   |
   |
LL | let _x: u16<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `u16`
   |
   |
help: primitive type `u16` doesn't have generic parameters
   |
LL - let _x: u16<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: u16; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `u32`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:12:13
   |
   |
LL | let _x: u32<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `u32`
   |
   |
help: primitive type `u32` doesn't have generic parameters
   |
LL - let _x: u32<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: u32; //~ ERROR type arguments are not allowed on builtin type

error[E0109]: type arguments are not allowed on builtin type `u64`
  --> /checkout/src/test/ui/typeck/prim-with-args.rs:13:13
   |
   |
LL | let _x: u64<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         --- ^^^^^ type argument not allowed
   |         not allowed on builtin type `u64`
   |
   |
help: primitive type `u64` doesn't have generic parameters
   |
LL - let _x: u64<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: u64; //~ ERROR type arguments are not allowed on builtin type


error[E0109]: type arguments are not allowed on builtin type `char`
   |
   |
LL | let _x: char<isize>; //~ ERROR type arguments are not allowed on builtin type
   |         ---- ^^^^^ type argument not allowed
   |         not allowed on builtin type `char`
   |
   |
help: primitive type `char` doesn't have generic parameters
   |
LL - let _x: char<isize>; //~ ERROR type arguments are not allowed on builtin type
LL + let _x: char; //~ ERROR type arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `isize`
   |
   |
LL | let _x: isize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         |
   |         not allowed on builtin type `isize`
   |
help: primitive type `isize` doesn't have generic parameters
   |
LL - let _x: isize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: isize; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `i8`
   |
   |
LL | let _x: i8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         -- ^^^^^^^ lifetime argument not allowed
   |         not allowed on builtin type `i8`
   |
   |
help: primitive type `i8` doesn't have generic parameters
   |
LL - let _x: i8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: i8; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `i16`
   |
   |
LL | let _x: i16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `i16`
   |
   |
help: primitive type `i16` doesn't have generic parameters
   |
LL - let _x: i16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: i16; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `i32`
   |
   |
LL | let _x: i32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `i32`
   |
   |
help: primitive type `i32` doesn't have generic parameters
   |
LL - let _x: i32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: i32; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `i64`
   |
   |
LL | let _x: i64<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `i64`
   |
   |
help: primitive type `i64` doesn't have generic parameters
   |
LL - let _x: i64<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: i64; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `usize`
   |
   |
LL | let _x: usize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `usize`
   |
   |
help: primitive type `usize` doesn't have generic parameters
   |
LL - let _x: usize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: usize; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `u8`
   |
   |
LL | let _x: u8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         -- ^^^^^^^ lifetime argument not allowed
   |         not allowed on builtin type `u8`
   |
   |
help: primitive type `u8` doesn't have generic parameters
   |
LL - let _x: u8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: u8; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `u16`
   |
   |
LL | let _x: u16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `u16`
   |
   |
help: primitive type `u16` doesn't have generic parameters
   |
LL - let _x: u16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: u16; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `u32`
   |
   |
LL | let _x: u32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `u32`
   |
   |
help: primitive type `u32` doesn't have generic parameters
   |
LL - let _x: u32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
LL + let _x: u32; //~ ERROR lifetime arguments are not allowed on builtin type


error[E0109]: lifetime arguments are not allowed on builtin type `u64`
   |
   |
LL | let _x: u64<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
   |         |
   |         not allowed on builtin type `u64`
   |
