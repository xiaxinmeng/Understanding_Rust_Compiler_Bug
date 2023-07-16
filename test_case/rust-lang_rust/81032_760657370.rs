plain
Cloning into 'rust-toolstate'...
{"embedded-book":"test-pass","miri":"build-fail","rls":"test-pass","rustbook":"test-fail","reference":"test-pass","rustfmt":"test-pass","cargo-miri":"test-fail","rust-by-example":"test-pass","edition-guide":"test-pass","nomicon":"test-pass","book":"test-pass"}[master ecc0bc2] (linux CI update)
 1 file changed, 1 insertion(+)
To https://github.com/rust-lang-nursery/rust-toolstate
   2dde7d7..ecc0bc2  master -> master
Build completed successfully in 0:00:04
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
---
failures:

---- compile_test stdout ----
normalized stderr:
error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:23:5
   |
LL |     rc.clone();
   |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`
   |
   = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:26:5
LL |     arc.clone();
LL |     arc.clone();
   |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:29:5
   |
LL |     rcweak.clone();
   |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:32:5
LL |     arc_weak.clone();
LL |     arc_weak.clone();
   |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:36:33
   |
LL |     let _: Arc<dyn SomeTrait> = x.clone();
   |                                 ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`

error: using `clone` on type `T` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:40:5
   |
LL |     t.clone();
   |     ^^^^^^^^^ help: try removing the `clone` call: `t`
   |
   = note: `-D clippy::clone-on-copy` implied by `-D warnings`

error: using `clone` on type `std::option::Option<T>` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:42:5
   |
LL |     Some(t).clone();
   |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`

error: using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:48:22
   |
LL |     let z: &Vec<_> = y.clone();
   |
   |
   = note: `#[deny(clippy::clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |     let z: &Vec<_> = &(*y).clone();
   |                      ^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |     let z: &Vec<_> = <&std::vec::Vec<i32>>::clone(y);


error: using `clone` on type `many_derefs::E` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:84:20
   |
LL |         let _: E = a.clone();
   |                    ^^^^^^^^^ help: try dereferencing it: `*****a`

error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:89:22
   |
LL |         let _ = &mut encoded.clone();
   |
help: try dereferencing it
   |
   |
LL |         let _ = &mut &(*encoded).clone();
   |                      ^^^^^^^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |         let _ = &mut <&[u8]>::clone(encoded);


error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:90:18
   |
LL |         let _ = &encoded.clone();
   |
help: try dereferencing it
   |
   |
LL |         let _ = &&(*encoded).clone();
   |                  ^^^^^^^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |         let _ = &<&[u8]>::clone(encoded);


error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:108:14
   |
LL |         Some(try_opt!(Some(rc)).clone())
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `Rc::<u8>::clone(&try_opt!(Some(rc)))`

error: call to `.clone()` on a reference in this situation does nothing
  --> $DIR/unnecessary_clone.rs:89:29
   |
LL |         let _ = &mut encoded.clone();
   |                             ^^^^^^^^ unnecessary method call
   |
   = note: `-D noop-method-call` implied by `-D warnings`
   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed

error: call to `.clone()` on a reference in this situation does nothing
  --> $DIR/unnecessary_clone.rs:90:25
   |
LL |         let _ = &encoded.clone();
   |                         ^^^^^^^^ unnecessary method call
   |
   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
error: aborting due to 14 previous errors




expected stderr:
error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:23:5
   |
LL |     rc.clone();
   |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`
   |
   = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:26:5
LL |     arc.clone();
LL |     arc.clone();
   |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:29:5
   |
LL |     rcweak.clone();
   |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:32:5
LL |     arc_weak.clone();
LL |     arc_weak.clone();
   |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`

error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:36:33
   |
LL |     let _: Arc<dyn SomeTrait> = x.clone();
   |                                 ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`

error: using `clone` on type `T` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:40:5
   |
LL |     t.clone();
   |     ^^^^^^^^^ help: try removing the `clone` call: `t`
   |
   = note: `-D clippy::clone-on-copy` implied by `-D warnings`

error: using `clone` on type `std::option::Option<T>` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:42:5
   |
LL |     Some(t).clone();
   |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`

error: using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:48:22
   |
LL |     let z: &Vec<_> = y.clone();
   |
   |
   = note: `#[deny(clippy::clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |     let z: &Vec<_> = &(*y).clone();
   |                      ^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |     let z: &Vec<_> = <&std::vec::Vec<i32>>::clone(y);


error: using `clone` on type `many_derefs::E` which implements the `Copy` trait
  --> $DIR/unnecessary_clone.rs:84:20
   |
LL |         let _: E = a.clone();
   |                    ^^^^^^^^^ help: try dereferencing it: `*****a`

error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:89:22
   |
LL |         let _ = &mut encoded.clone();
   |
help: try dereferencing it
   |
   |
LL |         let _ = &mut &(*encoded).clone();
   |                      ^^^^^^^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |         let _ = &mut <&[u8]>::clone(encoded);


error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
  --> $DIR/unnecessary_clone.rs:90:18
   |
LL |         let _ = &encoded.clone();
   |
help: try dereferencing it
   |
   |
LL |         let _ = &&(*encoded).clone();
   |                  ^^^^^^^^^^^^^^^^^^^
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |         let _ = &<&[u8]>::clone(encoded);


error: using `.clone()` on a ref-counted pointer
  --> $DIR/unnecessary_clone.rs:108:14
   |
LL |         Some(try_opt!(Some(rc)).clone())
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `Rc::<u8>::clone(&try_opt!(Some(rc)))`
error: aborting due to 12 previous errors




diff of stderr:

 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:23:5
    |
 LL |     rc.clone();
    |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`
    |
    = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`
 
 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:26:5
 LL |     arc.clone();
 LL |     arc.clone();
    |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`
 
 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:29:5
    |
 LL |     rcweak.clone();
    |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`
 
 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:32:5
 LL |     arc_weak.clone();
 LL |     arc_weak.clone();
    |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`
 
 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:36:33
    |
 LL |     let _: Arc<dyn SomeTrait> = x.clone();
    |                                 ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`
 
 error: using `clone` on type `T` which implements the `Copy` trait
   --> $DIR/unnecessary_clone.rs:40:5
    |
 LL |     t.clone();
    |     ^^^^^^^^^ help: try removing the `clone` call: `t`
    |
    = note: `-D clippy::clone-on-copy` implied by `-D warnings`
 
 error: using `clone` on type `std::option::Option<T>` which implements the `Copy` trait
   --> $DIR/unnecessary_clone.rs:42:5
    |
 LL |     Some(t).clone();
    |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`
 
 error: using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type
   --> $DIR/unnecessary_clone.rs:48:22
    |
 LL |     let z: &Vec<_> = y.clone();
    |
    |
    = note: `#[deny(clippy::clone_double_ref)]` on by default
 help: try dereferencing it
    |
 LL |     let z: &Vec<_> = &(*y).clone();
    |                      ^^^^^^^^^^^^^
 help: or try being explicit if you are sure, that you want to clone a reference
    |
 LL |     let z: &Vec<_> = <&std::vec::Vec<i32>>::clone(y);
 
 
 error: using `clone` on type `many_derefs::E` which implements the `Copy` trait
   --> $DIR/unnecessary_clone.rs:84:20
    |
 LL |         let _: E = a.clone();
    |                    ^^^^^^^^^ help: try dereferencing it: `*****a`
 
 error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
   --> $DIR/unnecessary_clone.rs:89:22
    |
 LL |         let _ = &mut encoded.clone();
    |
 help: try dereferencing it
    |
    |
 LL |         let _ = &mut &(*encoded).clone();
    |                      ^^^^^^^^^^^^^^^^^^^
 help: or try being explicit if you are sure, that you want to clone a reference
    |
 LL |         let _ = &mut <&[u8]>::clone(encoded);
 
 
 error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type
   --> $DIR/unnecessary_clone.rs:90:18
    |
 LL |         let _ = &encoded.clone();
    |
 help: try dereferencing it
    |
    |
 LL |         let _ = &&(*encoded).clone();
    |                  ^^^^^^^^^^^^^^^^^^^
 help: or try being explicit if you are sure, that you want to clone a reference
    |
 LL |         let _ = &<&[u8]>::clone(encoded);
error: test failed, to rerun pass '--test compile-test'
 
 
 error: using `.clone()` on a ref-counted pointer
   --> $DIR/unnecessary_clone.rs:108:14
    |
 LL |         Some(try_opt!(Some(rc)).clone())
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `Rc::<u8>::clone(&try_opt!(Some(rc)))`
-error: aborting due to 12 previous errors
-error: aborting due to 12 previous errors
+error: call to `.clone()` on a reference in this situation does nothing
+  --> $DIR/unnecessary_clone.rs:89:29
+   |
+LL |         let _ = &mut encoded.clone();
+   |                             ^^^^^^^^ unnecessary method call
+   |
+   = note: `-D noop-method-call` implied by `-D warnings`
+   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
+
+error: call to `.clone()` on a reference in this situation does nothing
+  --> $DIR/unnecessary_clone.rs:90:25
+   |
+LL |         let _ = &encoded.clone();
+   |                         ^^^^^^^^ unnecessary method call
+   |
+   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
+error: aborting due to 14 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/unnecessary_clone.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base' 'unnecessary_clone.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unnecessary_clone.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/unnecessary_clone.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-bd8c817a160fb1e1.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e3c044d770c3edb5.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/unnecessary_clone.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":472,"byte_end":482,"line_start":23,"line_end":23,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    rc.clone();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::clone-on-ref-ptr` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":472,"byte_end":482,"line_start":23,"line_end":23,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    rc.clone();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"Rc::<bool>::clone(&rc)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:23:5\n   |\nLL |     rc.clone();\n   |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`\n   |\n   = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":509,"byte_end":520,"line_start":26,"line_end":26,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    arc.clone();","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":509,"byte_end":520,"line_start":26,"line_end":26,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    arc.clone();","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"Arc::<bool>::clone(&arc)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:26:5\n   |\nLL |     arc.clone();\n   |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":549,"byte_end":563,"line_start":29,"line_end":29,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    rcweak.clone();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":549,"byte_end":563,"line_start":29,"line_end":29,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    rcweak.clone();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"Weak::<bool>::clone(&rcweak)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:29:5\n   |\nLL |     rcweak.clone();\n   |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":600,"byte_end":616,"line_start":32,"line_end":32,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    arc_weak.clone();","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":600,"byte_end":616,"line_start":32,"line_end":32,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    arc_weak.clone();","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":"Weak::<bool>::clone(&arc_weak)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:32:5\n   |\nLL |     arc_weak.clone();\n   |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":717,"byte_end":726,"line_start":36,"line_end":36,"column_start":33,"column_end":42,"is_primary":true,"text":[{"text":"    let _: Arc<dyn SomeTrait> = x.clone();","highlight_start":33,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":717,"byte_end":726,"line_start":36,"line_end":36,"column_start":33,"column_end":42,"is_primary":true,"text":[{"text":"    let _: Arc<dyn SomeTrait> = x.clone();","highlight_start":33,"highlight_end":42}],"label":null,"suggested_replacement":"Arc::<SomeImpl>::clone(&x)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:36:33\n   |\nLL |     let _: Arc<dyn SomeTrait> = x.clone();\n   |                                 ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`\n\n"}
{"message":"using `clone` on type `T` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":777,"byte_end":786,"line_start":40,"line_end":40,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    t.clone();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::clone-on-copy` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try removing the `clone` call","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":777,"byte_end":786,"line_start":40,"line_end":40,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    t.clone();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"t","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `T` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:40:5\n   |\nLL |     t.clone();\n   |     ^^^^^^^^^ help: try removing the `clone` call: `t`\n   |\n   = note: `-D clippy::clone-on-copy` implied by `-D warnings`\n\n"}
{"message":"using `clone` on type `std::option::Option<T>` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":793,"byte_end":808,"line_start":42,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Some(t).clone();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try removing the `clone` call","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":793,"byte_end":808,"line_start":42,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Some(t).clone();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"Some(t)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `std::option::Option<T>` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:42:5\n   |\nLL |     Some(t).clone();\n   |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::clone_double_ref)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":"&(*y).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":"<&std::vec::Vec<i32>>::clone(y)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:48:22\n   |\nLL |     let z: &Vec<_> = y.clone();\n   |                      ^^^^^^^^^\n   |\n   = note: `#[deny(clippy::clone_double_ref)]` on by default\nhelp: try dereferencing it\n   |\nLL |     let z: &Vec<_> = &(*y).clone();\n   |                      ^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |     let z: &Vec<_> = <&std::vec::Vec<i32>>::clone(y);\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `clone` on type `many_derefs::E` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1604,"byte_end":1613,"line_start":84,"line_end":84,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"        let _: E = a.clone();","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1604,"byte_end":1613,"line_start":84,"line_end":84,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"        let _: E = a.clone();","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":"*****a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `many_derefs::E` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:84:20\n   |\nLL |         let _: E = a.clone();\n   |                    ^^^^^^^^^ help: try dereferencing it: `*****a`\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":"&(*encoded).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":"<&[u8]>::clone(encoded)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:89:22\n   |\nLL |         let _ = &mut encoded.clone();\n   |                      ^^^^^^^^^^^^^^^\n   |\nhelp: try dereferencing it\n   |\nLL |         let _ = &mut &(*encoded).clone();\n   |                      ^^^^^^^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |         let _ = &mut <&[u8]>::clone(encoded);\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":"&(*encoded).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":"<&[u8]>::clone(encoded)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:90:18\n   |\nLL |         let _ = &encoded.clone();\n   |                  ^^^^^^^^^^^^^^^\n   |\nhelp: try dereferencing it\n   |\nLL |         let _ = &&(*encoded).clone();\n   |                  ^^^^^^^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |         let _ = &<&[u8]>::clone(encoded);\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":2067,"byte_end":2093,"line_start":108,"line_end":108,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"        Some(try_opt!(Some(rc)).clone())","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":2067,"byte_end":2093,"line_start":108,"line_end":108,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"        Some(try_opt!(Some(rc)).clone())","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":"Rc::<u8>::clone(&try_opt!(Some(rc)))","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:108:14\n   |\nLL |         Some(try_opt!(Some(rc)).clone())\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `Rc::<u8>::clone(&try_opt!(Some(rc)))`\n\n"}
{"message":"call to `.clone()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1712,"byte_end":1720,"line_start":89,"line_end":89,"column_start":29,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":29,"highlight_end":37}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D noop-method-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.clone()` on a reference in this situation does nothing\n  --> tests/ui/unnecessary_clone.rs:89:29\n   |\nLL |         let _ = &mut encoded.clone();\n   |                             ^^^^^^^^ unnecessary method call\n   |\n   = note: `-D noop-method-call` implied by `-D warnings`\n   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed\n\n"}
{"message":"call to `.clone()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1746,"byte_end":1754,"line_start":90,"line_end":90,"column_start":25,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":25,"highlight_end":33}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.clone()` on a reference in this situation does nothing\n  --> tests/ui/unnecessary_clone.rs:90:25\n   |\nLL |         let _ = &encoded.clone();\n   |                         ^^^^^^^^ unnecessary method call\n   |\n   = note: the type `&[u8]` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
