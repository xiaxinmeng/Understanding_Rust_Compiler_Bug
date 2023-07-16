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
failures:

---- compile_test stdout ----
normalized stderr:
error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         for _ in &[42] {
LL | |             let foo: &Option<_> = &Some::<u8>(42);
LL | |         }
LL | |     }
   | |_____^
   |
   |
   = note: `-D clippy::if-same-then-else` implied by `-D warnings`
note: same as this
   |
LL |       if true {
   |  _____________^
   |  _____________^
LL | |         for _ in &[42] {
LL | |             let foo: &Option<_> = &Some::<u8>(42);
LL | |             if true {
LL | |         }
LL | |     } else {
   | |_____^


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         f32::NAN
LL | |     };
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:90:21
   |
   |
LL |       let _ = if true {
LL | |         f32::NAN
LL | |     } else {
   | |_____^


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         Ok("foo")?;
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:97:13
   |
   |
LL |       if true {
   |  _____________^
LL | |         Ok("foo")?;
LL | |     } else {
   | |_____^


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
LL | |         let foo = "";
LL | |         let foo = "";
LL | |         return Ok(&foo[0..]);
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:121:20
   |
   |
LL |       } else if true {
   |  ____________________^
LL | |         let foo = "";
LL | |         return Ok(&foo[0..]);
LL | |     } else {

error: aborting due to 4 previous errors




expected stderr:
error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         for _ in &[42] {
LL | |             let foo: &Option<_> = &Some::<u8>(42);
LL | |         }
LL | |     }
   | |_____^
   |
   |
   = note: `-D clippy::if-same-then-else` implied by `-D warnings`
note: same as this
   |
LL |       if true {
   |  _____________^
   |  _____________^
LL | |         for _ in &[42] {
LL | |             let foo: &Option<_> = &Some::<u8>(42);
LL | |             if true {
LL | |         }
LL | |     } else {
   | |_____^


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         if let Some(a) = Some(42) {}
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:33:13
   |
   |
LL |       if true {
   |  _____________^
LL | |         if let Some(a) = Some(42) {}
LL | |     } else {


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         if let (1, .., 3) = (1, 2, 3) {}
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:40:13
   |
   |
LL |       if true {
   |  _____________^
LL | |         if let (1, .., 3) = (1, 2, 3) {}
LL | |     } else {


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         f32::NAN
LL | |     };
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:90:21
   |
   |
LL |       let _ = if true {
LL | |         f32::NAN
LL | |     } else {
   | |_____^


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
   |  ____________^
LL | |         //~ ERROR same body as `if` block
LL | |         Ok("foo")?;
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:97:13
   |
   |
LL |       if true {
   |  _____________^
LL | |         Ok("foo")?;
LL | |     } else {


error: this `if` has identical blocks
   |
LL |       } else {
   |  ____________^
LL | |         let foo = "";
LL | |         let foo = "";
LL | |         return Ok(&foo[0..]);
LL | |     }
   |
note: same as this
  --> $DIR/if_same_then_else2.rs:121:20
   |
   |
LL |       } else if true {
   |  ____________________^
LL | |         let foo = "";
LL | |         return Ok(&foo[0..]);
LL | |     } else {

error: aborting due to 6 previous errors




diff of stderr:

 error: this `if` has identical blocks
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         for _ in &[42] {
 LL | |             let foo: &Option<_> = &Some::<u8>(42);
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
    = note: `-D clippy::if-same-then-else` implied by `-D warnings`
 note: same as this
    |
 LL |       if true {
    |  _____________^
    |  _____________^
 LL | |         for _ in &[42] {
 LL | |             let foo: &Option<_> = &Some::<u8>(42);
 LL | |             if true {
 LL | |         }
 LL | |     } else {
    | |_____^
 
 
 error: this `if` has identical blocks
-   |
-LL |       } else {
-   |  ____________^
-   |  ____________^
-LL | |         //~ ERROR same body as `if` block
-LL | |         if let Some(a) = Some(42) {}
-LL | |     }
-   | |_____^
-   |
-note: same as this
-   |
-LL |       if true {
-   |  _____________^
-   |  _____________^
-LL | |         if let Some(a) = Some(42) {}
-LL | |     } else {
-   | |_____^
-
-error: this `if` has identical blocks
-   |
-LL |       } else {
-   |  ____________^
-   |  ____________^
-LL | |         //~ ERROR same body as `if` block
-LL | |         if let (1, .., 3) = (1, 2, 3) {}
-LL | |     }
-   | |_____^
-   |
-note: same as this
-   |
-LL |       if true {
-   |  _____________^
-   |  _____________^
-LL | |         if let (1, .., 3) = (1, 2, 3) {}
-LL | |     } else {
-   | |_____^
-
-error: this `if` has identical blocks
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         f32::NAN
 LL | |     };
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:90:21
    |
    |
 LL |       let _ = if true {
 LL | |         f32::NAN
 LL | |     } else {
    | |_____^
 
 
 error: this `if` has identical blocks
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         //~ ERROR same body as `if` block
 LL | |         Ok("foo")?;
 LL | |     }
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:97:13
    |
    |
 LL |       if true {
    |  _____________^
 LL | |         Ok("foo")?;
 LL | |     } else {
 
 
 error: this `if` has identical blocks
    |
 LL |       } else {
    |  ____________^
 LL | |         let foo = "";
 LL | |         let foo = "";
 LL | |         return Ok(&foo[0..]);
 LL | |     }
    |
 note: same as this
   --> $DIR/if_same_then_else2.rs:121:20
    |
    |
 LL |       } else if true {
    |  ____________________^
 LL | |         let foo = "";
 LL | |         return Ok(&foo[0..]);
 LL | |     } else {
 
-error: aborting due to 6 previous errors
+error: aborting due to 4 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/if_same_then_else2.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base' 'if_same_then_else2.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/if_same_then_else2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/if_same_then_else2.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-c43e08f62c64f186.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-1bcf52e6f16bdb4f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/if_same_then_else2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":500,"byte_end":741,"line_start":21,"line_end":31,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        for _ in &[42] {","highlight_start":1,"highlight_end":25},{"text":"            let foo: &Option<_> = &Some::<u8>(42);","highlight_start":1,"highlight_end":51},{"text":"            if true {","highlight_start":1,"highlight_end":22},{"text":"                break;","highlight_start":1,"highlight_end":23},{"text":"            } else {","highlight_start":1,"highlight_end":21},{"text":"                continue;","highlight_start":1,"highlight_end":26},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::if-same-then-else` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":295,"byte_end":494,"line_start":12,"line_end":21,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    if true {","highlight_start":13,"highlight_end":14},{"text":"        for _ in &[42] {","highlight_start":1,"highlight_end":25},{"text":"            let foo: &Option<_> = &Some::<u8>(42);","highlight_start":1,"highlight_end":51},{"text":"            if true {","highlight_start":1,"highlight_end":22},{"text":"                break;","highlight_start":1,"highlight_end":23},{"text":"            } else {","highlight_start":1,"highlight_end":21},{"text":"                continue;","highlight_start":1,"highlight_end":26},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:21:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         for _ in &[42] {\nLL | |             let foo: &Option<_> = &Some::<u8>(42);\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::if-same-then-else` implied by `-D warnings`\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:12:13\n   |\nLL |       if true {\n   |  _____________^\nLL | |         for _ in &[42] {\nLL | |             let foo: &Option<_> = &Some::<u8>(42);\nLL | |             if true {\n...  |\nLL | |         }\nLL | |     } else {\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1901,"byte_end":1967,"line_start":92,"line_end":95,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        f32::NAN","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1871,"byte_end":1895,"line_start":90,"line_end":92,"column_start":21,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = if true {","highlight_start":21,"highlight_end":22},{"text":"        f32::NAN","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:92:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         f32::NAN\nLL | |     };\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:90:21\n   |\nLL |       let _ = if true {\n   |  _____________________^\nLL | |         f32::NAN\nLL | |     } else {\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2015,"byte_end":2084,"line_start":99,"line_end":102,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        //~ ERROR same body as `if` block","highlight_start":1,"highlight_end":42},{"text":"        Ok(\"foo\")?;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":1982,"byte_end":2009,"line_start":97,"line_end":99,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    if true {","highlight_start":13,"highlight_end":14},{"text":"        Ok(\"foo\")?;","highlight_start":1,"highlight_end":20},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:99:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         //~ ERROR same body as `if` block\nLL | |         Ok(\"foo\")?;\nLL | |     }\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:97:13\n   |\nLL |       if true {\n   |  _____________^\nLL | |         Ok(\"foo\")?;\nLL | |     } else {\n   | |_____^\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2528,"byte_end":2587,"line_start":124,"line_end":127,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        let foo = \"\";","highlight_start":1,"highlight_end":22},{"text":"        return Ok(&foo[0..]);","highlight_start":1,"highlight_end":30},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/if_same_then_else2.rs","byte_start":2463,"byte_end":2522,"line_start":121,"line_end":124,"column_start":20,"column_end":6,"is_primary":true,"text":[{"text":"    } else if true {","highlight_start":20,"highlight_end":21},{"text":"        let foo = \"\";","highlight_start":1,"highlight_end":22},{"text":"        return Ok(&foo[0..]);","highlight_start":1,"highlight_end":30},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/if_same_then_else2.rs:124:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         let foo = \"\";\nLL | |         return Ok(&foo[0..]);\nLL | |     }\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/if_same_then_else2.rs:121:20\n   |\nLL |       } else if true {\n   |  ____________________^\nLL | |         let foo = \"\";\nLL | |         return Ok(&foo[0..]);\nLL | |     } else {\n   | |_____^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
