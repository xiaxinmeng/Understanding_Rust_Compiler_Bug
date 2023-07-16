plain

---- compile_test stdout ----
diff of stderr:

+error: impl method assumes more implied bounds than the corresponding trait method
+  --> $DIR/others.rs:45:5
+   |
+LL |     fn deref(&self) -> &'static T {
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #105572 <https://github.com/rust-lang/rust/issues/105572>
+   = note: for more information, see issue #105572 <https://github.com/rust-lang/rust/issues/105572>
+   = note: `#[deny(implied_bounds_entailment)]` on by default
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:54:5
    |
 LL |     ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
    = note: `-D clippy::borrow-interior-mutable-const` implied by `-D warnings`
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:55:16
    |
 LL |     assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:58:22
    |
 LL |     let _once_ref = &ONCE_INIT; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:59:25
    |
 LL |     let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:60:27
    |
 LL |     let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:61:26
    |
 LL |     let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:72:14
    |
 LL |     let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:73:14
    |
 LL |     let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:74:19
    |
 LL |     let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:75:14
    |
 LL |     let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:76:13
    |
 LL |     let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:82:13
    |
 LL |     let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:87:5
    |
 LL |     CELL.set(2); //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
 
 error: a `const` item with interior mutability should not be borrowed
   --> $DIR/others.rs:88:16
    |
 LL |     assert_eq!(CELL.get(), 6); //~ ERROR interior mutability
    |
    |
    = help: assign this const to a local or static variable, and use the variable here
-error: aborting due to 14 previous errors
+error: aborting due to 15 previous errors
 
 
---
To only update this specific test, also pass `--test-args borrow_interior_mutable_const/others.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/borrow_interior_mutable_const/others.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_interior_mutable_const/others.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-992c1a552e84eb88.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ec78d46878d61bf1.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_interior_mutable_const/others.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"impl method assumes more implied bounds than the corresponding trait method","code":{"code":"implied_bounds_entailment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":1456,"byte_end":1485,"line_start":45,"line_end":45,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    fn deref(&self) -> &'static T {","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #105572 <https://github.com/rust-lang/rust/issues/105572>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(implied_bounds_entailment)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: impl method assumes more implied bounds than the corresponding trait method\n  --> tests/ui/borrow_interior_mutable_const/others.rs:45:5\n   |\nLL |     fn deref(&self) -> &'static T {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #105572 <https://github.com/rust-lang/rust/issues/105572>\n   = note: `#[deny(implied_bounds_entailment)]` on by default\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":1716,"byte_end":1722,"line_start":54,"line_end":54,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::borrow-interior-mutable-const` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:54:5\n   |\nLL |     ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability\n   |     ^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n   = note: `-D clippy::borrow-interior-mutable-const` implied by `-D warnings`\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":1796,"byte_end":1802,"line_start":55,"line_end":55,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"    assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:55:16\n   |\nLL |     assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability\n   |                ^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":1910,"byte_end":1919,"line_start":58,"line_end":58,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let _once_ref = &ONCE_INIT; //~ ERROR interior mutability","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:58:22\n   |\nLL |     let _once_ref = &ONCE_INIT; //~ ERROR interior mutability\n   |                      ^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":1975,"byte_end":1984,"line_start":59,"line_end":59,"column_start":25,"column_end":34,"is_primary":true,"text":[{"text":"    let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability","highlight_start":25,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:59:25\n   |\nLL |     let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability\n   |                         ^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2042,"byte_end":2051,"line_start":60,"line_end":60,"column_start":27,"column_end":36,"is_primary":true,"text":[{"text":"    let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability","highlight_start":27,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:60:27\n   |\nLL |     let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability\n   |                           ^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2108,"byte_end":2117,"line_start":61,"line_end":61,"column_start":26,"column_end":35,"is_primary":true,"text":[{"text":"    let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:61:26\n   |\nLL |     let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability\n   |                          ^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2577,"byte_end":2589,"line_start":72,"line_end":72,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:72:14\n   |\nLL |     let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability\n   |              ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2634,"byte_end":2646,"line_start":73,"line_end":73,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:73:14\n   |\nLL |     let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability\n   |              ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2698,"byte_end":2710,"line_start":74,"line_end":74,"column_start":19,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability","highlight_start":19,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:74:19\n   |\nLL |     let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability\n   |                   ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2758,"byte_end":2770,"line_start":75,"line_end":75,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:75:14\n   |\nLL |     let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability\n   |              ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":2819,"byte_end":2831,"line_start":76,"line_end":76,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:76:13\n   |\nLL |     let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability\n   |             ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":3058,"byte_end":3070,"line_start":82,"line_end":82,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:82:13\n   |\nLL |     let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability\n   |             ^^^^^^^^^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":3211,"byte_end":3215,"line_start":87,"line_end":87,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    CELL.set(2); //~ ERROR interior mutability","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:87:5\n   |\nLL |     CELL.set(2); //~ ERROR interior mutability\n   |     ^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}
{"message":"a `const` item with interior mutability should not be borrowed","code":{"code":"clippy::borrow_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_interior_mutable_const/others.rs","byte_start":3269,"byte_end":3273,"line_start":88,"line_end":88,"column_start":16,"column_end":20,"is_primary":true,"text":[{"text":"    assert_eq!(CELL.get(), 6); //~ ERROR interior mutability","highlight_start":16,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"assign this const to a local or static variable, and use the variable here","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item with interior mutability should not be borrowed\n  --> tests/ui/borrow_interior_mutable_const/others.rs:88:16\n   |\nLL |     assert_eq!(CELL.get(), 6); //~ ERROR interior mutability\n   |                ^^^^\n   |\n   = help: assign this const to a local or static variable, and use the variable here\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
