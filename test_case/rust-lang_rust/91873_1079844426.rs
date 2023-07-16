plain
......................................................iii........................................... 12700/12759
...........................................................
failures:

---- [ui] ui/traits/suggest-deferences/root-obligation.rs stdout ----

11 fn main() {
11 fn main() {
12     let _ = get_vowel_count("asdf");
- 
15 



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/root-obligation/root-obligation.fixed
To only update this specific test, also pass `--test-args traits/suggest-deferences/root-obligation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/root-obligation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/root-obligation" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/root-obligation/auxiliary"
stdout: none
--- stderr -------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0277]: expected a `Fn<(char,)>` closure, found `char`
  --> /checkout/src/test/ui/traits/suggest-deferences/root-obligation.rs:6:38
   |
LL |         .filter(|c| "aeiou".contains(c))
   |                             -------- ^ expected an `Fn<(char,)>` closure, found `char`
   |                             required by a bound introduced by this call
   |
   |
   = help: the trait `Fn<(char,)>` is not implemented for `char`
   = note: required because of the requirements on the impl of `FnOnce<(char,)>` for `&char`
   = note: required because of the requirements on the impl of `Pattern<'_>` for `&char`
note: required by a bound in `core::str::<impl str>::contains`
   |
   |
LL |     pub fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
   |                            ^^^^^^^^^^^ required by this bound in `core::str::<impl str>::contains`
help: consider dereferencing here
   |
LL |         .filter(|c| "aeiou".contains(*c))

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
