plain

6    |
7    = note: positive implementation in crate `core`
8 
- error[E0751]: found both positive and negative implementation of trait `std::marker::Copy` for type `memchr::memmem::genericsimd::Forward`:
+ error[E0751]: found both positive and negative implementation of trait `std::marker::Copy` for type `memchr::memmem::prefilter::PrefilterFn`:
10   --> $DIR/coherence-negative-impls-copy-problematic-non-adt.rs:69:1
11    |
12 LL | impl<T> !Copy for T {}

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt/coherence-negative-impls-copy-problematic-non-adt.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt/coherence-negative-impls-copy-problematic-non-adt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-negative-impls-copy-problematic-non-adt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0751]: found both positive and negative implementation of trait `std::marker::Copy` for type `!`:
   |
   |
LL | impl !Copy for ! {}
   | ^^^^^^^^^^^^^^^^ negative implementation here
   = note: positive implementation in crate `core`


error[E0751]: found both positive and negative implementation of trait `std::marker::Copy` for type `memchr::memmem::prefilter::PrefilterFn`:
   |
   |
LL | impl<T> !Copy for T {}
   | ^^^^^^^^^^^^^^^^^^^ negative implementation here
   = note: positive implementation in crate `memchr`

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:44:1
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:44:1
   |
LL | impl !Copy for str {}
   | |              |
   | |              `str` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:47:1
   |
LL | impl !Copy for fn() {}
   | |              |
   | |              |
   | |              `fn()` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:50:1
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:50:1
   |
LL | impl !Copy for ! {}
   | |              |
   | |              `!` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:54:1
   |
LL | impl !Copy for <Type as Trait>::Result {}
   | |              |
   | |              |
   | |              `<Type as Trait>::Result` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:57:1
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:57:1
   |
LL | impl !Copy for [Type] {}
   | |              |
   | |              this is not defined in the current crate because slices are always foreign
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:60:1
   |
LL | impl !Copy for &mut std::cmp::Ordering {}
   | |              |
   | |              `std::cmp::Ordering` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:63:1
   |
LL | impl !Copy for dyn std::any::Any {}
   | |              |
   | |              |
   | |              `dyn Any` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:66:1
  --> /checkout/src/test/ui/coherence/coherence-negative-impls-copy-problematic-non-adt.rs:66:1
   |
LL | impl !Copy for () {}
   | |              |
   | |              this is not defined in the current crate because tuples are always foreign
   | impl doesn't use only types from inside the current crate
   |
   |
   = note: define and implement a trait or new type instead

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> !Copy for T {}
   |      ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to 11 previous errors

Some errors have detailed explanations: E0117, E0210, E0751.
For more information about an error, try `rustc --explain E0117`.
