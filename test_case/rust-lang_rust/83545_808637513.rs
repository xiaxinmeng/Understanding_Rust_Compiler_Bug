plain

 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:9:28
    |
 LL |     writeln!(&mut v, "{}", "{hello}");
    |
    |
    = note: `-D clippy::write-literal` implied by `-D warnings`
    |
    |
 LL |     writeln!(&mut v, "{{hello}}");
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:10:29
    |
    |
 LL |     writeln!(&mut v, r"{}", r"{hello}");
error: test failed, to rerun pass '--test compile-test'
    |
 help: try this
    |
    |
 LL |     writeln!(&mut v, r"{{hello}}");
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:11:28
    |
    |
 LL |     writeln!(&mut v, "{}", '/'');
    |
 help: try this
    |
    |
 LL |     writeln!(&mut v, "'");
    |                       ^--
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:12:28
    |
    |
 LL |     writeln!(&mut v, "{}", '"');
    |
 help: try this
    |
    |
 LL |     writeln!(&mut v, "/"");
    |                       ^^--
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:14:29
    |
    |
 LL |     writeln!(&mut v, r"{}", '/'');
    |
 help: try this
    |
    |
 LL |     writeln!(&mut v, r"'");
    |                        ^--
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:18:9
    |
 LL | /         "hello /
 LL | /         "hello /
 LL | |         world!"
    | |_______________^
    |
 help: try this
    |
 LL |         "some hello /
    |
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:25:9
   --> $DIR/write_literal_2.rs:25:9
    |
 LL |         "1", "2", "3",
    |         ^^^
    |
 help: try this
    |
-LL |         "some 1{} / {}", "2", "3",
-   |               ^        --
+LL |         "some 1/
+LL |         {} / {}", "2", "3",
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:25:14
    |
---
    |                   ^^^
    |
 help: try this
    |
 LL |         {} / 3",
 LL |         "1", "2",
 
 error: aborting due to 9 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-19ca3b17afe90112/out/test_build_base/write_literal_2.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args write_literal_2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/write_literal_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-19ca3b17afe90112/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-19ca3b17afe90112/out/test_build_base/write_literal_2.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-2a79111c7ffd0851.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-3e6c923d343daab8.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3ddf9f6cc09731de.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-9816c8a0844f9255.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-19ca3b17afe90112/out/test_build_base/write_literal_2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":149,"byte_end":158,"line_start":9,"line_end":9,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", \"{hello}\");","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::write-literal` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":147,"byte_end":158,"line_start":9,"line_end":9,"column_start":26,"column_end":37,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", \"{hello}\");","highlight_start":26,"highlight_end":37}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":144,"byte_end":146,"line_start":9,"line_end":9,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", \"{hello}\");","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":"{{hello}}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:9:28\n   |\nLL |     writeln!(&mut v, \"{}\", \"{hello}\");\n   |                            ^^^^^^^^^\n   |\n   = note: `-D clippy::write-literal` implied by `-D warnings`\nhelp: try this\n   |\nLL |     writeln!(&mut v, \"{{hello}}\");\n   |                       ^^^^^^^^^--\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":189,"byte_end":199,"line_start":10,"line_end":10,"column_start":29,"column_end":39,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", r\"{hello}\");","highlight_start":29,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":187,"byte_end":199,"line_start":10,"line_end":10,"column_start":27,"column_end":39,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", r\"{hello}\");","highlight_start":27,"highlight_end":39}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":184,"byte_end":186,"line_start":10,"line_end":10,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", r\"{hello}\");","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":"{{hello}}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:10:29\n   |\nLL |     writeln!(&mut v, r\"{}\", r\"{hello}\");\n   |                             ^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL |     writeln!(&mut v, r\"{{hello}}\");\n   |                        ^^^^^^^^^--\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":229,"byte_end":233,"line_start":11,"line_end":11,"column_start":28,"column_end":32,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\\'');","highlight_start":28,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":227,"byte_end":233,"line_start":11,"line_end":11,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\\'');","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":224,"byte_end":226,"line_start":11,"line_end":11,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\\'');","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":"'","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:11:28\n   |\nLL |     writeln!(&mut v, \"{}\", '\\'');\n   |                            ^^^^\n   |\nhelp: try this\n   |\nLL |     writeln!(&mut v, \"'\");\n   |                       ^--\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":263,"byte_end":266,"line_start":12,"line_end":12,"column_start":28,"column_end":31,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\"');","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":261,"byte_end":266,"line_start":12,"line_end":12,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\"');","highlight_start":26,"highlight_end":31}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":258,"byte_end":260,"line_start":12,"line_end":12,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"    writeln!(&mut v, \"{}\", '\"');","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":"\\\"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:12:28\n   |\nLL |     writeln!(&mut v, \"{}\", '\"');\n   |                            ^^^\n   |\nhelp: try this\n   |\nLL |     writeln!(&mut v, \"\\\"\");\n   |                       ^^--\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":345,"byte_end":349,"line_start":14,"line_end":14,"column_start":29,"column_end":33,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", '\\'');","highlight_start":29,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":343,"byte_end":349,"line_start":14,"line_end":14,"column_start":27,"column_end":33,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", '\\'');","highlight_start":27,"highlight_end":33}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":340,"byte_end":342,"line_start":14,"line_end":14,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    writeln!(&mut v, r\"{}\", '\\'');","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":"'","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:14:29\n   |\nLL |     writeln!(&mut v, r\"{}\", '\\'');\n   |                             ^^^^\n   |\nhelp: try this\n   |\nLL |     writeln!(&mut v, r\"'\");\n   |                        ^--\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":409,"byte_end":433,"line_start":18,"line_end":19,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        \"hello \\","highlight_start":9,"highlight_end":17},{"text":"        world!\"","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":399,"byte_end":433,"line_start":17,"line_end":19,"column_start":18,"column_end":16,"is_primary":true,"text":[{"text":"        \"some {}\",","highlight_start":18,"highlight_end":19},{"text":"        \"hello \\","highlight_start":1,"highlight_end":17},{"text":"        world!\"","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":396,"byte_end":398,"line_start":17,"line_end":17,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"        \"some {}\",","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":"hello \\\n        world!","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:18:9\n   |\nLL | /         \"hello \\\nLL | |         world!\"\n   | |_______________^\n   |\nhelp: try this\n   |\nLL |         \"some hello \\\nLL |         world!\"\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":516,"byte_end":519,"line_start":25,"line_end":25,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        \"1\", \"2\", \"3\",","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":506,"byte_end":519,"line_start":24,"line_end":25,"column_start":18,"column_end":12,"is_primary":true,"text":[{"text":"        {} \\\\ {}\",","highlight_start":18,"highlight_end":19},{"text":"        \"1\", \"2\", \"3\",","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":485,"byte_end":487,"line_start":23,"line_end":23,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"        \"some {}\\","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":"1","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:25:9\n   |\nLL |         \"1\", \"2\", \"3\",\n   |         ^^^\n   |\nhelp: try this\n   |\nLL |         \"some 1\\\nLL |         {} \\\\ {}\", \"2\", \"3\",\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":521,"byte_end":524,"line_start":25,"line_end":25,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"        \"1\", \"2\", \"3\",","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":519,"byte_end":524,"line_start":25,"line_end":25,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"        \"1\", \"2\", \"3\",","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":497,"byte_end":499,"line_start":24,"line_end":24,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        {} \\\\ {}\",","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:25:14\n   |\nLL |         \"1\", \"2\", \"3\",\n   |              ^^^\n   |\nhelp: try this\n   |\nLL |         2 \\\\ {}\",\nLL |         \"1\", \"3\",\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":526,"byte_end":529,"line_start":25,"line_end":25,"column_start":19,"column_end":22,"is_primary":true,"text":[{"text":"        \"1\", \"2\", \"3\",","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":524,"byte_end":529,"line_start":25,"line_end":25,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"        \"1\", \"2\", \"3\",","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":503,"byte_end":505,"line_start":24,"line_end":24,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"        {} \\\\ {}\",","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":"3","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:25:19\n   |\nLL |         \"1\", \"2\", \"3\",\n   |                   ^^^\n   |\nhelp: try this\n   |\nLL |         {} \\\\ 3\",\nLL |         \"1\", \"2\",\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
