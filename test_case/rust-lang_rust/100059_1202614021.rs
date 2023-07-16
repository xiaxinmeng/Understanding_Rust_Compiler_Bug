plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 792bc5a0102d0973d42183a2b267850bb905236f and 6b2f0623bf6fef200349da61312e983c9ae0c0b1
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: useless use of `format!`
   --> $DIR/format.rs:19:5
    |
 LL |     format!("foo");
    |     ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
    |
    = note: `-D clippy::useless-format` implied by `-D warnings`
 error: useless use of `format!`
   --> $DIR/format.rs:20:5
    |
    |
 LL |     format!("{{}}");
    |     ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{}".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:21:5
    |
    |
 LL |     format!("{{}} abc {{}}");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{} abc {}".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:22:5
    |
 LL | /     format!(
 LL | /     format!(
 LL | |         r##"foo {{}}
 LL | | " bar"##
 LL | |     );
    |
    |
 help: consider using `.to_string()`
    |
 LL ~     r##"foo {}
 LL ~ " bar"##.to_string();
 
 error: useless use of `format!`
   --> $DIR/format.rs:27:13
    |
    |
 LL |     let _ = format!("");
    |             ^^^^^^^^^^^ help: consider using `String::new()`: `String::new()`
 error: useless use of `format!`
   --> $DIR/format.rs:29:5
    |
 LL |     format!("{}", "foo");
 LL |     format!("{}", "foo");
    |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:33:5
    |
    |
 LL |     format!("{:+}", "foo"); // Warn when the format makes no difference.
    |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:34:5
    |
    |
 LL |     format!("{:<}", "foo"); // Warn when the format makes no difference.
    |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:39:5
    |
 LL |     format!("{}", arg);
 LL |     format!("{}", arg);
    |     ^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:43:5
    |
    |
 LL |     format!("{:+}", arg); // Warn when the format makes no difference.
    |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:44:5
    |
    |
 LL |     format!("{:<}", arg); // Warn when the format makes no difference.
    |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:71:5
    |
 LL |     format!("{}", 42.to_string());
 LL |     format!("{}", 42.to_string());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `42.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:73:5
    |
    |
 LL |     format!("{}", x.display().to_string());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.display().to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:77:18
    |
    |
 LL |     let _ = Some(format!("{}", a + "bar"));
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `a + "bar"`
 error: useless use of `format!`
   --> $DIR/format.rs:81:22
    |
    |
 LL |     let _s: String = format!("{}", &*v.join("/n"));
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `(&*v.join("/n")).to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:87:13
    |
    |
 LL |     let _ = format!("{x}");
    |             ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:89:13
    |
    |
 LL |     let _ = format!("{y}", y = x);
    |             ^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`
 error: useless use of `format!`
-  --> $DIR/format.rs:92:13
+  --> $DIR/format.rs:93:13
    |
    |
 LL |     let _ = format!("{abc}");
    |             ^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `abc.to_string()`
 error: useless use of `format!`
-  --> $DIR/format.rs:94:13
+  --> $DIR/format.rs:95:13
    |
    |
 LL |     let _ = format!("{xx}");
    |             ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `xx.to_string()`
 error: aborting due to 19 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args format.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ead25036d552bfe8.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":322,"byte_end":336,"line_start":19,"line_end":19,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-format` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":322,"byte_end":336,"line_start":19,"line_end":19,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:19:5\n   |\nLL |     format!(\"foo\");\n   |     ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"foo\".to_string()`\n   |\n   = note: `-D clippy::useless-format` implied by `-D warnings`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":342,"byte_end":357,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"{{}}\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":342,"byte_end":357,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"{{}}\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"\"{}\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:20:5\n   |\nLL |     format!(\"{{}}\");\n   |     ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"{}\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":363,"byte_end":387,"line_start":21,"line_end":21,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    format!(\"{{}} abc {{}}\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":363,"byte_end":387,"line_start":21,"line_end":21,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    format!(\"{{}} abc {{}}\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":"\"{} abc {}\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:21:5\n   |\nLL |     format!(\"{{}} abc {{}}\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"{} abc {}\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":393,"byte_end":437,"line_start":22,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    format!(","highlight_start":5,"highlight_end":13},{"text":"        r##\"foo {{}}","highlight_start":1,"highlight_end":21},{"text":"\" bar\"##","highlight_start":1,"highlight_end":9},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":393,"byte_end":437,"line_start":22,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    format!(","highlight_start":5,"highlight_end":13},{"text":"        r##\"foo {{}}","highlight_start":1,"highlight_end":21},{"text":"\" bar\"##","highlight_start":1,"highlight_end":9},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"r##\"foo {}\n\" bar\"##.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:22:5\n   |\nLL | /     format!(\nLL | |         r##\"foo {{}}\nLL | | \" bar\"##\nLL | |     );\n   | |_____^\n   |\nhelp: consider using `.to_string()`\n   |\nLL ~     r##\"foo {}\nLL ~ \" bar\"##.to_string();\n   |\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":452,"byte_end":463,"line_start":27,"line_end":27,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = format!(\"\");","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `String::new()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":452,"byte_end":463,"line_start":27,"line_end":27,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = format!(\"\");","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":"String::new()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:27:13\n   |\nLL |     let _ = format!(\"\");\n   |             ^^^^^^^^^^^ help: consider using `String::new()`: `String::new()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":470,"byte_end":490,"line_start":29,"line_end":29,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{}\", \"foo\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":470,"byte_end":490,"line_start":29,"line_end":29,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{}\", \"foo\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:29:5\n   |\nLL |     format!(\"{}\", \"foo\");\n   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"foo\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":625,"byte_end":647,"line_start":33,"line_end":33,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", \"foo\"); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":625,"byte_end":647,"line_start":33,"line_end":33,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", \"foo\"); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:33:5\n   |\nLL |     format!(\"{:+}\", \"foo\"); // Warn when the format makes no difference.\n   |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"foo\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":698,"byte_end":720,"line_start":34,"line_end":34,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", \"foo\"); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":698,"byte_end":720,"line_start":34,"line_end":34,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", \"foo\"); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:34:5\n   |\nLL |     format!(\"{:<}\", \"foo\"); // Warn when the format makes no difference.\n   |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"foo\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":869,"byte_end":887,"line_start":39,"line_end":39,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    format!(\"{}\", arg);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":869,"byte_end":887,"line_start":39,"line_end":39,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    format!(\"{}\", arg);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:39:5\n   |\nLL |     format!(\"{}\", arg);\n   |     ^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1014,"byte_end":1034,"line_start":43,"line_end":43,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", arg); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1014,"byte_end":1034,"line_start":43,"line_end":43,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", arg); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:43:5\n   |\nLL |     format!(\"{:+}\", arg); // Warn when the format makes no difference.\n   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1085,"byte_end":1105,"line_start":44,"line_end":44,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", arg); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1085,"byte_end":1105,"line_start":44,"line_end":44,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", arg); // Warn when the format makes no difference.","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:44:5\n   |\nLL |     format!(\"{:<}\", arg); // Warn when the format makes no difference.\n   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1957,"byte_end":1986,"line_start":71,"line_end":71,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    format!(\"{}\", 42.to_string());","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1957,"byte_end":1986,"line_start":71,"line_end":71,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    format!(\"{}\", 42.to_string());","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":"42.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:71:5\n   |\nLL |     format!(\"{}\", 42.to_string());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `42.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2046,"byte_end":2084,"line_start":73,"line_end":73,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    format!(\"{}\", x.display().to_string());","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2046,"byte_end":2084,"line_start":73,"line_end":73,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    format!(\"{}\", x.display().to_string());","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":"x.display().to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:73:5\n   |\nLL |     format!(\"{}\", x.display().to_string());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.display().to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2157,"byte_end":2181,"line_start":77,"line_end":77,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = Some(format!(\"{}\", a + \"bar\"));","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2157,"byte_end":2181,"line_start":77,"line_end":77,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = Some(format!(\"{}\", a + \"bar\"));","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":"a + \"bar\"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:77:18\n   |\nLL |     let _ = Some(format!(\"{}\", a + \"bar\"));\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `a + \"bar\"`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2302,"byte_end":2331,"line_start":81,"line_end":81,"column_start":22,"column_end":51,"is_primary":true,"text":[{"text":"    let _s: String = format!(\"{}\", &*v.join(\"\\n\"));","highlight_start":22,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2302,"byte_end":2331,"line_start":81,"line_end":81,"column_start":22,"column_end":51,"is_primary":true,"text":[{"text":"    let _s: String = format!(\"{}\", &*v.join(\"\\n\"));","highlight_start":22,"highlight_end":51}],"label":null,"suggested_replacement":"(&*v.join(\"\\n\")).to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:81:22\n   |\nLL |     let _s: String = format!(\"{}\", &*v.join(\"\\n\"));\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `(&*v.join(\"\\n\")).to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2419,"byte_end":2433,"line_start":87,"line_end":87,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = format!(\"{x}\");","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2419,"byte_end":2433,"line_start":87,"line_end":87,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = format!(\"{x}\");","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":"x.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:87:13\n   |\nLL |     let _ = format!(\"{x}\");\n   |             ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2500,"byte_end":2521,"line_start":89,"line_end":89,"column_start":13,"column_end":34,"is_primary":true,"text":[{"text":"    let _ = format!(\"{y}\", y = x);","highlight_start":13,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2500,"byte_end":2521,"line_start":89,"line_end":89,"column_start":13,"column_end":34,"is_primary":true,"text":[{"text":"    let _ = format!(\"{y}\", y = x);","highlight_start":13,"highlight_end":34}],"label":null,"suggested_replacement":"x.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:89:13\n   |\nLL |     let _ = format!(\"{y}\", y = x);\n   |             ^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2576,"byte_end":2592,"line_start":93,"line_end":93,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    let _ = format!(\"{abc}\");","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2576,"byte_end":2592,"line_start":93,"line_end":93,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    let _ = format!(\"{abc}\");","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":"abc.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:93:13\n   |\nLL |     let _ = format!(\"{abc}\");\n   |             ^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `abc.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2625,"byte_end":2640,"line_start":95,"line_end":95,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = format!(\"{xx}\");","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2625,"byte_end":2640,"line_start":95,"line_end":95,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = format!(\"{xx}\");","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"xx.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:95:13\n   |\nLL |     let _ = format!(\"{xx}\");\n   |             ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `xx.to_string()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
