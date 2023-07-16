plain
[00:46:28] ....................................................................................................
[00:46:34] ....................................................................................................
[00:46:41] ....................................................................................................
[00:46:47] ....................................................................................................
[00:46:54] ..........................................i.........................................F...............
[00:47:06] .....................................ii.............................................................
[00:47:13] ....................................................................................................
[00:47:19] .................i....................................................................
[00:47:19] failures:
[00:47:19] failures:
[00:47:19] 
[00:47:19] ---- [ui (nll)] ui/nll/get_default.rs stdout ----
[00:47:19]  diff of stderr:
[00:47:19] 
[00:47:19] 4 LL |         match map.get() {
[00:47:19] 5    |               --- immutable borrow occurs here
[00:47:19] 6 ...
[00:47:19] - LL |                 map.set(String::new()); // Just AST errors here
[00:47:19] + LL |                 map.set(String::new()); // Ideally, this would not error.
[00:47:19] 8    |                 ^^^ mutable borrow occurs here
[00:47:19] 9 ...
[00:47:19] 10 LL | }
[00:47:19] 
[00:47:19] 11    | - immutable borrow ends he^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:47:19] +    |
[00:47:19] + note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 26:1...
[00:47:19] +   --> $DIR/get_default.rs:26:1
[00:47:19] +    |
[00:47:19] + LL | / fn ok(map: &mut Map) -> &String {
[00:47:19] + LL | |     loop {
[00:47:19] + LL | |         match map.get() {
[00:47:19] + LL | |             Some(v) => {
[00:47:19] + ...  |
[00:47:19] + LL | |     }
[00:47:19] + LL | | }
[00:47:19] + 
[00:47:19] + 
[00:47:19] + error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[00:47:19] +   --> $DIR/get_default.rs:45:17
[00:47:19] +    |
[00:47:19] + LL |         match map.get() {
[00:47:19] +    |               --- immutable borrow occurs here
[00:47:19] 42 LL |             Some(v) => {
[00:47:19] 43 LL |                 map.set(String::new()); // Both AST and MIR error here
[00:47:19] 44    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:47:19] 
[00:47:19] 46 LL |                 return v;
[00:47:19] 47    |                        - borrow later used here
[00:47:19] - error: aborting due to 4 previous errors
[00:47:19] - error: aborting due to 4 previous errors
[00:47:19] + error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[00:47:19] +   --> $DIR/get_default.rs:51:17
[00:47:19] +    |
[00:47:19] + LL |         match map.get() {
[00:47:19] +    |               --- immutable borrow occurs here
[00:47:19] + ...
[00:47:19] + LL |                 map.set(String::new()); // Ideally, just AST would error here
[00:47:19] +    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:47:19] +    |
[00:47:19] + note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 41:1...
[00:47:19] +   --> $DIR/get_default.rs:41:1
[00:47:19] +    |
[00:47:19] + LL | / fn err(map: &mut Map) -> &String {
[00:47:19] + LL | |     loop {
[00:47:19] + LL | |         match map.get() {
[00:47:19] + LL | |             Some(v) => {
[00:47:19] + ...  |
[00:47:19] + LL | |     }
[00:47:19] + LL | | }
[00:47:19] + 
[00:47:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:47:19] + error: aborting due to 6 previous errors
[00:47:19] 50 
[00:47:19] 50 
[00:47:19] 51 For more information about this error, try `rustc --explain E0502`.
[00:47:19] 52 
[00:47:19] 
[00:47:19] 
[00:47:19] The actual stderr differed from the expected stderr.
[00:47:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.nll.stderr
[00:47:19] To update references, run this command from build directory:
[00:47:19] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'nll/get_default.rs'
[00:47:19] error: 1 errors occurred comparing output.
[00:47:19] status: exit code: 101
[00:47:19] status: exit code: 101
[00:47:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/get_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:19] ------------------------------------------
[00:47:19] 
[00:47:19] ------------------------------------------
[00:47:19] stderr:
[00:47:19] stderr:
[00:47:19] ------------------------------------------
[00:47:19] {"message":"cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n