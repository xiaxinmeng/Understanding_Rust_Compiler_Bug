plain
............................................................iii......................... 12936/13015
...............................................................................
failures:

---- [ui] src/test/ui/suggestions/enum-method-probe.rs stdout ----

9     }
10 }
11 
11 
- fn test_result_in_result() -> Result<(), ()> { 
+ fn test_result_in_result() -> Result<(), ()> {
13     let res: Result<_, ()> = Ok(Foo);
14     res?.get();
15     //~^ ERROR no method named `get` found for enum `Result` in the current scope
17     Ok(())
18 }
19 
- fn test_result_in_plain() { 
- fn test_result_in_plain() { 
+ fn test_result_in_plain() {
21     let res: Result<_, ()> = Ok(Foo);
22     res.expect("REASON").get();
23     //~^ ERROR no method named `get` found for enum `Result` in the current scope

24     //~| HELP consider using `Result::expect` to unwrap the `Foo` value, panicking if the value is an `Err`
26 
- fn test_option_in_option() -> Option<()> { 
+ fn test_option_in_option() -> Option<()> {
28     let res: Option<_> = Some(Foo);
28     let res: Option<_> = Some(Foo);
29     res?.get();
30     //~^ ERROR no method named `get` found for enum `Option` in the current scope
32     Some(())
33 }
34 
- fn test_option_in_plain() { 
- fn test_option_in_plain() { 
+ fn test_option_in_plain() {
36     let res: Option<_> = Some(Foo);
37     res.expect("REASON").get();
38     //~^ ERROR no method named `get` found for enum `Option` in the current scope

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-method-probe/enum-method-probe.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/enum-method-probe.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/enum-method-probe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-method-probe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-method-probe/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `get` found for enum `Result` in the current scope
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:14:9
LL |     res.get();
LL |     res.get();
   |         ^^^ method not found in `Result<Foo, ()>`
   |
note: the method `get` exists on the type `Foo`
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:7:5
LL |     fn get(&self) -> u8 {
   |     ^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Foo` value, propagating a `Result::Err` value to the caller
   |
LL |     res?.get();

error[E0599]: no method named `get` found for enum `Result` in the current scope
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:22:9
   |
   |
LL |     res.get();
   |         ^^^ method not found in `Result<Foo, ()>`
   |
note: the method `get` exists on the type `Foo`
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:7:5
LL |     fn get(&self) -> u8 {
   |     ^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `Foo` value, panicking if the value is an `Err`
   |
LL |     res.expect("REASON").get();

error[E0599]: no method named `get` found for enum `Option` in the current scope
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:29:9
   |
   |
LL |     res.get();
   |         ^^^ method not found in `Option<Foo>`
   |
note: the method `get` exists on the type `Foo`
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:7:5
LL |     fn get(&self) -> u8 {
   |     ^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Foo` value, propagating a `None` to the caller
   |
LL |     res?.get();

error[E0599]: no method named `get` found for enum `Option` in the current scope
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:37:9
   |
   |
LL |     res.get();
   |         ^^^ method not found in `Option<Foo>`
   |
note: the method `get` exists on the type `Foo`
  --> /checkout/src/test/ui/suggestions/enum-method-probe.rs:7:5
LL |     fn get(&self) -> u8 {
   |     ^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^
help: consider using `Option::expect` to unwrap the `Foo` value, panicking if the value is `None`
   |
LL |     res.expect("REASON").get();

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
