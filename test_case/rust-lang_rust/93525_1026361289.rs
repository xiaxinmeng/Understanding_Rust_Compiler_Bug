plain
-hello dup fd
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestdhvjXy/fs.stage-id.stdout

-hello dup fd
+error: unsupported operation: can't call foreign function: readdir64
+   --> /checkout/library/std/src/sys/unix/fs.rs:482:33
---
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestdhvjXy/fs.stage-id.stderr
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestdhvjXy" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdhvjXy/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestdhvjXy/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

---- compile_test stdout ----
diff of stderr:

+error: cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized
+   |
+   |
+LL | unsafe impl<P> Send for Complex<P, u32> {}
+   |
+   |
+   = note: `-D suspicious-auto-trait-impls` implied by `-D warnings`
+   = warning: this will change its meaning in a future release!
+note: try using the same sequence of generic parameters as the struct definition
+  --> $DIR/non_send_fields_in_send_ty.rs:122:1
+   |
+   |
+LL | / pub struct Complex<A, B> {
+LL | |     field1: A,
+LL | |     field2: B,
+LL | | }
+   | |_^
+   = note: `u32` is not a generic parameter
+
+error: cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized
+   |
+   |
+LL | unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}
+   |
+   = warning: this will change its meaning in a future release!
+   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>
+note: try using the same sequence of generic parameters as the struct definition
+note: try using the same sequence of generic parameters as the struct definition
+  --> $DIR/non_send_fields_in_send_ty.rs:122:1
+   |
+LL | / pub struct Complex<A, B> {
+LL | |     field1: A,
+LL | |     field2: B,
+LL | | }
+   | |_^
+   = note: `std::sync::MutexGuard<'static, bool>` is not a generic parameter
+
 error: some fields in `RingBuffer<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T> Send for RingBuffer<T> {}
    |
    |
    = note: `-D clippy::non-send-fields-in-send-ty` implied by `-D warnings`
 note: it is not safe to send field `data` to another thread
    |
    |
 LL |     data: Vec<UnsafeCell<T>>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    = help: add bounds on type parameter `T` that satisfy `Vec<UnsafeCell<T>>: Send`
 
 error: some fields in `MvccRwLock<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T> Send for MvccRwLock<T> {}
    |
    |
 note: it is not safe to send field `lock` to another thread
    |
    |
 LL |     lock: Mutex<Box<T>>,
    |     ^^^^^^^^^^^^^^^^^^^
    = help: add bounds on type parameter `T` that satisfy `Mutex<Box<T>>: Send`
 
 error: some fields in `ArcGuard<RC, T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<RC, T: Send> Send for ArcGuard<RC, T> {}
    |
    |
 note: it is not safe to send field `head` to another thread
    |
    |
 LL |     head: Arc<RC>,
    |     ^^^^^^^^^^^^^
    = help: add bounds on type parameter `RC` that satisfy `Arc<RC>: Send`
 
 error: some fields in `DeviceHandle<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T: UsbContext> Send for DeviceHandle<T> {}
    |
    |
 note: it is not safe to send field `context` to another thread
    |
 LL |     context: T,
    |     ^^^^^^^^^^
    |     ^^^^^^^^^^
    = help: add `T: Send` bound in `Send` impl
 
 error: some fields in `NoGeneric` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl Send for NoGeneric {}
    |
    |
 note: it is not safe to send field `rc_is_not_send` to another thread
    |
    |
 LL |     rc_is_not_send: Rc<String>,
    = help: use a thread-safe type that implements `Send`
 
 
 error: some fields in `MultiField<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T> Send for MultiField<T> {}
    |
    |
 note: it is not safe to send field `field1` to another thread
    |
 LL |     field1: T,
    |     ^^^^^^^^^
    |     ^^^^^^^^^
    = help: add `T: Send` bound in `Send` impl
 note: it is not safe to send field `field2` to another thread
    |
 LL |     field2: T,
    |     ^^^^^^^^^
    |     ^^^^^^^^^
    = help: add `T: Send` bound in `Send` impl
 note: it is not safe to send field `field3` to another thread
    |
 LL |     field3: T,
    |     ^^^^^^^^^
    |     ^^^^^^^^^
    = help: add `T: Send` bound in `Send` impl
 
 error: some fields in `MyOption<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T> Send for MyOption<T> {}
    |
 note: it is not safe to send field `0` to another thread
   --> $DIR/non_send_fields_in_send_ty.rs:66:12
    |
    |
 LL |     MySome(T),
    |            ^
    = help: add `T: Send` bound in `Send` impl
 
 error: some fields in `MultiParam<A, B>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<A, B> Send for MultiParam<A, B> {}
    |
    |
 note: it is not safe to send field `vec` to another thread
    |
    |
 LL |     vec: Vec<(A, B)>,
    |     ^^^^^^^^^^^^^^^^
    = help: add bounds on type parameters `A, B` that satisfy `Vec<(A, B)>: Send`
 
 error: some fields in `HeuristicTest` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl Send for HeuristicTest {}
    |
    |
 note: it is not safe to send field `field4` to another thread
    |
    |
 LL |     field4: (*const NonSend, Rc<u8>),
    = help: use a thread-safe type that implements `Send`
 
 
 error: some fields in `AttrTest3<T>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<T> Send for AttrTest3<T> {}
    |
 note: it is not safe to send field `0` to another thread
   --> $DIR/non_send_fields_in_send_ty.rs:114:11
    |
    |
 LL |     Enum2(T),
    |           ^
    = help: add `T: Send` bound in `Send` impl
 
 error: some fields in `Complex<P, u32>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<P> Send for Complex<P, u32> {}
    |
    |
 note: it is not safe to send field `field1` to another thread
    |
 LL |     field1: A,
    |     ^^^^^^^^^
    |     ^^^^^^^^^
    = help: add `P: Send` bound in `Send` impl
 
 error: some fields in `Complex<Q, MutexGuard<'static, bool>>` are not safe to be sent to another thread
    |
    |
 LL | unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}
    |
    |
 note: it is not safe to send field `field2` to another thread
    |
 LL |     field2: B,
    |     ^^^^^^^^^
    = help: use a thread-safe type that implements `Send`
---
To only update this specific test, also pass `--test-args non_send_fields_in_send_ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/non_send_fields_in_send_ty.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/non_send_fields_in_send_ty.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b682a5a8a9c64b20.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/non_send_fields_in_send_ty.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized","code":{"code":"suspicious_auto_trait_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2588,"byte_end":2630,"line_start":127,"line_end":127,"column_start":1,"column_end":43,"is_primary":true,"text":[{"text":"unsafe impl<P> Send for Complex<P, u32> {}","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D suspicious-auto-trait-impls` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this will change its meaning in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using the same sequence of generic parameters as the struct definition","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2528,"byte_end":2586,"line_start":122,"line_end":125,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub struct Complex<A, B> {","highlight_start":1,"highlight_end":27},{"text":"    field1: A,","highlight_start":1,"highlight_end":15},{"text":"    field2: B,","highlight_start":1,"highlight_end":15},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`u32` is not a generic parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized\n  --> tests/ui/non_send_fields_in_send_ty.rs:127:1\n   |\nLL | unsafe impl<P> Send for Complex<P, u32> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D suspicious-auto-trait-impls` implied by `-D warnings`\n   = warning: this will change its meaning in a future release!\n   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>\nnote: try using the same sequence of generic parameters as the struct definition\n  --> tests/ui/non_send_fields_in_send_ty.rs:122:1\n   |\nLL | / pub struct Complex<A, B> {\nLL | |     field1: A,\nLL | |     field2: B,\nLL | | }\n   | |_^\n   = note: `u32` is not a generic parameter\n\n"}
{"message":"cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized","code":{"code":"suspicious_auto_trait_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2660,"byte_end":2730,"line_start":130,"line_end":130,"column_start":1,"column_end":71,"is_primary":true,"text":[{"text":"unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change its meaning in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using the same sequence of generic parameters as the struct definition","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2528,"byte_end":2586,"line_start":122,"line_end":125,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub struct Complex<A, B> {","highlight_start":1,"highlight_end":27},{"text":"    field1: A,","highlight_start":1,"highlight_end":15},{"text":"    field2: B,","highlight_start":1,"highlight_end":15},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::sync::MutexGuard<'static, bool>` is not a generic parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cross-crate traits with a default impl, like `std::marker::Send`, should not be specialized\n  --> tests/ui/non_send_fields_in_send_ty.rs:130:1\n   |\nLL | unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this will change its meaning in a future release!\n   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>\nnote: try using the same sequence of generic parameters as the struct definition\n  --> tests/ui/non_send_fields_in_send_ty.rs:122:1\n   |\nLL | / pub struct Complex<A, B> {\nLL | |     field1: A,\nLL | |     field2: B,\nLL | | }\n   | |_^\n   = note: `std::sync::MutexGuard<'static, bool>` is not a generic parameter\n\n"}
{"message":"some fields in `RingBuffer<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":312,"byte_end":352,"line_start":16,"line_end":16,"column_start":1,"column_end":41,"is_primary":true,"text":[{"text":"unsafe impl<T> Send for RingBuffer<T> {}","highlight_start":1,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::non-send-fields-in-send-ty` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"it is not safe to send field `data` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":245,"byte_end":269,"line_start":11,"line_end":11,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    data: Vec<UnsafeCell<T>>,","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add bounds on type parameter `T` that satisfy `Vec<UnsafeCell<T>>: Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `RingBuffer<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:16:1\n   |\nLL | unsafe impl<T> Send for RingBuffer<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::non-send-fields-in-send-ty` implied by `-D warnings`\nnote: it is not safe to send field `data` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:11:5\n   |\nLL |     data: Vec<UnsafeCell<T>>,\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: add bounds on type parameter `T` that satisfy `Vec<UnsafeCell<T>>: Send`\n\n"}
{"message":"some fields in `MvccRwLock<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":464,"byte_end":504,"line_start":24,"line_end":24,"column_start":1,"column_end":41,"is_primary":true,"text":[{"text":"unsafe impl<T> Send for MvccRwLock<T> {}","highlight_start":1,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `lock` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":440,"byte_end":459,"line_start":21,"line_end":21,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    lock: Mutex<Box<T>>,","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add bounds on type parameter `T` that satisfy `Mutex<Box<T>>: Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `MvccRwLock<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:24:1\n   |\nLL | unsafe impl<T> Send for MvccRwLock<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `lock` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:21:5\n   |\nLL |     lock: Mutex<Box<T>>,\n   |     ^^^^^^^^^^^^^^^^^^^\n   = help: add bounds on type parameter `T` that satisfy `Mutex<Box<T>>: Send`\n\n"}
{"message":"some fields in `ArcGuard<RC, T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":605,"byte_end":657,"line_start":32,"line_end":32,"column_start":1,"column_end":53,"is_primary":true,"text":[{"text":"unsafe impl<RC, T: Send> Send for ArcGuard<RC, T> {}","highlight_start":1,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `head` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":587,"byte_end":600,"line_start":29,"line_end":29,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    head: Arc<RC>,","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add bounds on type parameter `RC` that satisfy `Arc<RC>: Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `ArcGuard<RC, T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:32:1\n   |\nLL | unsafe impl<RC, T: Send> Send for ArcGuard<RC, T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `head` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:29:5\n   |\nLL |     head: Arc<RC>,\n   |     ^^^^^^^^^^^^^\n   = help: add bounds on type parameter `RC` that satisfy `Arc<RC>: Send`\n\n"}
{"message":"some fields in `DeviceHandle<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":917,"byte_end":971,"line_start":48,"line_end":48,"column_start":1,"column_end":55,"is_primary":true,"text":[{"text":"unsafe impl<T: UsbContext> Send for DeviceHandle<T> {}","highlight_start":1,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `context` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":859,"byte_end":869,"line_start":44,"line_end":44,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    context: T,","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `DeviceHandle<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:48:1\n   |\nLL | unsafe impl<T: UsbContext> Send for DeviceHandle<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `context` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:44:5\n   |\nLL |     context: T,\n   |     ^^^^^^^^^^\n   = help: add `T: Send` bound in `Send` impl\n\n"}
{"message":"some fields in `NoGeneric` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1052,"byte_end":1085,"line_start":55,"line_end":55,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"unsafe impl Send for NoGeneric {}","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `rc_is_not_send` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1021,"byte_end":1047,"line_start":52,"line_end":52,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    rc_is_not_send: Rc<String>,","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use a thread-safe type that implements `Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `NoGeneric` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:55:1\n   |\nLL | unsafe impl Send for NoGeneric {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `rc_is_not_send` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:52:5\n   |\nLL |     rc_is_not_send: Rc<String>,\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: use a thread-safe type that implements `Send`\n\n"}
{"message":"some fields in `MultiField<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1162,"byte_end":1202,"line_start":63,"line_end":63,"column_start":1,"column_end":41,"is_primary":true,"text":[{"text":"unsafe impl<T> Send for MultiField<T> {}","highlight_start":1,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `field1` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1118,"byte_end":1127,"line_start":58,"line_end":58,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    field1: T,","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"it is not safe to send field `field2` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1133,"byte_end":1142,"line_start":59,"line_end":59,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    field2: T,","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"it is not safe to send field `field3` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1148,"byte_end":1157,"line_start":60,"line_end":60,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    field3: T,","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `MultiField<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:63:1\n   |\nLL | unsafe impl<T> Send for MultiField<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `field1` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:58:5\n   |\nLL |     field1: T,\n   |     ^^^^^^^^^\n   = help: add `T: Send` bound in `Send` impl\nnote: it is not safe to send field `field2` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:59:5\n   |\nLL |     field2: T,\n   |     ^^^^^^^^^\n   = help: add `T: Send` bound in `Send` impl\nnote: it is not safe to send field `field3` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:60:5\n   |\nLL |     field3: T,\n   |     ^^^^^^^^^\n   = help: add `T: Send` bound in `Send` impl\n\n"}
{"message":"some fields in `MyOption<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1257,"byte_end":1295,"line_start":70,"line_end":70,"column_start":1,"column_end":39,"is_primary":true,"text":[{"text":"unsafe impl<T> Send for MyOption<T> {}","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `0` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1238,"byte_end":1239,"line_start":66,"line_end":66,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    MySome(T),","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `MyOption<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:70:1\n   |\nLL | unsafe impl<T> Send for MyOption<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `0` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:66:12\n   |\nLL |     MySome(T),\n   |            ^\n   = help: add `T: Send` bound in `Send` impl\n\n"}
{"message":"some fields in `MultiParam<A, B>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1542,"byte_end":1588,"line_start":82,"line_end":82,"column_start":1,"column_end":47,"is_primary":true,"text":[{"text":"unsafe impl<A, B> Send for MultiParam<A, B> {}","highlight_start":1,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `vec` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1521,"byte_end":1537,"line_start":79,"line_end":79,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    vec: Vec<(A, B)>,","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add bounds on type parameters `A, B` that satisfy `Vec<(A, B)>: Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `MultiParam<A, B>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:82:1\n   |\nLL | unsafe impl<A, B> Send for MultiParam<A, B> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `vec` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:79:5\n   |\nLL |     vec: Vec<(A, B)>,\n   |     ^^^^^^^^^^^^^^^^\n   = help: add bounds on type parameters `A, B` that satisfy `Vec<(A, B)>: Send`\n\n"}
{"message":"some fields in `HeuristicTest` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2026,"byte_end":2063,"line_start":100,"line_end":100,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"unsafe impl Send for HeuristicTest {}","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `field4` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":1909,"byte_end":1941,"line_start":95,"line_end":95,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    field4: (*const NonSend, Rc<u8>),","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use a thread-safe type that implements `Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `HeuristicTest` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:100:1\n   |\nLL | unsafe impl Send for HeuristicTest {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `field4` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:95:5\n   |\nLL |     field4: (*const NonSend, Rc<u8>),\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: use a thread-safe type that implements `Send`\n\n"}
{"message":"some fields in `AttrTest3<T>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2434,"byte_end":2473,"line_start":119,"line_end":119,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"unsafe impl<T> Send for AttrTest3<T> {}","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `0` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2347,"byte_end":2348,"line_start":114,"line_end":114,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    Enum2(T),","highlight_start":11,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `T: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `AttrTest3<T>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:119:1\n   |\nLL | unsafe impl<T> Send for AttrTest3<T> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `0` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:114:11\n   |\nLL |     Enum2(T),\n   |           ^\n   = help: add `T: Send` bound in `Send` impl\n\n"}
{"message":"some fields in `Complex<P, u32>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2588,"byte_end":2630,"line_start":127,"line_end":127,"column_start":1,"column_end":43,"is_primary":true,"text":[{"text":"unsafe impl<P> Send for Complex<P, u32> {}","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `field1` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2559,"byte_end":2568,"line_start":123,"line_end":123,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    field1: A,","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"add `P: Send` bound in `Send` impl","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `Complex<P, u32>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:127:1\n   |\nLL | unsafe impl<P> Send for Complex<P, u32> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `field1` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:123:5\n   |\nLL |     field1: A,\n   |     ^^^^^^^^^\n   = help: add `P: Send` bound in `Send` impl\n\n"}
{"message":"some fields in `Complex<Q, MutexGuard<'static, bool>>` are not safe to be sent to another thread","code":{"code":"clippy::non_send_fields_in_send_ty","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2660,"byte_end":2730,"line_start":130,"line_end":130,"column_start":1,"column_end":71,"is_primary":true,"text":[{"text":"unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"it is not safe to send field `field2` to another thread","code":null,"level":"note","spans":[{"file_name":"tests/ui/non_send_fields_in_send_ty.rs","byte_start":2574,"byte_end":2583,"line_start":124,"line_end":124,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    field2: B,","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use a thread-safe type that implements `Send`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: some fields in `Complex<Q, MutexGuard<'static, bool>>` are not safe to be sent to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:130:1\n   |\nLL | unsafe impl<Q: Send> Send for Complex<Q, MutexGuard<'static, bool>> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: it is not safe to send field `field2` to another thread\n  --> tests/ui/non_send_fields_in_send_ty.rs:124:5\n   |\nLL |     field2: B,\n   |     ^^^^^^^^^\n   = help: use a thread-safe type that implements `Send`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
