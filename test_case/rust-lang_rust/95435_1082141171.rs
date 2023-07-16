plain

---- [ui] ui/issues/issue-21763.rs stdout ----
diff of stderr:

8    = note: required because it appears within the type `(Rc<()>, Rc<()>)`
9    = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(Rc<()>, Rc<()>)>`
10 note: required because it appears within the type `hashbrown::map::HashMap<Rc<()>, Rc<()>, RandomState>`
-   --> /home/k1000/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.0/src/map.rs:188:12
12    |
12    |
13 LL | pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator + Clone = Global> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-21763.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21763.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `Rc<()>` cannot be sent between threads safely
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     foo::<HashMap<Rc<()>, Rc<()>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<()>` cannot be sent between threads safely
   |
   = help: within `(Rc<()>, Rc<()>)`, the trait `Send` is not implemented for `Rc<()>`
   = note: required because it appears within the type `(Rc<()>, Rc<()>)`
   = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(Rc<()>, Rc<()>)>`
note: required because it appears within the type `hashbrown::map::HashMap<Rc<()>, Rc<()>, RandomState>`
   |
   |
LL | pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator + Clone = Global> {
   |            ^^^^^^^
note: required because it appears within the type `HashMap<Rc<()>, Rc<()>>`
   |
   |
LL | pub struct HashMap<K, V, S = RandomState> {
note: required by a bound in `foo`
  --> /checkout/src/test/ui/issues/issue-21763.rs:6:11
   |
   |
LL | fn foo<T: Send>() {}
   |           ^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
