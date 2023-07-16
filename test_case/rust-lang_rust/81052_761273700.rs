plain
    Checking addr2line v0.14.0
error: this file contains an unclosed delimiter
    --> library/std/src/io/mod.rs:2571:3
     |
2480 | trait SizeHint {
     |                - unclosed delimiter
2481 |     fn lower_bound(&self) -> usize {
     |                                    - this delimiter might not be properly closed...
2490 | }
2490 | }
     | - ...as it matches this but it has different indentation
2571 | }
     |   ^


error: implementation is not supported in `trait`s or `impl`s
     |
     |
2493 | impl<T> SizeHint for T {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
     |
     |
2500 | impl<T> SizeHint for BufReader<T> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
     |
     |
2515 | pub struct Split<B> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
     |
     |
2521 | impl<B: BufRead> Iterator for Split<B> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
     |
     |
2547 | pub struct Lines<B> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
     |
     |
2552 | impl<B: BufRead> Iterator for Lines<B> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: `self` parameter is only allowed in associated functions
     |
     |
2483 |     fn upper_bound(&self) -> Option<usize> {
     |                    ^^^^^ not semantically valid as function parameter
     |
     = note: associated functions are those in `impl` or `trait` definitions

error: `self` parameter is only allowed in associated functions
     |
     |
2487 |     fn size_hint(&self) -> (usize, Option<usize>) {
     |                  ^^^^^ not semantically valid as function parameter
     |
     = note: associated functions are those in `impl` or `trait` definitions

error[E0401]: can't use generic parameters from outer function
     |
     |
2483 |     fn upper_bound(&self) -> Option<usize> {
     |                    |
     |                    use of generic parameter from outer function
     |                    use of generic parameter from outer function
     |                    can't use `Self` here

error[E0401]: can't use generic parameters from outer function
     |
     |
2487 |     fn size_hint(&self) -> (usize, Option<usize>) {
     |                  |
     |                  use of generic parameter from outer function
     |                  use of generic parameter from outer function
     |                  can't use `Self` here

error[E0412]: cannot find type `Split` in this scope
     |
     |
2052 |     fn split(self, byte: u8) -> Split<Self>
     |
help: consider importing one of these items
     |
254  | use alloc::slice::Split;
---
254  | use core::str::Split;
     |
       and 2 other candidates

error[E0422]: cannot find struct, variant or union type `Split` in this scope
     |
     |
2056 |         Split { buf: self, delim: byte }
     |
help: consider importing one of these items
     |
254  | use alloc::slice::Split;
---
254  | use core::str::Split;
     |
       and 2 other candidates

error[E0412]: cannot find type `Lines` in this scope
     |
     |
2089 |     fn lines(self) -> Lines<Self>
     |
help: consider importing one of these items
     |
254  | use alloc::str::Lines;
254  | use alloc::str::Lines;
     |
254  | use core::str::Lines;
     |
254  | use crate::str::Lines;
     |

error[E0422]: cannot find struct, variant or union type `Lines` in this scope
     |
     |
2093 |         Lines { buf: self }
     |
help: consider importing one of these items
     |
254  | use alloc::str::Lines;
254  | use alloc::str::Lines;
     |
254  | use core::str::Lines;
     |
254  | use crate::str::Lines;
     |

error[E0599]: no method named `size_hint` found for type parameter `R` in the current scope
     |
     |
2475 |         self.inner.size_hint()
     |                    ^^^^^^^^^ method not found in `R`
     = help: items from traits can only be used if the type parameter is bounded by the trait
     = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `size_hint`, perhaps you need to restrict type parameter `R` with it:
     |
2459 | impl<R: core::iter::Iterator + Read + SizeHint> Iterator for Bytes<R> {

error[E0308]: mismatched types
    --> library/std/src/io/mod.rs:2481:36
     |
     |
2481 |       fn lower_bound(&self) -> usize {
2482 | |
2482 | |
2483 | |     fn upper_bound(&self) -> Option<usize> {
2484 | |         None
2489 | |     }
2490 | | }
2490 | | }
     | |_^ expected `usize`, found `()`
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0308, E0401, E0412, E0422, E0599.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
