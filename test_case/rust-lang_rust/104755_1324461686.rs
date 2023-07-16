plain
........................................................................................ 10472/13887
........................................................................................ 10560/13887
........................................................................................ 10648/13887
........................................................................................ 10736/13887
.................................................F...F.................................. 10824/13887
........................................................iiiii...i....i.i................ 11000/13887
........................................................................................ 11088/13887
.......................................................i................................ 11176/13887
.................................................................iiiiii.i..ii..iiiiiiiii 11264/13887
---
To only update this specific test, also pass `--test-args return/return-impl-trait-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-impl-trait-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait-bad/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/return/return-impl-trait-bad.rs:5:5
   |
   |
LL | fn bad_echo<T>(_t: T) -> T {
   |             -            - expected `T` because of return type
   |             this type parameter
   |             this type parameter
LL |     "this should not suggest impl Trait" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait-bad.rs:9:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | fn bad_echo_2<T: Trait>(_t: T) -> T {
   |               -                   - expected `T` because of return type
   |               this type parameter
   |               this type parameter
LL |     "this will not suggest it, because that would probably be wrong" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait-bad.rs:17:5
   |
LL | fn other_bounds_bad<T>() -> T
   |                     -       - expected `T` because of return type
   |                     this type parameter
...
...
LL |     "don't suggest this, because Option<T> places additional constraints" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait-bad.rs:28:5
   |
LL | fn used_in_trait<T>() -> T
   |                  |       |
   |                  |       expected `T` because of return type
   |                  |       help: consider using an impl return type: `impl Send`
   |                  this type parameter
   |                  this type parameter
...
LL |     "don't suggest this, because the generic param is used in the bound." //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`
   = note: the caller chooses the value of a type parameter

---
To only update this specific test, also pass `--test-args return/return-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/return/return-impl-trait.rs:15:5
   |
   |
LL | fn bar<T: Trait + std::marker::Sync>() -> T
   |        |                                  |
   |        |                                  expected `T` because of return type
   |        this type parameter                help: consider using an impl return type: `impl Trait + std::marker::Sync + Send`
...
...
LL |     () //~ ERROR mismatched types
   |     ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`
   = note: the caller chooses the value of a type parameter


error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait.rs:23:5
   |
LL | fn other_bounds<T>() -> T
   |                 |       |
   |                 |       expected `T` because of return type
   |                 |       help: consider using an impl return type: `impl Trait`
   |                 this type parameter
   |                 this type parameter
...
LL |     () //~ ERROR mismatched types
   |     ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`
   = note: the caller chooses the value of a type parameter

