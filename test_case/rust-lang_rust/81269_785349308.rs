plain
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
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
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
error: aborting due to 11 previous errors




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
-  --> $DIR/unnecessary_clone.rs:32:5
-   |
-LL |     arc_weak.clone();
-   |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`
-
-error: using `.clone()` on a ref-counted pointer
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
-error: aborting due to 12 previous errors
+error: aborting due to 11 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/unnecessary_clone.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base' 'unnecessary_clone.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unnecessary_clone.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/unnecessary_clone.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-26a3592219becd1a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/unnecessary_clone.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":472,"byte_end":482,"line_start":23,"line_end":23,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    rc.clone();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::clone-on-ref-ptr` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":472,"byte_end":482,"line_start":23,"line_end":23,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    rc.clone();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"Rc::<bool>::clone(&rc)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:23:5\n   |\nLL |     rc.clone();\n   |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`\n   |\n   = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":509,"byte_end":520,"line_start":26,"line_end":26,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    arc.clone();","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":509,"byte_end":520,"line_start":26,"line_end":26,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    arc.clone();","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"Arc::<bool>::clone(&arc)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:26:5\n   |\nLL |     arc.clone();\n   |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":549,"byte_end":563,"line_start":29,"line_end":29,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    rcweak.clone();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":549,"byte_end":563,"line_start":29,"line_end":29,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    rcweak.clone();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"Weak::<bool>::clone(&rcweak)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:29:5\n   |\nLL |     rcweak.clone();\n   |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":717,"byte_end":726,"line_start":36,"line_end":36,"column_start":33,"column_end":42,"is_primary":true,"text":[{"text":"    let _: Arc<dyn SomeTrait> = x.clone();","highlight_start":33,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":717,"byte_end":726,"line_start":36,"line_end":36,"column_start":33,"column_end":42,"is_primary":true,"text":[{"text":"    let _: Arc<dyn SomeTrait> = x.clone();","highlight_start":33,"highlight_end":42}],"label":null,"suggested_replacement":"Arc::<SomeImpl>::clone(&x)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:36:33\n   |\nLL |     let _: Arc<dyn SomeTrait> = x.clone();\n   |                                 ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`\n\n"}
{"message":"using `clone` on type `T` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":777,"byte_end":786,"line_start":40,"line_end":40,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    t.clone();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::clone-on-copy` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try removing the `clone` call","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":777,"byte_end":786,"line_start":40,"line_end":40,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    t.clone();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"t","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `T` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:40:5\n   |\nLL |     t.clone();\n   |     ^^^^^^^^^ help: try removing the `clone` call: `t`\n   |\n   = note: `-D clippy::clone-on-copy` implied by `-D warnings`\n\n"}
{"message":"using `clone` on type `std::option::Option<T>` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":793,"byte_end":808,"line_start":42,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Some(t).clone();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try removing the `clone` call","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":793,"byte_end":808,"line_start":42,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Some(t).clone();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"Some(t)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `std::option::Option<T>` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:42:5\n   |\nLL |     Some(t).clone();\n   |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::clone_double_ref)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":"&(*y).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":899,"byte_end":908,"line_start":48,"line_end":48,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    let z: &Vec<_> = y.clone();","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":"<&std::vec::Vec<i32>>::clone(y)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&std::vec::Vec<i32>` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:48:22\n   |\nLL |     let z: &Vec<_> = y.clone();\n   |                      ^^^^^^^^^\n   |\n   = note: `#[deny(clippy::clone_double_ref)]` on by default\nhelp: try dereferencing it\n   |\nLL |     let z: &Vec<_> = &(*y).clone();\n   |                      ^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |     let z: &Vec<_> = <&std::vec::Vec<i32>>::clone(y);\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `clone` on type `many_derefs::E` which implements the `Copy` trait","code":{"code":"clippy::clone_on_copy","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1604,"byte_end":1613,"line_start":84,"line_end":84,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"        let _: E = a.clone();","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1604,"byte_end":1613,"line_start":84,"line_end":84,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"        let _: E = a.clone();","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":"*****a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on type `many_derefs::E` which implements the `Copy` trait\n  --> tests/ui/unnecessary_clone.rs:84:20\n   |\nLL |         let _: E = a.clone();\n   |                    ^^^^^^^^^ help: try dereferencing it: `*****a`\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":"&(*encoded).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1705,"byte_end":1720,"line_start":89,"line_end":89,"column_start":22,"column_end":37,"is_primary":true,"text":[{"text":"        let _ = &mut encoded.clone();","highlight_start":22,"highlight_end":37}],"label":null,"suggested_replacement":"<&[u8]>::clone(encoded)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:89:22\n   |\nLL |         let _ = &mut encoded.clone();\n   |                      ^^^^^^^^^^^^^^^\n   |\nhelp: try dereferencing it\n   |\nLL |         let _ = &mut &(*encoded).clone();\n   |                      ^^^^^^^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |         let _ = &mut <&[u8]>::clone(encoded);\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type","code":{"code":"clippy::clone_double_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try dereferencing it","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":"&(*encoded).clone()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or try being explicit if you are sure, that you want to clone a reference","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":1739,"byte_end":1754,"line_start":90,"line_end":90,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"        let _ = &encoded.clone();","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":"<&[u8]>::clone(encoded)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `clone` on a double-reference; this will copy the reference of type `&[u8]` instead of cloning the inner type\n  --> tests/ui/unnecessary_clone.rs:90:18\n   |\nLL |         let _ = &encoded.clone();\n   |                  ^^^^^^^^^^^^^^^\n   |\nhelp: try dereferencing it\n   |\nLL |         let _ = &&(*encoded).clone();\n   |                  ^^^^^^^^^^^^^^^^^^^\nhelp: or try being explicit if you are sure, that you want to clone a reference\n   |\nLL |         let _ = &<&[u8]>::clone(encoded);\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"using `.clone()` on a ref-counted pointer","code":{"code":"clippy::clone_on_ref_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":2067,"byte_end":2093,"line_start":108,"line_end":108,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"        Some(try_opt!(Some(rc)).clone())","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":2067,"byte_end":2093,"line_start":108,"line_end":108,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"        Some(try_opt!(Some(rc)).clone())","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":"Rc::<u8>::clone(&try_opt!(Some(rc)))","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `.clone()` on a ref-counted pointer\n  --> tests/ui/unnecessary_clone.rs:108:14\n   |\nLL |         Some(try_opt!(Some(rc)).clone())\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `Rc::<u8>::clone(&try_opt!(Some(rc)))`\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:12:13
   |
LL |     let _ = a == b;
   |
   |
   = note: `-D clippy::vtable-address-comparisons` implied by `-D warnings`
   = help: consider extracting and comparing data pointers only
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:13:13
   |
   |
LL |     let _ = a != b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:14:13
   |
LL |     let _ = a < b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:15:13
   |
LL |     let _ = a <= b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:16:13
   |
LL |     let _ = a > b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:17:13
   |
LL |     let _ = a >= b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
---

error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:25:5
   |
LL |     Rc::ptr_eq(&a, &a);
   |
   = help: consider extracting and comparing data pointers only

error: aborting due to 9 previous errors
error: aborting due to 9 previous errors



expected stderr:
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:12:13
   |
LL |     let _ = a == b;
   |
   |
   = note: `-D clippy::vtable-address-comparisons` implied by `-D warnings`
   = help: consider extracting and comparing data pointers only
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:13:13
   |
   |
LL |     let _ = a != b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:14:13
   |
LL |     let _ = a < b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:15:13
   |
LL |     let _ = a <= b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:16:13
   |
LL |     let _ = a > b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:17:13
   |
LL |     let _ = a >= b;
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
---

error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:25:5
   |
LL |     Rc::ptr_eq(&a, &a);
   |
   = help: consider extracting and comparing data pointers only

error: comparing trait object pointers compares a non-unique vtable address
error: comparing trait object pointers compares a non-unique vtable address
  --> $DIR/vtable_address_comparisons.rs:28:5
   |
LL |     Arc::ptr_eq(&a, &a);
   |
   = help: consider extracting and comparing data pointers only

error: aborting due to 10 previous errors
---

 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:12:13
    |
 LL |     let _ = a == b;
    |
    |
    = note: `-D clippy::vtable-address-comparisons` implied by `-D warnings`
    = help: consider extracting and comparing data pointers only
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:13:13
    |
    |
 LL |     let _ = a != b;
    |
    = help: consider extracting and comparing data pointers only
 
 error: comparing trait object pointers compares a non-unique vtable address
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:14:13
    |
 LL |     let _ = a < b;
    |
    = help: consider extracting and comparing data pointers only
 
 error: comparing trait object pointers compares a non-unique vtable address
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:15:13
    |
 LL |     let _ = a <= b;
    |
    = help: consider extracting and comparing data pointers only
 
 error: comparing trait object pointers compares a non-unique vtable address
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:16:13
    |
 LL |     let _ = a > b;
    |
    = help: consider extracting and comparing data pointers only
 
 error: comparing trait object pointers compares a non-unique vtable address
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:17:13
    |
 LL |     let _ = a >= b;
    |
    = help: consider extracting and comparing data pointers only
 
 error: comparing trait object pointers compares a non-unique vtable address
---
 
 error: comparing trait object pointers compares a non-unique vtable address
   --> $DIR/vtable_address_comparisons.rs:25:5
    |
 LL |     Rc::ptr_eq(&a, &a);
    |
    = help: consider extracting and comparing data pointers only
 
-error: comparing trait object pointers compares a non-unique vtable address
-error: comparing trait object pointers compares a non-unique vtable address
-  --> $DIR/vtable_address_comparisons.rs:28:5
-   |
-LL |     Arc::ptr_eq(&a, &a);
-   |     ^^^^^^^^^^^^^^^^^^^
-   |
-   = help: consider extracting and comparing data pointers only
-error: aborting due to 10 previous errors
+error: aborting due to 9 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/vtable_address_comparisons.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base' 'vtable_address_comparisons.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/vtable_address_comparisons.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/vtable_address_comparisons.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-26a3592219becd1a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/vtable_address_comparisons.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":264,"byte_end":270,"line_start":12,"line_end":12,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = a == b;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::vtable-address-comparisons` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:12:13\n   |\nLL |     let _ = a == b;\n   |             ^^^^^^\n   |\n   = note: `-D clippy::vtable-address-comparisons` implied by `-D warnings`\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":284,"byte_end":290,"line_start":13,"line_end":13,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = a != b;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:13:13\n   |\nLL |     let _ = a != b;\n   |             ^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":304,"byte_end":309,"line_start":14,"line_end":14,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = a < b;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:14:13\n   |\nLL |     let _ = a < b;\n   |             ^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":323,"byte_end":329,"line_start":15,"line_end":15,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = a <= b;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:15:13\n   |\nLL |     let _ = a <= b;\n   |             ^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":343,"byte_end":348,"line_start":16,"line_end":16,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = a > b;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:16:13\n   |\nLL |     let _ = a > b;\n   |             ^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":362,"byte_end":368,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = a >= b;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:17:13\n   |\nLL |     let _ = a >= b;\n   |             ^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":374,"byte_end":387,"line_start":18,"line_end":18,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    ptr::eq(a, b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:18:5\n   |\nLL |     ptr::eq(a, b);\n   |     ^^^^^^^^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":454,"byte_end":467,"line_start":22,"line_end":22,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    ptr::eq(a, b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:22:5\n   |\nLL |     ptr::eq(a, b);\n   |     ^^^^^^^^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}
{"message":"comparing trait object pointers compares a non-unique vtable address","code":{"code":"clippy::vtable_address_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/vtable_address_comparisons.rs","byte_start":513,"byte_end":531,"line_start":25,"line_end":25,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    Rc::ptr_eq(&a, &a);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting and comparing data pointers only","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: comparing trait object pointers compares a non-unique vtable address\n  --> tests/ui/vtable_address_comparisons.rs:25:5\n   |\nLL |     Rc::ptr_eq(&a, &a);\n   |     ^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider extracting and comparing data pointers only\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
