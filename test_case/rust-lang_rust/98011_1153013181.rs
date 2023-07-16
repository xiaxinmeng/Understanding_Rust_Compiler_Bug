plain
........................................................................................ 12584/13060
........................................................................................ 12672/13060
........................................................................................ 12760/13060
........................................................................................ 12848/13060
............................................................................F..F.......F 12936/13060
........F.FF...FFF.iii.................................................................. 13024/13060
failures:

---- [ui] src/test/ui/variance/variance-associated-types.rs stdout ----
diff of stderr:
---
To only update this specific test, also pass `--test-args variance/variance-associated-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-associated-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, +]
   |
   |
LL | / struct Foo<'a, T : Trait<'a>> { //~ ERROR [-, +]
LL | |     field: (T, &'a ())
LL | | }


error[E0208]: [o, o]
   |
   |
LL | / struct Bar<'a, T : Trait<'a>> { //~ ERROR [o, o]
LL | |     field: <T as Trait<'a>>::Type
LL | | }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-associated-consts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-associated-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-consts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-consts/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [o]
   |
   |
LL | / struct Foo<T: Trait> { //~ ERROR [o]
LL | |     field: [u8; <T as Trait>::Const]
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-object-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-object-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [o]
   |
   |
LL | / struct Foo<'a> { //~ ERROR [o]
LL | |     x: Box<dyn Fn(i32) -> &'a i32 + 'static>
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-trait-object-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-trait-object-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-object-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-object-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-]
   |
   |
LL | / struct TOption<'a> { //~ ERROR [-]
LL | |     v: Option<Box<dyn T + 'a>>,
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-regions-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-direct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-direct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, -, -]
   |
   |
LL | / struct Test2<'a, 'b, 'c> { //~ ERROR [-, -, -]
LL | |     x: &'a isize,
LL | |     y: &'b [isize],
LL | |     c: &'c str
LL | | }


error[E0208]: [+, +, +]
   |
   |
LL | / struct Test3<'a, 'b, 'c> { //~ ERROR [+, +, +]
LL | |     x: extern "Rust" fn(&'a isize),
LL | |     y: extern "Rust" fn(&'b [isize]),
LL | |     c: extern "Rust" fn(&'c str),
LL | | }


error[E0208]: [-, o]
   |
   |
LL | / struct Test4<'a, 'b:'a> { //~ ERROR [-, o]
LL | |     x: &'a mut &'b isize,
LL | | }


error[E0208]: [+, o]
   |
   |
LL | / struct Test5<'a, 'b:'a> { //~ ERROR [+, o]
LL | |     x: extern "Rust" fn(&'a mut &'b isize),
LL | | }


error[E0208]: [-, o]
   |
   |
LL | / struct Test6<'a, 'b:'a> { //~ ERROR [-, o]
LL | |     x: &'a mut extern "Rust" fn(&'b isize),
LL | | }

error[E0208]: [*]
  --> /checkout/src/test/ui/variance/variance-regions-direct.rs:52:1
   |
   |
LL | / struct Test7<'a> { //~ ERROR [*]
LL | |     x: isize
LL | | }


error[E0208]: [+, -, o]
   |
   |
LL | / enum Test8<'a, 'b, 'c:'b> { //~ ERROR [+, -, o]
LL | |     Test8A(extern "Rust" fn(&'a isize)),
LL | |     Test8B(&'b [isize]),
LL | |     Test8C(&'b mut &'c str),
LL | | }

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-regions-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-indirect" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-indirect/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, -, o, *]
   |
   |
LL | / enum Base<'a, 'b, 'c:'b, 'd> { //~ ERROR [+, -, o, *]
LL | |     Test8A(extern "Rust" fn(&'a isize)),
LL | |     Test8B(&'b [isize]),
LL | |     Test8C(&'b mut &'c str),
LL | | }


error[E0208]: [*, o, -, +]
   |
   |
LL | / struct Derived1<'w, 'x:'y, 'y, 'z> { //~ ERROR [*, o, -, +]
LL | |     f: Base<'z, 'y, 'x, 'w>
LL | | }


error[E0208]: [o, o, *]
   |
   |
LL | / struct Derived2<'a, 'b:'a, 'c> { //~ ERROR [o, o, *]
LL | |     f: Base<'a, 'a, 'b, 'c>
LL | | }


error[E0208]: [o, -, *]
   |
   |
LL | / struct Derived3<'a:'b, 'b, 'c> { //~ ERROR [o, -, *]
LL | |     f: Base<'a, 'b, 'a, 'c>
LL | | }


error[E0208]: [+, -, o]
   |
   |
LL | / struct Derived4<'a, 'b, 'c:'b> { //~ ERROR [+, -, o]
LL | |     f: Base<'a, 'b, 'c, 'a>
LL | | }

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-types-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-types-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, +]
   |
   |
LL | / struct TestImm<A, B> { //~ ERROR [+, +]
LL | |     x: A,
LL | |     y: B,
LL | | }


error[E0208]: [+, o]
   |
   |
LL | / struct TestMut<A, B:'static> { //~ ERROR [+, o]
LL | |     x: A,
LL | |     y: &'static mut B,
LL | | }


error[E0208]: [+, o]
   |
   |
LL | / struct TestIndirect<A:'static, B:'static> { //~ ERROR [+, o]
LL | |     m: TestMut<A, B>
LL | | }


error[E0208]: [o, o]
   |
   |
LL | / struct TestIndirect2<A:'static, B:'static> { //~ ERROR [o, o]
LL | |     n: TestMut<A, B>,
LL | |     m: TestMut<B, A>
LL | | }


error[E0208]: [o, o]
   |
   |
LL | / struct TestObject<A, R> { //~ ERROR [o, o]
LL | |     n: Box<dyn Setter<A>+Send>,
LL | |     m: Box<dyn Getter<R>+Send>,
LL | | }

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-trait-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-trait-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, +]
   |
   |
LL | / struct TestStruct<U,T:Setter<U>> { //~ ERROR [+, +]
LL | |     t: T, u: U
LL | | }


error[E0208]: [*, +]
   |
   |
LL | / enum TestEnum<U,T:Setter<U>> { //~ ERROR [*, +]
LL | |     Foo(T)
LL | | }


error[E0208]: [*, +]
   |
   |
LL | / struct TestContraStruct<U,T:Setter<U>> { //~ ERROR [*, +]
LL | |     t: T
LL | | }


error[E0208]: [*, +]
   |
   |
LL | / struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR [*, +]
LL | |     t: T
LL | | }

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, o, o]
   |
   |
LL | / struct InvariantMut<'a,A:'a,B:'a> { //~ ERROR [-, o, o]
LL | |     t: &'a mut (A,B)
LL | | }

error[E0208]: [o]
  --> /checkout/src/test/ui/variance/variance-types.rs:15:1
   |
   |
LL | / struct InvariantCell<A> { //~ ERROR [o]
LL | |     t: Cell<A>
LL | | }

error[E0208]: [o]
  --> /checkout/src/test/ui/variance/variance-types.rs:20:1
   |
   |
LL | / struct InvariantIndirect<A> { //~ ERROR [o]
LL | |     t: InvariantCell<A>
LL | | }

error[E0208]: [+]
  --> /checkout/src/test/ui/variance/variance-types.rs:25:1
   |
   |
LL | / struct Covariant<A> { //~ ERROR [+]
LL | |     t: A, u: fn() -> A
LL | | }

error[E0208]: [-]
  --> /checkout/src/test/ui/variance/variance-types.rs:30:1
   |
   |
LL | / struct Contravariant<A> { //~ ERROR [-]
LL | |     t: fn(A)
LL | | }


error[E0208]: [+, -, o]
   |
   |
LL | / enum Enum<A,B,C> { //~ ERROR [+, -, o]
LL | |     Foo(Covariant<A>),
LL | |     Bar(Contravariant<B>),
LL | |     Zed(Covariant<C>,Contravariant<C>)
LL | | }

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0208`.
