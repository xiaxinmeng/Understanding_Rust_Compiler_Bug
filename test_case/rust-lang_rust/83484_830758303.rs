plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

---- compile_test stdout ----
diff of stderr:

 error: this `if` has identical blocks
-   |
-LL |       if true {
-   |  _____________^
-   |  _____________^
-LL | |         for _ in &[42] {
-LL | |             let foo: &Option<_> = &Some::<u8>(42);
-LL | |             if foo.is_some() {
-LL | |         }
-LL | |     } else {
-   | |_____^
-   |
-   |
-   = note: `-D clippy::if-same-then-else` implied by `-D warnings`
-note: same as this
-   |
-LL |       } else {
-   |  ____________^
-   |  ____________^
-LL | |         //~ ERROR same body as `if` block
-LL | |         for _ in &[42] {
-LL | |             let bar: &Option<_> = &Some::<u8>(42);
-LL | |         }
-LL | |     }
-   | |_____^
-
-
-error: this `if` has identical blocks
    |
 LL |       if true {
    |  _____________^
    |  _____________^
 LL | |         if let Some(a) = Some(42) {}
 LL | |     } else {
    |
    |
+   = note: `-D clippy::if-same-then-else` implied by `-D warnings`
 note: same as this
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         if let Some(a) = Some(42) {}
 LL | |     }
 
 
 error: this `if` has identical blocks
    |
 LL |       if true {
    |  _____________^
    |  _____________^
 LL | |         if let (1, .., 3) = (1, 2, 3) {}
 LL | |     } else {
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:43:12
    |
    |
 LL |       } else {
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         if let (1, .., 3) = (1, 2, 3) {}
 LL | |     }
 
 
 error: this `if` has identical blocks
    |
 LL |       let _ = if true {
    |  _____________________^
 LL | |         f32::NAN
---
   --> $DIR/if_same_then_else2.rs:93:12
    |
 LL |       } else {
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         f32::NAN
 LL | |     };
 
 
 error: this `if` has identical blocks
    |
 LL |       if true {
    |  _____________^
    |  _____________^
 LL | |         Ok("foo")?;
 LL | |     } else {
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:100:12
    |
    |
 LL |       } else {
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         Ok("foo")?;
 LL | |     }
 
 
 error: this `if` has identical blocks
    |
 LL |       } else if true {
    |  ____________________^
 LL | |         let foo = "";
 LL | |         let foo = "";
 LL | |         return Ok(&foo[0..]);
 LL | |     } else {
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:125:12
    |
    |
 LL |       } else {
    |  ____________^
 LL | |         let foo = "";
 LL | |         return Ok(&foo[0..]);
 LL | |     }
 
-error: aborting due to 6 previous errors
+error: aborting due to 5 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/if_same_then_else2.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args if_same_then_else2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/if_same_then_else2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/if_same_then_else2.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/if_same_then_else2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":808,"byte_end":852,"line_start":34,"line_end":36,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    if true {","highlight_start":13,"highlight_end":14},{"text":"        if let Some(a) = Some(42) {}","highlight_start":1,"highlight_end":37},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::if-same-then-else` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":858,"byte_end":944,"line_start":36,"line_end":39,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        if let Some(a) = Some(42) {}","highlight_start":1,"highlight_end":37},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:34:13\n   |\nLL |       if true {\n   |  _____________^\nLL | |         if let Some(a) = Some(42) {}\nLL | |     } else {\n   | |_____^\n   |\n   = note: `-D clippy::if-same-then-else` implied by `-D warnings`\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:36:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         if let Some(a) = Some(42) {}\nLL | |     }\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":958,"byte_end":1006,"line_start":41,"line_end":43,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    if true {","highlight_start":13,"highlight_end":14},{"text":"        if let (1, .., 3) = (1, 2, 3) {}","highlight_start":1,"highlight_end":41},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1012,"byte_end":1102,"line_start":43,"line_end":46,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        if let (1, .., 3) = (1, 2, 3) {}","highlight_start":1,"highlight_end":41},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:41:13\n   |\nLL |       if true {\n   |  _____________^\nLL | |         if let (1, .., 3) = (1, 2, 3) {}\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:43:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         if let (1, .., 3) = (1, 2, 3) {}\nLL | |     }\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1924,"byte_end":1948,"line_start":91,"line_end":93,"column_start":21,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = if true {","highlight_start":21,"highlight_end":22},{"text":"        f32::NAN","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1954,"byte_end":2020,"line_start":93,"line_end":96,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        f32::NAN","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:91:21\n   |\nLL |       let _ = if true {\n   |  _____________________^\nLL | |         f32::NAN\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:93:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         f32::NAN\nLL | |     };\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2035,"byte_end":2062,"line_start":98,"line_end":100,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    if true {","highlight_start":13,"highlight_end":14},{"text":"        Ok(\"foo\")?;","highlight_start":1,"highlight_end":20},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2068,"byte_end":2137,"line_start":100,"line_end":103,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        Ok(\"foo\")?;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:98:13\n   |\nLL |       if true {\n   |  _____________^\nLL | |         Ok(\"foo\")?;\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:100:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         Ok(\"foo\")?;\nLL | |     }\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2516,"byte_end":2575,"line_start":122,"line_end":125,"column_start":20,"column_end":6,"is_primary":true,"text":[{"text":"    } else if true {","highlight_start":20,"highlight_end":21},{"text":"        let foo = \"\";","highlight_start":1,"highlight_end":22},{"text":"        return Ok(&foo[0..]);","highlight_start":1,"highlight_end":30},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2581,"byte_end":2640,"line_start":125,"line_end":128,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        let foo = \"\";","highlight_start":1,"highlight_end":22},{"text":"        return Ok(&foo[0..]);","highlight_start":1,"highlight_end":30},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:122:20\n   |\nLL |       } else if true {\n   |  ____________________^\nLL | |         let foo = \"\";\nLL | |         return Ok(&foo[0..]);\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:125:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         let foo = \"\";\nLL | |         return Ok(&foo[0..]);\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: transmute from a pointer type (`*const T`) to a reference type (`&T`)
    |
    |
 LL |     let _: &T = std::mem::transmute(p);
    |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*p`
    |
    = note: `-D clippy::transmute-ptr-to-ref` implied by `-D warnings`
 
 error: transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)
    |
    |
 LL |     let _: &mut T = std::mem::transmute(m);
    |                     ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *m`
 
 error: transmute from a pointer type (`*mut T`) to a reference type (`&T`)
    |
    |
 LL |     let _: &T = std::mem::transmute(m);
    |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*m`
 
 error: transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)
    |
    |
 LL |     let _: &mut T = std::mem::transmute(p as *mut T);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *(p as *mut T)`
 
 error: transmute from a pointer type (`*const U`) to a reference type (`&T`)
    |
    |
 LL |     let _: &T = std::mem::transmute(o);
    |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(o as *const T)`
 
 error: transmute from a pointer type (`*mut U`) to a reference type (`&mut T`)
    |
    |
 LL |     let _: &mut T = std::mem::transmute(om);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *(om as *mut T)`
 
 error: transmute from a pointer type (`*mut U`) to a reference type (`&T`)
    |
    |
 LL |     let _: &T = std::mem::transmute(om);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(om as *const T)`
 
 error: transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<u8>`)
    |
    |
 LL |     let _: &Foo<u8> = unsafe { std::mem::transmute::<_, &Foo<_>>(raw) };
-   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const Foo<_>)`
+   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const issue1231::Foo<u8>)`
 
 error: transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<&u8>`)
    |
    |
 LL |     let _: &Foo<&u8> = unsafe { std::mem::transmute::<_, &Foo<&_>>(raw) };
-   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const Foo<&_>)`
+   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const issue1231::Foo<&u8>)`
 
 error: transmute from a pointer type (`*const i32`) to a reference type (`&u8`)
    |
    |
 LL |     unsafe { std::mem::transmute::<_, Bar>(raw) };
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const u8)`
 error: aborting due to 10 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_ptr_to_ref.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmute_ptr_to_ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/transmute_ptr_to_ref.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_ptr_to_ref.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_ptr_to_ref.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"transmute from a pointer type (`*const T`) to a reference type (`&T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":135,"byte_end":157,"line_start":4,"line_end":4,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(p);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-ptr-to-ref` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":135,"byte_end":157,"line_start":4,"line_end":4,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(p);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":"&*p","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*const T`) to a reference type (`&T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:4:17\n   |\nLL |     let _: &T = std::mem::transmute(p);\n   |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*p`\n   |\n   = note: `-D clippy::transmute-ptr-to-ref` implied by `-D warnings`\n\n"}
{"message":"transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":201,"byte_end":223,"line_start":7,"line_end":7,"column_start":21,"column_end":43,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(m);","highlight_start":21,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":201,"byte_end":223,"line_start":7,"line_end":7,"column_start":21,"column_end":43,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(m);","highlight_start":21,"highlight_end":43}],"label":null,"suggested_replacement":"&mut *m","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:7:21\n   |\nLL |     let _: &mut T = std::mem::transmute(m);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *m`\n\n"}
{"message":"transmute from a pointer type (`*mut T`) to a reference type (`&T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":271,"byte_end":293,"line_start":10,"line_end":10,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(m);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":271,"byte_end":293,"line_start":10,"line_end":10,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(m);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":"&*m","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*mut T`) to a reference type (`&T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:10:17\n   |\nLL |     let _: &T = std::mem::transmute(m);\n   |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*m`\n\n"}
{"message":"transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":337,"byte_end":369,"line_start":13,"line_end":13,"column_start":21,"column_end":53,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(p as *mut T);","highlight_start":21,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":337,"byte_end":369,"line_start":13,"line_end":13,"column_start":21,"column_end":53,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(p as *mut T);","highlight_start":21,"highlight_end":53}],"label":null,"suggested_replacement":"&mut *(p as *mut T)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*mut T`) to a reference type (`&mut T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:13:21\n   |\nLL |     let _: &mut T = std::mem::transmute(p as *mut T);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *(p as *mut T)`\n\n"}
{"message":"transmute from a pointer type (`*const U`) to a reference type (`&T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":421,"byte_end":443,"line_start":16,"line_end":16,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(o);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":421,"byte_end":443,"line_start":16,"line_end":16,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(o);","highlight_start":17,"highlight_end":39}],"label":null,"suggested_replacement":"&*(o as *const T)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*const U`) to a reference type (`&T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:16:17\n   |\nLL |     let _: &T = std::mem::transmute(o);\n   |                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(o as *const T)`\n\n"}
{"message":"transmute from a pointer type (`*mut U`) to a reference type (`&mut T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":501,"byte_end":524,"line_start":19,"line_end":19,"column_start":21,"column_end":44,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(om);","highlight_start":21,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":501,"byte_end":524,"line_start":19,"line_end":19,"column_start":21,"column_end":44,"is_primary":true,"text":[{"text":"    let _: &mut T = std::mem::transmute(om);","highlight_start":21,"highlight_end":44}],"label":null,"suggested_replacement":"&mut *(om as *mut T)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*mut U`) to a reference type (`&mut T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:19:21\n   |\nLL |     let _: &mut T = std::mem::transmute(om);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&mut *(om as *mut T)`\n\n"}
{"message":"transmute from a pointer type (`*mut U`) to a reference type (`&T`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":585,"byte_end":608,"line_start":22,"line_end":22,"column_start":17,"column_end":40,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(om);","highlight_start":17,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":585,"byte_end":608,"line_start":22,"line_end":22,"column_start":17,"column_end":40,"is_primary":true,"text":[{"text":"    let _: &T = std::mem::transmute(om);","highlight_start":17,"highlight_end":40}],"label":null,"suggested_replacement":"&*(om as *const T)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*mut U`) to a reference type (`&T`)\n  --> tests/ui/transmute_ptr_to_ref.rs:22:17\n   |\nLL |     let _: &T = std::mem::transmute(om);\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(om as *const T)`\n\n"}
{"message":"transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<u8>`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":780,"byte_end":818,"line_start":32,"line_end":32,"column_start":32,"column_end":70,"is_primary":true,"text":[{"text":"    let _: &Foo<u8> = unsafe { std::mem::transmute::<_, &Foo<_>>(raw) };","highlight_start":32,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":780,"byte_end":818,"line_start":32,"line_end":32,"column_start":32,"column_end":70,"is_primary":true,"text":[{"text":"    let _: &Foo<u8> = unsafe { std::mem::transmute::<_, &Foo<_>>(raw) };","highlight_start":32,"highlight_end":70}],"label":null,"suggested_replacement":"&*(raw as *const issue1231::Foo<u8>)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<u8>`)\n  --> tests/ui/transmute_ptr_to_ref.rs:32:32\n   |\nLL |     let _: &Foo<u8> = unsafe { std::mem::transmute::<_, &Foo<_>>(raw) };\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const issue1231::Foo<u8>)`\n\n"}
{"message":"transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<&u8>`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":855,"byte_end":894,"line_start":34,"line_end":34,"column_start":33,"column_end":72,"is_primary":true,"text":[{"text":"    let _: &Foo<&u8> = unsafe { std::mem::transmute::<_, &Foo<&_>>(raw) };","highlight_start":33,"highlight_end":72}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":855,"byte_end":894,"line_start":34,"line_end":34,"column_start":33,"column_end":72,"is_primary":true,"text":[{"text":"    let _: &Foo<&u8> = unsafe { std::mem::transmute::<_, &Foo<&_>>(raw) };","highlight_start":33,"highlight_end":72}],"label":null,"suggested_replacement":"&*(raw as *const issue1231::Foo<&u8>)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*const i32`) to a reference type (`&issue1231::Foo<&u8>`)\n  --> tests/ui/transmute_ptr_to_ref.rs:34:33\n   |\nLL |     let _: &Foo<&u8> = unsafe { std::mem::transmute::<_, &Foo<&_>>(raw) };\n   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const issue1231::Foo<&u8>)`\n\n"}
{"message":"transmute from a pointer type (`*const i32`) to a reference type (`&u8`)","code":{"code":"clippy::transmute_ptr_to_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":971,"byte_end":1005,"line_start":38,"line_end":38,"column_start":14,"column_end":48,"is_primary":true,"text":[{"text":"    unsafe { std::mem::transmute::<_, Bar>(raw) };","highlight_start":14,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_ptr_to_ref.rs","byte_start":971,"byte_end":1005,"line_start":38,"line_end":38,"column_start":14,"column_end":48,"is_primary":true,"text":[{"text":"    unsafe { std::mem::transmute::<_, Bar>(raw) };","highlight_start":14,"highlight_end":48}],"label":null,"suggested_replacement":"&*(raw as *const u8)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer type (`*const i32`) to a reference type (`&u8`)\n  --> tests/ui/transmute_ptr_to_ref.rs:38:14\n   |\nLL |     unsafe { std::mem::transmute::<_, Bar>(raw) };\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `&*(raw as *const u8)`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
