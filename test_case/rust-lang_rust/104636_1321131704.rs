plain
...........................................iii.......................................... 13816/13881
.................................................................
failures:

---- [ui] src/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash.rs stdout ----

22 
23 error: aborting due to 2 previous errors
24 
24 
- For more information about this error, try `rustc --explain E0225`.
+ Some errors have detailed explanations: E0225, E0782.
+ For more information about an error, try `rustc --explain E0225`.
26 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash/new-trait-with-all-as-supertraits-sug-name-clash.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/new-trait-with-all-as-supertraits-sug-name-clash.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash.rs:6:13
   |
LL | fn foo() -> NewTrait + NewTrait2 {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
help: add `dyn` keyword before this trait
   |
LL | fn foo() -> dyn NewTrait + NewTrait2 {}


error[E0225]: only auto traits can be used as additional traits in a trait object
  --> /checkout/src/test/ui/traits/new-trait-with-all-as-supertraits-sug-name-clash.rs:6:24
   |
LL | fn foo() -> NewTrait + NewTrait2 {}
   |             --------   ^^^^^^^^^ additional non-auto trait
   |             first non-auto trait
   |
   |
   = help: consider creating a new trait with all of these as supertraits and using that trait here instead: `trait NewTrait3: NewTrait + NewTrait2 {}`
   = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0225, E0782.
For more information about an error, try `rustc --explain E0225`.
