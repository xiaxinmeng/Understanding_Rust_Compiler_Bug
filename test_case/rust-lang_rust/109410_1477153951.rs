plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

---- compile_test stdout ----
diff of stderr:

 error: called `assert!` with `Result::is_ok`
    |
    |
 LL |     assert!(r.is_ok());
    |     ^^^^^^^^^^^^^^^^^^ help: replace with: `r.unwrap()`
    |
    = note: `-D clippy::assertions-on-result-states` implied by `-D warnings`
 
 error: called `assert!` with `Result::is_ok`
    |
    |
 LL |     assert!(get_ok().is_ok());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `get_ok().unwrap()`
 
 error: called `assert!` with `Result::is_ok`
    |
    |
 LL |     assert!(get_ok_macro!().is_ok());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `get_ok_macro!().unwrap()`
 
-error: called `assert!` with `Result::is_ok`
-   |
-   |
-LL |     assert!(r.is_ok());
-   |     ^^^^^^^^^^^^^^^^^^ help: replace with: `r.unwrap()`
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
 
 
-error: called `assert!` with `Result::is_ok`
-   |
-   |
-LL |         assert!(r.is_ok());
-   |         ^^^^^^^^^^^^^^^^^^ help: replace with: `r.unwrap()`
 
 
-error: called `assert!` with `Result::is_err`
-   |
-   |
-LL |     assert!(r.is_err());
-   |     ^^^^^^^^^^^^^^^^^^^ help: replace with: `r.unwrap_err()`
 
 
-error: called `assert!` with `Result::is_err`
-   |
-   |
-LL |     assert!(res.is_err())
-   |     ^^^^^^^^^^^^^^^^^^^^^ help: replace with: `res.unwrap_err();`
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
-error: aborting due to 7 previous errors
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to 3 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/assertions_on_result_states.stage-id.stderr
thread '[ui] ui/assertions_on_result_states.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 4, column: 2)', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/runtest.rs:2397:15
diff of stderr:


 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:6:9
    |
 LL |     let b = x.borrow();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:6:5
    |
 LL | /     let b = x.borrow();
 LL | |     baz().await
 LL | | }
    | |_^
    = note: `-D clippy::await-holding-refcell-ref` implied by `-D warnings`
 
 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:11:9
    |
 LL |     let b = x.borrow_mut();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:11:5
    |
 LL | /     let b = x.borrow_mut();
 LL | |     baz().await
 LL | | }
 
 
 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:32:9
    |
 LL |     let b = x.borrow_mut();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:32:5
    |
 LL | /     let b = x.borrow_mut();
 LL | |
 LL | |     let second = baz().await;
 LL | |
 ...  |
 LL | |     first + second + third
 LL | | }
 
 
 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:44:9
    |
 LL |     let b = x.borrow_mut();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:44:5
    |
 LL | /     let b = x.borrow_mut();
 LL | |
 LL | |     let second = baz().await;
 LL | |
 ...  |
 LL | |     first + second + third
 LL | | }
 
 
 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:59:13
    |
 LL |         let b = x.borrow_mut();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:59:9
    |
 LL | /         let b = x.borrow_mut();
 LL | |         baz().await
 LL | |     };
 
 
 error: this `RefCell` reference is held across an `await` point
   --> $DIR/await_holding_refcell_ref.rs:71:13
    |
 LL |         let b = x.borrow_mut();
    |
    |
    = help: ensure the reference is dropped before calling `await`
 note: these are all the `await` points this reference is held through
   --> $DIR/await_holding_refcell_ref.rs:71:9
    |
 LL | /         let b = x.borrow_mut();
 LL | |         baz().await
 LL | |     }
 
 
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
+
+error: internal compiler error: unexpected panic
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/await_holding_refcell_ref.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args await_holding_refcell_ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/await_holding_refcell_ref.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/await_holding_refcell_ref.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/await_holding_refcell_ref.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":118,"byte_end":119,"line_start":6,"line_end":6,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let b = x.borrow();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":114,"byte_end":151,"line_start":6,"line_end":8,"column_start":5,"column_end":2,"is_primary":true,"text":[{"text":"    let b = x.borrow();","highlight_start":5,"highlight_end":24},{"text":"    baz().await","highlight_start":1,"highlight_end":16},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`-D clippy::await-holding-refcell-ref` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:6:9\n   |\nLL |     let b = x.borrow();\n   |         ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:6:5\n   |\nLL | /     let b = x.borrow();\nLL | |     baz().await\nLL | | }\n   | |_^\n   = note: `-D clippy::await-holding-refcell-ref` implied by `-D warnings`\n\n"}
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":205,"byte_end":206,"line_start":11,"line_end":11,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":201,"byte_end":242,"line_start":11,"line_end":13,"column_start":5,"column_end":2,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":5,"highlight_end":28},{"text":"    baz().await","highlight_start":1,"highlight_end":16},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:11:9\n   |\nLL |     let b = x.borrow_mut();\n   |         ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:11:5\n   |\nLL | /     let b = x.borrow_mut();\nLL | |     baz().await\nLL | | }\n   | |_^\n\n"}
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":525,"byte_end":526,"line_start":32,"line_end":32,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":521,"byte_end":635,"line_start":32,"line_end":39,"column_start":5,"column_end":2,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":5,"highlight_end":28},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    let second = baz().await;","highlight_start":1,"highlight_end":30},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    let third = baz().await;","highlight_start":1,"highlight_end":29},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    first + second + third","highlight_start":1,"highlight_end":27},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:32:9\n   |\nLL |     let b = x.borrow_mut();\n   |         ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:32:5\n   |\nLL | /     let b = x.borrow_mut();\nLL | |\nLL | |     let second = baz().await;\nLL | |\n...  |\nLL | |     first + second + third\nLL | | }\n   | |_^\n\n"}
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":720,"byte_end":721,"line_start":44,"line_end":44,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":716,"byte_end":844,"line_start":44,"line_end":53,"column_start":5,"column_end":2,"is_primary":true,"text":[{"text":"    let b = x.borrow_mut();","highlight_start":5,"highlight_end":28},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    let second = baz().await;","highlight_start":1,"highlight_end":30},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    drop(b);","highlight_start":1,"highlight_end":13},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    let third = baz().await;","highlight_start":1,"highlight_end":29},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    first + second + third","highlight_start":1,"highlight_end":27},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:44:9\n   |\nLL |     let b = x.borrow_mut();\n   |         ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:44:5\n   |\nLL | /     let b = x.borrow_mut();\nLL | |\nLL | |     let second = baz().await;\nLL | |\n...  |\nLL | |     first + second + third\nLL | | }\n   | |_^\n\n"}
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":952,"byte_end":953,"line_start":59,"line_end":59,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let b = x.borrow_mut();","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":948,"byte_end":997,"line_start":59,"line_end":61,"column_start":9,"column_end":6,"is_primary":true,"text":[{"text":"        let b = x.borrow_mut();","highlight_start":9,"highlight_end":32},{"text":"        baz().await","highlight_start":1,"highlight_end":20},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:59:13\n   |\nLL |         let b = x.borrow_mut();\n   |             ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:59:9\n   |\nLL | /         let b = x.borrow_mut();\nLL | |         baz().await\nLL | |     };\n   | |_____^\n\n"}
{"message":"this `RefCell` reference is held across an `await` point","code":{"code":"clippy::await_holding_refcell_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":1203,"byte_end":1204,"line_start":71,"line_end":71,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"        let b = x.borrow_mut();","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure the reference is dropped before calling `await`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"these are all the `await` points this reference is held through","code":null,"level":"note","spans":[{"file_name":"tests/ui/await_holding_refcell_ref.rs","byte_start":1199,"byte_end":1248,"line_start":71,"line_end":73,"column_start":9,"column_end":6,"is_primary":true,"text":[{"text":"        let b = x.borrow_mut();","highlight_start":9,"highlight_end":32},{"text":"        baz().await","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `RefCell` reference is held across an `await` point\n  --> tests/ui/await_holding_refcell_ref.rs:71:13\n   |\nLL |         let b = x.borrow_mut();\n   |             ^\n   |\n   = help: ensure the reference is dropped before calling `await`\nnote: these are all the `await` points this reference is held through\n  --> tests/ui/await_holding_refcell_ref.rs:71:9\n   |\nLL | /         let b = x.borrow_mut();\nLL | |         baz().await\nLL | |     }\n   | |_____^\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}

------------------------------------------


diff of stderr:

 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL |     let foo: &Box<bool>;
    |              ^^^^^^^^^^ help: try: `&bool`
 note: the lint level is defined here
   --> $DIR/borrow_box.rs:1:9
    |
 LL | #![deny(clippy::borrowed_box)]
 LL | #![deny(clippy::borrowed_box)]
    |         ^^^^^^^^^^^^^^^^^^^^
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL |     foo: &'a Box<bool>,
    |          ^^^^^^^^^^^^^ help: try: `&'a bool`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL |     fn test4(a: &Box<bool>);
    |                 ^^^^^^^^^^ help: try: `&bool`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test14(_display: &Box<dyn Display>) {}
    |                         ^^^^^^^^^^^^^^^^^ help: try: `&dyn Display`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test15(_display: &Box<dyn Display + Send>) {}
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(dyn Display + Send)`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test16<'a>(_display: &'a Box<dyn Display + 'a>) {}
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&'a (dyn Display + 'a)`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test17(_display: &Box<impl Display>) {}
    |                         ^^^^^^^^^^^^^^^^^^ help: try: `&impl Display`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test18(_display: &Box<impl Display + Send>) {}
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(impl Display + Send)`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test19<'a>(_display: &'a Box<impl Display + 'a>) {}
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&'a (impl Display + 'a)`
 
 error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
    |
    |
 LL | pub fn test20(_display: &Box<(dyn Display + Send)>) {}
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(dyn Display + Send)`
 
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
+
+error: internal compiler error: unexpected panic
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
 error: aborting due to 10 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_box.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrow_box.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/borrow_box.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_box.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_box.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":646,"byte_end":656,"line_start":20,"line_end":20,"column_start":14,"column_end":24,"is_primary":true,"text":[{"text":"    let foo: &Box<bool>;","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":8,"byte_end":28,"line_start":1,"line_end":1,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![deny(clippy::borrowed_box)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":646,"byte_end":656,"line_start":20,"line_end":20,"column_start":14,"column_end":24,"is_primary":true,"text":[{"text":"    let foo: &Box<bool>;","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":"&bool","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:20:14\n   |\nLL |     let foo: &Box<bool>;\n   |              ^^^^^^^^^^ help: try: `&bool`\n   |\nnote: the lint level is defined here\n  --> tests/ui/borrow_box.rs:1:9\n   |\nLL | #![deny(clippy::borrowed_box)]\n   |         ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":689,"byte_end":702,"line_start":24,"line_end":24,"column_start":10,"column_end":23,"is_primary":true,"text":[{"text":"    foo: &'a Box<bool>,","highlight_start":10,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":689,"byte_end":702,"line_start":24,"line_end":24,"column_start":10,"column_end":23,"is_primary":true,"text":[{"text":"    foo: &'a Box<bool>,","highlight_start":10,"highlight_end":23}],"label":null,"suggested_replacement":"&'a bool","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:24:10\n   |\nLL |     foo: &'a Box<bool>,\n   |          ^^^^^^^^^^^^^ help: try: `&'a bool`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":737,"byte_end":747,"line_start":28,"line_end":28,"column_start":17,"column_end":27,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<bool>);","highlight_start":17,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":737,"byte_end":747,"line_start":28,"line_end":28,"column_start":17,"column_end":27,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<bool>);","highlight_start":17,"highlight_end":27}],"label":null,"suggested_replacement":"&bool","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:28:17\n   |\nLL |     fn test4(a: &Box<bool>);\n   |                 ^^^^^^^^^^ help: try: `&bool`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1974,"byte_end":1991,"line_start":94,"line_end":94,"column_start":25,"column_end":42,"is_primary":true,"text":[{"text":"pub fn test14(_display: &Box<dyn Display>) {}","highlight_start":25,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1974,"byte_end":1991,"line_start":94,"line_end":94,"column_start":25,"column_end":42,"is_primary":true,"text":[{"text":"pub fn test14(_display: &Box<dyn Display>) {}","highlight_start":25,"highlight_end":42}],"label":null,"suggested_replacement":"&dyn Display","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:94:25\n   |\nLL | pub fn test14(_display: &Box<dyn Display>) {}\n   |                         ^^^^^^^^^^^^^^^^^ help: try: `&dyn Display`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2020,"byte_end":2044,"line_start":95,"line_end":95,"column_start":25,"column_end":49,"is_primary":true,"text":[{"text":"pub fn test15(_display: &Box<dyn Display + Send>) {}","highlight_start":25,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2020,"byte_end":2044,"line_start":95,"line_end":95,"column_start":25,"column_end":49,"is_primary":true,"text":[{"text":"pub fn test15(_display: &Box<dyn Display + Send>) {}","highlight_start":25,"highlight_end":49}],"label":null,"suggested_replacement":"&(dyn Display + Send)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:95:25\n   |\nLL | pub fn test15(_display: &Box<dyn Display + Send>) {}\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(dyn Display + Send)`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2077,"byte_end":2102,"line_start":96,"line_end":96,"column_start":29,"column_end":54,"is_primary":true,"text":[{"text":"pub fn test16<'a>(_display: &'a Box<dyn Display + 'a>) {}","highlight_start":29,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2077,"byte_end":2102,"line_start":96,"line_end":96,"column_start":29,"column_end":54,"is_primary":true,"text":[{"text":"pub fn test16<'a>(_display: &'a Box<dyn Display + 'a>) {}","highlight_start":29,"highlight_end":54}],"label":null,"suggested_replacement":"&'a (dyn Display + 'a)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:96:29\n   |\nLL | pub fn test16<'a>(_display: &'a Box<dyn Display + 'a>) {}\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&'a (dyn Display + 'a)`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2132,"byte_end":2150,"line_start":98,"line_end":98,"column_start":25,"column_end":43,"is_primary":true,"text":[{"text":"pub fn test17(_display: &Box<impl Display>) {}","highlight_start":25,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2132,"byte_end":2150,"line_start":98,"line_end":98,"column_start":25,"column_end":43,"is_primary":true,"text":[{"text":"pub fn test17(_display: &Box<impl Display>) {}","highlight_start":25,"highlight_end":43}],"label":null,"suggested_replacement":"&impl Display","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:98:25\n   |\nLL | pub fn test17(_display: &Box<impl Display>) {}\n   |                         ^^^^^^^^^^^^^^^^^^ help: try: `&impl Display`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2179,"byte_end":2204,"line_start":99,"line_end":99,"column_start":25,"column_end":50,"is_primary":true,"text":[{"text":"pub fn test18(_display: &Box<impl Display + Send>) {}","highlight_start":25,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2179,"byte_end":2204,"line_start":99,"line_end":99,"column_start":25,"column_end":50,"is_primary":true,"text":[{"text":"pub fn test18(_display: &Box<impl Display + Send>) {}","highlight_start":25,"highlight_end":50}],"label":null,"suggested_replacement":"&(impl Display + Send)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:99:25\n   |\nLL | pub fn test18(_display: &Box<impl Display + Send>) {}\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(impl Display + Send)`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2237,"byte_end":2263,"line_start":100,"line_end":100,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"pub fn test19<'a>(_display: &'a Box<impl Display + 'a>) {}","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2237,"byte_end":2263,"line_start":100,"line_end":100,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"pub fn test19<'a>(_display: &'a Box<impl Display + 'a>) {}","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":"&'a (impl Display + 'a)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:100:29\n   |\nLL | pub fn test19<'a>(_display: &'a Box<impl Display + 'a>) {}\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&'a (impl Display + 'a)`\n\n"}
{"message":"you seem to be trying to use `&Box<T>`. Consider using just `&T`","code":{"code":"clippy::borrowed_box","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2526,"byte_end":2552,"line_start":105,"line_end":105,"column_start":25,"column_end":51,"is_primary":true,"text":[{"text":"pub fn test20(_display: &Box<(dyn Display + Send)>) {}","highlight_start":25,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":2526,"byte_end":2552,"line_start":105,"line_end":105,"column_start":25,"column_end":51,"is_primary":true,"text":[{"text":"pub fn test20(_display: &Box<(dyn Display + Send)>) {}","highlight_start":25,"highlight_end":51}],"label":null,"suggested_replacement":"&(dyn Display + Send)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `&Box<T>`. Consider using just `&T`\n  --> tests/ui/borrow_box.rs:105:25\n   |\nLL | pub fn test20(_display: &Box<(dyn Display + Send)>) {}\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&(dyn Display + Send)`\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}

------------------------------------------


diff of stderr:

 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:9:13
    |
 LL |         let guard = x.lock().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:9:9
    |
 LL | /         let guard = x.lock().unwrap();
 LL | |         baz().await
 LL | |     }
    | |_____^
    = note: `-D clippy::await-holding-lock` implied by `-D warnings`
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:24:13
    |
 LL |         let guard = x.read().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:24:9
    |
 LL | /         let guard = x.read().unwrap();
 LL | |         baz().await
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:29:13
    |
 LL |         let mut guard = x.write().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:29:9
    |
 LL | /         let mut guard = x.write().unwrap();
 LL | |         baz().await
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:50:13
    |
 LL |         let guard = x.lock().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:50:9
    |
 LL | /         let guard = x.lock().unwrap();
 LL | |
 LL | |         let second = baz().await;
 LL | |
 ...  |
 LL | |         first + second + third
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:63:17
    |
 LL |             let guard = x.lock().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:63:13
    |
 LL | /             let guard = x.lock().unwrap();
 LL | |             baz().await
 LL | |         };
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:75:17
    |
 LL |             let guard = x.lock().unwrap();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:75:13
    |
 LL | /             let guard = x.lock().unwrap();
 LL | |             baz().await
 LL | |         }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:87:13
    |
 LL |         let guard = x.lock();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:87:9
    |
 LL | /         let guard = x.lock();
 LL | |         baz().await
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:102:13
    |
 LL |         let guard = x.read();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:102:9
    |
 LL | /         let guard = x.read();
 LL | |         baz().await
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:107:13
    |
 LL |         let mut guard = x.write();
    |
    |
    = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
 note: these are all the `await` points this lock is held through
   --> $DIR/await_holding_lock.rs:107:9
    |
 LL | /         let mut guard = x.write();
 LL | |         baz().await
 LL | |     }
 
 
 error: this `MutexGuard` is held across an `await` point
   --> $DIR/await_holding_lock.rs:128:13
    |
 LL |         let guard = x.lock();
    |
    |
---
 
-error: mutable key type
-  --> $DIR/mut_key.rs:74:5
-   |
-LL |     let _map = HashMap::<Vec<Cell<usize>>, usize>::new();
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
 
-error: mutable key type
-  --> $DIR/mut_key.rs:75:5
-  --> $DIR/mut_key.rs:75:5
-   |
-LL |     let _map = HashMap::<BTreeMap<Cell<usize>, ()>, usize>::new();
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
 
-error: mutable key type
-  --> $DIR/mut_key.rs:76:5
-  --> $DIR/mut_key.rs:76:5
-   |
-LL |     let _map = HashMap::<BTreeMap<(), Cell<usize>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:77:5
-   |
-   |
-LL |     let _map = HashMap::<BTreeSet<Cell<usize>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:78:5
-   |
-   |
-LL |     let _map = HashMap::<Option<Cell<usize>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:79:5
-   |
-   |
-LL |     let _map = HashMap::<Option<Vec<Cell<usize>>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:80:5
-   |
-   |
-LL |     let _map = HashMap::<Result<&mut usize, ()>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:82:5
-   |
-   |
-LL |     let _map = HashMap::<Box<Cell<usize>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:83:5
-   |
-   |
-LL |     let _map = HashMap::<Rc<Cell<usize>>, usize>::new();
-
-error: mutable key type
-  --> $DIR/mut_key.rs:84:5
-   |
-   |
-LL |     let _map = HashMap::<Arc<Cell<usize>>, usize>::new();
-
-error: aborting due to 17 previous errors
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to 4 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/mut_key.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mut_key.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/mut_key.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/mut_key.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/mut_key.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":637,"byte_end":661,"line_start":30,"line_end":30,"column_start":32,"column_end":56,"is_primary":true,"text":[{"text":"fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {","highlight_start":32,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::mutable-key-type` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:30:32\n   |\nLL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::mutable-key-type` implied by `-D warnings`\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":677,"byte_end":689,"line_start":30,"line_end":30,"column_start":72,"column_end":84,"is_primary":true,"text":[{"text":"fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {","highlight_start":72,"highlight_end":84}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:30:72\n   |\nLL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {\n   |                                                                        ^^^^^^^^^^^^\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":696,"byte_end":744,"line_start":31,"line_end":31,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    let _other: HashMap<Key, bool> = HashMap::new();","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:31:5\n   |\nLL |     let _other: HashMap<Key, bool> = HashMap::new();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":1547,"byte_end":1575,"line_start":58,"line_end":58,"column_start":22,"column_end":50,"is_primary":true,"text":[{"text":"fn tuples_bad<U>(_m: &mut HashMap<(Key, U), bool>) {}","highlight_start":22,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:58:22\n   |\nLL | fn tuples_bad<U>(_m: &mut HashMap<(Key, U), bool>) {}\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}

------------------------------------------


diff of stderr:

 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:59:14
    |
 LL |         Some(ref x) => x,
    |              ^^^^^ help: try this: `x`
    |
    = note: `-D clippy::needless-borrow` implied by `-D warnings`
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:65:14
    |
    |
 LL |         Some(ref x) => *x,
    |
 help: try this
    |
    |
 LL |         Some(x) => x,
 
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:71:14
    |
    |
 LL |         Some(ref x) => {
    |              ^^^^^
    |
 help: try this
    |
 LL ~         Some(x) => {
 LL |             f1(x);
 LL ~             f1(x);
 
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:81:14
    |
    |
 LL |         Some(ref x) => m1!(x),
    |              ^^^^^ help: try this: `x`
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:86:15
    |
    |
 LL |     let _ = |&ref x: &&String| {
    |               ^^^^^ help: try this: `x`
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:91:10
    |
    |
 LL |     let (ref y,) = (&x,);
    |
 help: try this
    |
    |
 LL ~     let (y,) = (&x,);
 LL ~     let _: &String = y;
 
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:101:14
    |
    |
 LL |         Some(ref x) => x.0,
    |              ^^^^^ help: try this: `x`
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:111:14
    |
    |
 LL |         E::A(ref x) | E::B(ref x) => *x,
    |
 help: try this
    |
    |
 LL |         E::A(x) | E::B(x) => x,
 
 error: this pattern creates a reference to a reference
   --> $DIR/needless_borrow_pat.rs:117:21
    |
    |
 LL |         if let Some(ref x) = Some(&String::new());
    |                     ^^^^^ help: try this: `x`
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:125:12
-   |
-   |
-LL | fn f2<'a>(&ref x: &&'a String) -> &'a String {
-   |
-help: try this
-   |
-   |
-LL ~ fn f2<'a>(&x: &&'a String) -> &'a String {
-LL |     let _: &String = x;
-LL ~     x
-   |
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
 
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:132:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |           ^^^^^ help: try this: `x`
 
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:140:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |
-help: try this
-   |
-   |
-LL ~     fn f(&x: &&String) {
-LL ~         let _: &String = x;
+note: the compiler unexpectedly panicked. this is a bug.
 
-error: aborting due to 12 previous errors
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to 9 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_borrow_pat.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args needless_borrow_pat.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_borrow_pat.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_borrow_pat.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_borrow_pat.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1177,"byte_end":1182,"line_start":59,"line_end":59,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-borrow` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1177,"byte_end":1182,"line_start":59,"line_end":59,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:59:14\n   |\nLL |         Some(ref x) => x,\n   |              ^^^^^ help: try this: `x`\n   |\n   = note: `-D clippy::needless-borrow` implied by `-D warnings`\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1309,"byte_end":1314,"line_start":65,"line_end":65,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => *x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1309,"byte_end":1314,"line_start":65,"line_end":65,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => *x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1319,"byte_end":1321,"line_start":65,"line_end":65,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"        Some(ref x) => *x,","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:65:14\n   |\nLL |         Some(ref x) => *x,\n   |              ^^^^^\n   |\nhelp: try this\n   |\nLL |         Some(x) => x,\n   |              ~     ~\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1441,"byte_end":1446,"line_start":71,"line_end":71,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1441,"byte_end":1446,"line_start":71,"line_end":71,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1487,"byte_end":1489,"line_start":73,"line_end":73,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"            f1(*x);","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:71:14\n   |\nLL |         Some(ref x) => {\n   |              ^^^^^\n   |\nhelp: try this\n   |\nLL ~         Some(x) => {\nLL |             f1(x);\nLL ~             f1(x);\n   |\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1618,"byte_end":1623,"line_start":81,"line_end":81,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => m1!(x),","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1618,"byte_end":1623,"line_start":81,"line_end":81,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => m1!(x),","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:81:14\n   |\nLL |         Some(ref x) => m1!(x),\n   |              ^^^^^ help: try this: `x`\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1717,"byte_end":1722,"line_start":86,"line_end":86,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = |&ref x: &&String| {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1717,"byte_end":1722,"line_start":86,"line_end":86,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = |&ref x: &&String| {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:86:15\n   |\nLL |     let _ = |&ref x: &&String| {\n   |               ^^^^^ help: try this: `x`\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1816,"byte_end":1821,"line_start":91,"line_end":91,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"    let (ref y,) = (&x,);","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1816,"byte_end":1821,"line_start":91,"line_end":91,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"    let (ref y,) = (&x,);","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":"y","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1854,"byte_end":1856,"line_start":92,"line_end":92,"column_start":22,"column_end":24,"is_primary":true,"text":[{"text":"    let _: &String = *y;","highlight_start":22,"highlight_end":24}],"label":null,"suggested_replacement":"y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:91:10\n   |\nLL |     let (ref y,) = (&x,);\n   |          ^^^^^\n   |\nhelp: try this\n   |\nLL ~     let (y,) = (&x,);\nLL ~     let _: &String = y;\n   |\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2079,"byte_end":2084,"line_start":101,"line_end":101,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => x.0,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2079,"byte_end":2084,"line_start":101,"line_end":101,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        Some(ref x) => x.0,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:101:14\n   |\nLL |         Some(ref x) => x.0,\n   |              ^^^^^ help: try this: `x`\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2274,"byte_end":2279,"line_start":111,"line_end":111,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        E::A(ref x) | E::B(ref x) => *x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2288,"byte_end":2293,"line_start":111,"line_end":111,"column_start":28,"column_end":33,"is_primary":true,"text":[{"text":"        E::A(ref x) | E::B(ref x) => *x,","highlight_start":28,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2274,"byte_end":2279,"line_start":111,"line_end":111,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"        E::A(ref x) | E::B(ref x) => *x,","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2288,"byte_end":2293,"line_start":111,"line_end":111,"column_start":28,"column_end":33,"is_primary":true,"text":[{"text":"        E::A(ref x) | E::B(ref x) => *x,","highlight_start":28,"highlight_end":33}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2298,"byte_end":2300,"line_start":111,"line_end":111,"column_start":38,"column_end":40,"is_primary":true,"text":[{"text":"        E::A(ref x) | E::B(ref x) => *x,","highlight_start":38,"highlight_end":40}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:111:14\n   |\nLL |         E::A(ref x) | E::B(ref x) => *x,\n   |              ^^^^^         ^^^^^\n   |\nhelp: try this\n   |\nLL |         E::A(x) | E::B(x) => x,\n   |              ~         ~     ~\n\n"}
{"message":"this pattern creates a reference to a reference","code":{"code":"clippy::needless_borrow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2397,"byte_end":2402,"line_start":117,"line_end":117,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"        if let Some(ref x) = Some(&String::new());","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":2397,"byte_end":2402,"line_start":117,"line_end":117,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"        if let Some(ref x) = Some(&String::new());","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern creates a reference to a reference\n  --> tests/ui/needless_borrow_pat.rs:117:21\n   |\nLL |         if let Some(ref x) = Some(&String::new());\n   |                     ^^^^^ help: try this: `x`\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}

------------------------------------------



error: auxiliary build of "tests/ui/auxiliary/proc_macro_attr.rs" failed to compile: 
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/auxiliary/proc_macro_attr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--emit=link" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate

------------------------------------------

diff of stderr:
diff of stderr:

 error: the following explicit lifetimes could be elided: 'a, 'b
   --> $DIR/needless_lifetimes.rs:18:1
    |
 LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
    |
    |
    = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
 help: elide the lifetimes
    |
 LL - fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
 LL + fn distinct_lifetimes(_x: &u8, _y: &u8, _z: u8) {}
 
 
 error: the following explicit lifetimes could be elided: 'a, 'b
   --> $DIR/needless_lifetimes.rs:20:1
    |
 LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
    |
 help: elide the lifetimes
    |
    |
 LL - fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
 LL + fn distinct_and_static(_x: &u8, _y: &u8, _z: &'static u8) {}
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:30:1
    |
 LL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
    |
 help: elide the lifetimes
    |
    |
 LL - fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
 LL + fn in_and_out(x: &u8, _y: u8) -> &u8 {
 
 
 error: the following explicit lifetimes could be elided: 'b
   --> $DIR/needless_lifetimes.rs:42:1
    |
 LL | fn multiple_in_and_out_2a<'a, 'b>(x: &'a u8, _y: &'b u8) -> &'a u8 {
    |
 help: elide the lifetimes
    |
    |
 LL - fn multiple_in_and_out_2a<'a, 'b>(x: &'a u8, _y: &'b u8) -> &'a u8 {
 LL + fn multiple_in_and_out_2a<'a>(x: &'a u8, _y: &u8) -> &'a u8 {
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:49:1
    |
 LL | fn multiple_in_and_out_2b<'a, 'b>(_x: &'a u8, y: &'b u8) -> &'b u8 {
    |
 help: elide the lifetimes
    |
    |
 LL - fn multiple_in_and_out_2b<'a, 'b>(_x: &'a u8, y: &'b u8) -> &'b u8 {
 LL + fn multiple_in_and_out_2b<'b>(_x: &u8, y: &'b u8) -> &'b u8 {
 
 
 error: the following explicit lifetimes could be elided: 'b
   --> $DIR/needless_lifetimes.rs:66:1
    |
 LL | fn deep_reference_1a<'a, 'b>(x: &'a u8, _y: &'b u8) -> Result<&'a u8, ()> {
    |
 help: elide the lifetimes
    |
    |
 LL - fn deep_reference_1a<'a, 'b>(x: &'a u8, _y: &'b u8) -> Result<&'a u8, ()> {
 LL + fn deep_reference_1a<'a>(x: &'a u8, _y: &u8) -> Result<&'a u8, ()> {
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:73:1
    |
 LL | fn deep_reference_1b<'a, 'b>(_x: &'a u8, y: &'b u8) -> Result<&'b u8, ()> {
    |
 help: elide the lifetimes
    |
    |
 LL - fn deep_reference_1b<'a, 'b>(_x: &'a u8, y: &'b u8) -> Result<&'b u8, ()> {
 LL + fn deep_reference_1b<'b>(_x: &u8, y: &'b u8) -> Result<&'b u8, ()> {
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:82:1
    |
 LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
    |
 help: elide the lifetimes
    |
    |
 LL - fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
 LL + fn deep_reference_3(x: &u8, _y: u8) -> Result<&u8, ()> {
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:87:1
    |
 LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
    |
 help: elide the lifetimes
    |
    |
 LL - fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
 LL + fn where_clause_without_lt<T>(x: &u8, _y: u8) -> Result<&u8, ()>
 
 
 error: the following explicit lifetimes could be elided: 'a, 'b
   --> $DIR/needless_lifetimes.rs:99:1
    |
 LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
    |
 help: elide the lifetimes
    |
    |
 LL - fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
 LL + fn lifetime_param_2(_x: Ref<'_>, _y: &u8) {}
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:123:1
    |
 LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
    |
 help: elide the lifetimes
    |
    |
 LL - fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
 LL + fn fn_bound_2<F, I>(_m: Lt<'_, I>, _f: F) -> Lt<'_, I>
 
 
 error: the following explicit lifetimes could be elided: 's
   --> $DIR/needless_lifetimes.rs:153:5
    |
 LL |     fn self_and_out<'s>(&'s self) -> &'s u8 {
    |
 help: elide the lifetimes
    |
    |
 LL -     fn self_and_out<'s>(&'s self) -> &'s u8 {
 LL +     fn self_and_out(&self) -> &u8 {
 
 
 error: the following explicit lifetimes could be elided: 't
   --> $DIR/needless_lifetimes.rs:160:5
    |
 LL |     fn self_and_in_out_1<'s, 't>(&'s self, _x: &'t u8) -> &'s u8 {
    |
 help: elide the lifetimes
    |
    |
 LL -     fn self_and_in_out_1<'s, 't>(&'s self, _x: &'t u8) -> &'s u8 {
 LL +     fn self_and_in_out_1<'s>(&'s self, _x: &u8) -> &'s u8 {
 
 
 error: the following explicit lifetimes could be elided: 's
   --> $DIR/needless_lifetimes.rs:167:5
    |
 LL |     fn self_and_in_out_2<'s, 't>(&'s self, x: &'t u8) -> &'t u8 {
    |
 help: elide the lifetimes
    |
    |
 LL -     fn self_and_in_out_2<'s, 't>(&'s self, x: &'t u8) -> &'t u8 {
 LL +     fn self_and_in_out_2<'t>(&self, x: &'t u8) -> &'t u8 {
 
 
 error: the following explicit lifetimes could be elided: 's, 't
   --> $DIR/needless_lifetimes.rs:171:5
    |
 LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
    |
 help: elide the lifetimes
    |
    |
 LL -     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
 LL +     fn distinct_self_and_in(&self, _x: &u8) {}
 
 
 error: the following explicit lifetimes could be elided: 'a
   --> $DIR/needless_lifetimes.rs:190:1
    |
 LL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
    |
 help: elide the lifetimes
    |
    |
 LL - fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
 LL + fn struct_with_lt(_foo: Foo<'_>) -> &str {
 
 
 error: the following explicit lifetimes could be elided: 'b
   --> $DIR/needless_lifetimes.rs:208:1
---
-
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:91:30
-   |
-LL |     require_impl_deref_c_str(c_str.to_owned());
-   |                              ^^^^^^^^^^^^^^^^ help: use: `c_str`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:92:31
-   |
-   |
-LL |     require_impl_deref_os_str(os_str.to_owned());
-   |                               ^^^^^^^^^^^^^^^^^ help: use: `os_str`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:93:29
-   |
-LL |     require_impl_deref_path(path.to_owned());
-LL |     require_impl_deref_path(path.to_owned());
-   |                             ^^^^^^^^^^^^^^^ help: use: `path`
-
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:94:28
-   |
-LL |     require_impl_deref_str(s.to_owned());
-   |                            ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:95:30
-   |
-LL |     require_impl_deref_slice(slice.to_owned());
-LL |     require_impl_deref_slice(slice.to_owned());
-   |                              ^^^^^^^^^^^^^^^^ help: use: `slice`
-
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:97:29
-   |
-LL |     require_deref_str_slice(s.to_owned(), slice.to_owned());
-   |                             ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:97:43
-   |
-   |
-LL |     require_deref_str_slice(s.to_owned(), slice.to_owned());
-   |                                           ^^^^^^^^^^^^^^^^ help: use: `slice`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:98:29
-   |
-   |
-LL |     require_deref_slice_str(slice.to_owned(), s.to_owned());
-   |                             ^^^^^^^^^^^^^^^^ help: use: `slice`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:98:47
-   |
-   |
-LL |     require_deref_slice_str(slice.to_owned(), s.to_owned());
-   |                                               ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:100:26
-   |
-   |
-LL |     require_as_ref_c_str(c_str.to_owned());
-   |                          ^^^^^^^^^^^^^^^^ help: use: `c_str`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:101:27
-   |
-LL |     require_as_ref_os_str(os_str.to_owned());
---
-
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:109:31
-   |
-LL |     require_impl_as_ref_c_str(c_str.to_owned());
-   |                               ^^^^^^^^^^^^^^^^ help: use: `c_str`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:110:32
-   |
-   |
-LL |     require_impl_as_ref_os_str(os_str.to_owned());
-   |                                ^^^^^^^^^^^^^^^^^ help: use: `os_str`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:111:30
-   |
-   |
-LL |     require_impl_as_ref_path(path.to_owned());
-   |                              ^^^^^^^^^^^^^^^ help: use: `path`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:112:29
-   |
-   |
-LL |     require_impl_as_ref_str(s.to_owned());
-   |                             ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:113:29
-   |
-   |
-LL |     require_impl_as_ref_str(x.to_owned());
-   |                             ^^^^^^^^^^^^ help: use: `&x`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:114:31
-   |
-LL |     require_impl_as_ref_slice(array.to_owned());
---
-
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:116:31
-   |
-LL |     require_impl_as_ref_slice(slice.to_owned());
-   |                               ^^^^^^^^^^^^^^^^ help: use: `slice`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:118:30
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());
-   |                              ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:118:44
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());
-   |                                            ^^^^^^^^^^^^^^^^ help: use: `array`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:119:30
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());
-   |                              ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:119:44
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());
-   |                                            ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:120:30
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());
-   |                              ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:120:44
-   |
-   |
-LL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());
-   |                                            ^^^^^^^^^^^^^^^^ help: use: `slice`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:121:30
-   |
-   |
-LL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());
-   |                              ^^^^^^^^^^^^^^^^ help: use: `array`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:121:48
-   |
-   |
-LL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());
-   |                                                ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:122:30
-   |
-   |
-LL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());
-   |                              ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:122:52
-   |
-   |
-LL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());
-   |                                                    ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:123:30
-   |
-   |
-LL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());
-   |                              ^^^^^^^^^^^^^^^^ help: use: `slice`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:123:48
-   |
-   |
-LL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());
-   |                                                ^^^^^^^^^^^^ help: use: `s`
-error: unnecessary use of `to_string`
-  --> $DIR/unnecessary_to_owned.rs:125:20
-   |
-   |
-LL |     let _ = x.join(&x_ref.to_string());
-   |                    ^^^^^^^^^^^^^^^^^^ help: use: `x_ref`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:127:13
-   |
-   |
-LL |     let _ = slice.to_vec().into_iter();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:128:13
-   |
-   |
-LL |     let _ = slice.to_owned().into_iter();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:129:13
-   |
-   |
-LL |     let _ = [std::path::PathBuf::new()][..].to_vec().into_iter();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:130:13
-   |
-   |
-LL |     let _ = [std::path::PathBuf::new()][..].to_owned().into_iter();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:132:13
-   |
-   |
-LL |     let _ = IntoIterator::into_iter(slice.to_vec());
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:133:13
-   |
-LL |     let _ = IntoIterator::into_iter(slice.to_owned());
-LL |     let _ = IntoIterator::into_iter(slice.to_owned());
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:134:13
-   |
-   |
-LL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_vec());
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
-error: unnecessary use of `to_owned`
-  --> $DIR/unnecessary_to_owned.rs:135:13
-   |
-   |
-LL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_owned());
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:197:14
-   |
-   |
-LL |     for t in file_types.to_vec() {
-   |
-help: use
-   |
-LL |     for t in file_types {
-LL |     for t in file_types {
-   |              ~~~~~~~~~~
-help: remove this `&`
-   |
-LL -         let path = match get_file_path(&t) {
-LL +         let path = match get_file_path(t) {
-
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:220:14
-   |
-   |
-LL |     let _ = &["x"][..].to_vec().into_iter();
-   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `["x"][..].iter().cloned()`
-error: unnecessary use of `to_vec`
-  --> $DIR/unnecessary_to_owned.rs:225:14
-   |
-   |
-LL |     let _ = &["x"][..].to_vec().into_iter();
-   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `["x"][..].iter().copied()`
-error: unnecessary use of `to_string`
-  --> $DIR/unnecessary_to_owned.rs:272:24
-   |
-   |
-LL |         Box::new(build(y.to_string()))
-   |                        ^^^^^^^^^^^^^ help: use: `y`
-error: unnecessary use of `to_string`
-  --> $DIR/unnecessary_to_owned.rs:380:12
-   |
-   |
-LL |         id("abc".to_string())
-   |            ^^^^^^^^^^^^^^^^^ help: use: `"abc"`
-error: aborting due to 79 previous errors
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to 5 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_to_owned.stage-id.stderr
thread '[ui] ui/unnecessary_to_owned.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 6, column: 2)', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/runtest.rs:2397:15


 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:14:9
    |
 LL |     let peekable = std::iter::empty::<u32>().peekable();
    |
    |
    = help: consider removing the call to `peekable`
    = note: `-D clippy::unused-peekable` implied by `-D warnings`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:18:9
    |
 LL |     let new_local = old_local;
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:22:9
    |
 LL |     let by_mut_ref = &mut by_mut_ref_test;
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:29:9
    |
 LL |     let peekable_from_fn = returns_peekable();
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:32:13
    |
 LL |     let mut peekable_using_iterator_method = std::iter::empty::<u32>().peekable();
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:37:9
    |
 LL |     let passed_along_ref = std::iter::empty::<u32>().peekable();
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:42:9
    |
 LL |     let _by_ref = by_ref_test.by_ref();
    |
    |
    = help: consider removing the call to `peekable`
 
 error: `peek` never called on `Peekable` iterator
   --> $DIR/unused_peekable.rs:44:13
    |
 LL |     let mut peekable_in_for_loop = std::iter::empty::<u32>().peekable();
    |
    |
    = help: consider removing the call to `peekable`
 
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
+
+error: internal compiler error: unexpected panic
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
 error: aborting due to 8 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_peekable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused_peekable.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_peekable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_peekable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_peekable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":208,"byte_end":216,"line_start":14,"line_end":14,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"    let peekable = std::iter::empty::<u32>().peekable();","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::unused-peekable` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:14:9\n   |\nLL |     let peekable = std::iter::empty::<u32>().peekable();\n   |         ^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n   = note: `-D clippy::unused-peekable` implied by `-D warnings`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":353,"byte_end":362,"line_start":18,"line_end":18,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"    let new_local = old_local;","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:18:9\n   |\nLL |     let new_local = old_local;\n   |         ^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":475,"byte_end":485,"line_start":22,"line_end":22,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"    let by_mut_ref = &mut by_mut_ref_test;","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:22:9\n   |\nLL |     let by_mut_ref = &mut by_mut_ref_test;\n   |         ^^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":653,"byte_end":669,"line_start":29,"line_end":29,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"    let peekable_from_fn = returns_peekable();","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:29:9\n   |\nLL |     let peekable_from_fn = returns_peekable();\n   |         ^^^^^^^^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":755,"byte_end":785,"line_start":32,"line_end":32,"column_start":13,"column_end":43,"is_primary":true,"text":[{"text":"    let mut peekable_using_iterator_method = std::iter::empty::<u32>().peekable();","highlight_start":13,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:32:13\n   |\nLL |     let mut peekable_using_iterator_method = std::iter::empty::<u32>().peekable();\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":969,"byte_end":985,"line_start":37,"line_end":37,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"    let passed_along_ref = std::iter::empty::<u32>().peekable();","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:37:9\n   |\nLL |     let passed_along_ref = std::iter::empty::<u32>().peekable();\n   |         ^^^^^^^^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":1164,"byte_end":1171,"line_start":42,"line_end":42,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"    let _by_ref = by_ref_test.by_ref();","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:42:9\n   |\nLL |     let _by_ref = by_ref_test.by_ref();\n   |         ^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
{"message":"`peek` never called on `Peekable` iterator","code":{"code":"clippy::unused_peekable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_peekable.rs","byte_start":1209,"byte_end":1229,"line_start":44,"line_end":44,"column_start":13,"column_end":33,"is_primary":true,"text":[{"text":"    let mut peekable_in_for_loop = std::iter::empty::<u32>().peekable();","highlight_start":13,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the call to `peekable`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `peek` never called on `Peekable` iterator\n  --> tests/ui/unused_peekable.rs:44:13\n   |\nLL |     let mut peekable_in_for_loop = std::iter::empty::<u32>().peekable();\n   |             ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider removing the call to `peekable`\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}

------------------------------------------


diff of stderr:

-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:43:18
-   |
-LL |         foo_rstr(rstr.as_ref());
-   |                  ^^^^^^^^^^^^^ help: try this: `rstr`
-note: the lint level is defined here
-  --> $DIR/useless_asref.rs:2:9
-   |
-   |
-LL | #![deny(clippy::useless_asref)]
-   |         ^^^^^^^^^^^^^^^^^^^^^
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
 
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:45:20
-   |
-   |
-LL |         foo_rslice(rslice.as_ref());
-   |                    ^^^^^^^^^^^^^^^ help: try this: `rslice`
 
-error: this call to `as_mut` does nothing
-  --> $DIR/useless_asref.rs:49:21
-   |
-   |
-LL |         foo_mrslice(mrslice.as_mut());
-   |                     ^^^^^^^^^^^^^^^^ help: try this: `mrslice`
 
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:51:20
-   |
-   |
-LL |         foo_rslice(mrslice.as_ref());
-   |                    ^^^^^^^^^^^^^^^^ help: try this: `mrslice`
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:58:20
-   |
-   |
-LL |         foo_rslice(rrrrrslice.as_ref());
-   |                    ^^^^^^^^^^^^^^^^^^^ help: try this: `rrrrrslice`
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:60:18
-   |
-   |
-LL |         foo_rstr(rrrrrstr.as_ref());
-   |                  ^^^^^^^^^^^^^^^^^ help: try this: `rrrrrstr`
-error: this call to `as_mut` does nothing
-  --> $DIR/useless_asref.rs:65:21
-   |
-   |
-LL |         foo_mrslice(mrrrrrslice.as_mut());
-   |                     ^^^^^^^^^^^^^^^^^^^^ help: try this: `mrrrrrslice`
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:67:20
-   |
-   |
-LL |         foo_rslice(mrrrrrslice.as_ref());
-   |                    ^^^^^^^^^^^^^^^^^^^^ help: try this: `mrrrrrslice`
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:71:16
-   |
-   |
-LL |     foo_rrrrmr((&&&&MoreRef).as_ref());
-   |                ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `(&&&&MoreRef)`
-error: this call to `as_mut` does nothing
-  --> $DIR/useless_asref.rs:121:13
-   |
-   |
-LL |     foo_mrt(mrt.as_mut());
-   |             ^^^^^^^^^^^^ help: try this: `mrt`
-error: this call to `as_ref` does nothing
-  --> $DIR/useless_asref.rs:123:12
-   |
-   |
-LL |     foo_rt(mrt.as_ref());
-   |            ^^^^^^^^^^^^ help: try this: `mrt`
-error: aborting due to 11 previous errors
-
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_asref.stage-id.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_asref.stage-id.stderr
thread '[ui] ui/useless_asref.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 1, column: 2)', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/runtest.rs:2397:15

 error: use of `File::read_to_end`
   --> $DIR/verbose_file_reads.rs:23:5
    |
    |
 LL |     f.read_to_end(&mut buffer)?;
    |
    = help: consider using `fs::read` instead
    = help: consider using `fs::read` instead
    = note: `-D clippy::verbose-file-reads` implied by `-D warnings`
-error: use of `File::read_to_string`
-  --> $DIR/verbose_file_reads.rs:26:5
-   |
-   |
-LL |     f.read_to_string(&mut string_buffer)?;
-   |
-   = help: consider using `fs::read_to_string` instead
-   = help: consider using `fs::read_to_string` instead
+thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45
 
-error: aborting due to 2 previous errors
+error: internal compiler error: unexpected panic
+
+
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)
+
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to previous error
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/verbose_file_reads.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args verbose_file_reads.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/verbose_file_reads.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/verbose_file_reads.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/verbose_file_reads.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `File::read_to_end`","code":{"code":"clippy::verbose_file_reads","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/verbose_file_reads.rs","byte_start":532,"byte_end":558,"line_start":23,"line_end":23,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    f.read_to_end(&mut buffer)?;","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `fs::read` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::verbose-file-reads` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of `File::read_to_end`\n  --> tests/ui/verbose_file_reads.rs:23:5\n   |\nLL |     f.read_to_end(&mut buffer)?;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using `fs::read` instead\n   = note: `-D clippy::verbose-file-reads` implied by `-D warnings`\n\n"}
thread 'rustc' panicked at 'internal error: entered unreachable code: inherent projection should have been normalized away above', src/tools/clippy/clippy_lints/src/dereference.rs:1460:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.70 (1de4719f 2023-03-21)

query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}

------------------------------------------


diff of stderr:

 error: useless use of `vec!`
   --> $DIR/vec.rs:28:14
    |
 LL |     on_slice(&vec![]);
    |              ^^^^^^^ help: you can use a slice directly: `&[]`
    |
    = note: `-D clippy::useless-vec` implied by `-D warnings`
 error: useless use of `vec!`
   --> $DIR/vec.rs:30:18
    |
    |
 LL |     on_mut_slice(&mut vec![]);
    |                  ^^^^^^^^^^^ help: you can use a slice directly: `&mut []`
 error: useless use of `vec!`
   --> $DIR/vec.rs:32:14
    |
    |
 LL |     on_slice(&vec![1, 2]);
    |              ^^^^^^^^^^^ help: you can use a slice directly: `&[1, 2]`
 error: useless use of `vec!`
   --> $DIR/vec.rs:34:18
    |
    |
 LL |     on_mut_slice(&mut vec![1, 2]);
    |                  ^^^^^^^^^^^^^^^ help: you can use a slice directly: `&mut [1, 2]`
 error: useless use of `vec!`
   --> $DIR/vec.rs:36:14
    |
    |
