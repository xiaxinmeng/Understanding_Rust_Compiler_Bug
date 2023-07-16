plain
normalized stderr:
error[E0034]: multiple applicable items in scope
  --> $DIR/edition-field.rs:23:27
   |
LL |     let _intersperse = it.intersperse(());
   |                           ^^^^^^^^^^^ multiple `intersperse` found
   |
note: candidate #1 is defined in an impl of the trait `Itertools` for the type `T`
   |
   |
LL |     fn intersperse(&self, separator: ()) -> Intersperse {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `edition_field_std::Iterator` for the type `MyIterator`
   |
   |
LL | impl Iterator for MyIterator {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #3 is defined in the trait `std::iter::Iterator`
  --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
   |
LL | /     fn intersperse(self, separator: Self::Item) -> Intersperse<Self>
LL | |     where
LL | |         Self: Sized,
LL | |         Self::Item: Clone,
help: disambiguate the associated function for candidate #1
   |
   |
LL |     let _intersperse = Itertools::intersperse(&it, ());
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: disambiguate the associated function for candidate #2
   |
   |
LL |     let _intersperse = edition_field_std::Iterator::intersperse(&it, ());
help: disambiguate the associated function for candidate #3
   |
   |
LL |     let _intersperse = std::iter::Iterator::intersperse(it, ());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
---
To only update this specific test, also pass `--test-args stability-attribute/edition-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/edition-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/edition-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/edition-field/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0034]: multiple applicable items in scope
  --> /checkout/src/test/ui/stability-attribute/edition-field.rs:23:27
   |
LL |     let _intersperse = it.intersperse(());
   |                           ^^^^^^^^^^^ multiple `intersperse` found
   |
note: candidate #1 is defined in an impl of the trait `Itertools` for the type `T`
  --> /checkout/src/test/ui/stability-attribute/edition-field.rs:9:5
   |
LL |     fn intersperse(&self, separator: ()) -> Intersperse {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `edition_field_std::Iterator` for the type `MyIterator`
  --> /checkout/src/test/ui/stability-attribute/edition-field.rs:19:1
   |
LL | impl Iterator for MyIterator {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #3 is defined in the trait `std::iter::Iterator`
   |
   |
LL | /     fn intersperse(self, separator: Self::Item) -> Intersperse<Self>
LL | |     where
LL | |         Self: Sized,
LL | |         Self::Item: Clone,
help: disambiguate the associated function for candidate #1
   |
   |
LL |     let _intersperse = Itertools::intersperse(&it, ());
help: disambiguate the associated function for candidate #2
   |
   |
LL |     let _intersperse = edition_field_std::Iterator::intersperse(&it, ());
help: disambiguate the associated function for candidate #3
   |
   |
LL |     let _intersperse = std::iter::Iterator::intersperse(it, ());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
