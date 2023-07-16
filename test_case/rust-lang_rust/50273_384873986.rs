plain
[00:40:36] running 1376 tests
[00:40:40] ..................................................................................i.................
[00:40:46] ..............................i.....................................................................
[00:40:49] ....................................................................................................
[00:40:53] ............................................................................................F.......
[00:40:56] ....................................................................................................
[00:41:00] ......................................................................F.............................
[00:41:11] ....................................................................................................
[00:41:16] ....................................................................................................
[00:41:23] ..................................i.................................................................
[00:41:28] ..........i.........................................................................................
---
[00:41:45] 
[00:41:45] ---- [ui] ui/error-codes/E0518.rs stdout ----
[00:41:45]  diff of stderr:
[00:41:45] 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] + error[E0518]: attribute should be applied to function or closure
[00:41:45] 2   --> $DIR/E0518.rs:11:1
[00:41:45] 3    |
[00:41:45] 4 LL | #[inline(always)] //~ ERROR: E0518
[00:41:45] 5    | ^^^^^^^^^^^^^^^^^
[00:41:45] 5    | ^^^^^^^^^^^^^^^^^
[00:41:45] 6 LL | struct Foo;
[00:41:45] -    | ----------- not a function
[00:41:45] +    | ----------- not a function or closure
[00:41:45] 8 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] + error[E0518]: attribute should be applied to function or closure
[00:41:45] 10   --> $DIR/E0518.rs:14:1
[00:41:45] 11    |
[00:41:45] 12 LL |   #[inline(never)] //~ ERROR: E0518
[00:41:45] 13    |   ^^^^^^^^^^^^^^^^
[00:41:45] 13    |   ^^^^^^^^^^^^^^^^
[00:41:45] 14 LL | / impl Foo {
[00:41:45] 15 LL | | }
[00:41:45] -    | |_- not a function
[00:41:45] +    | |_- not a function or closure
[00:41:45] 18 error: aborting due to 2 previous errors
[00:41:45] 19 
[00:41:45] 
[00:41:45] 
[00:41:45] 
[00:41:45] The actual stderr differed from the expected stderr.
[00:41:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0518.stderr
[00:41:45] To update references, run this command from build directory:
[00:41:45] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0518.rs'
[00:41:45] error: 1 errors occurred comparing output.
[00:41:45] status: exit code: 101
[00:41:45] status: exit code: 101
[00:41:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0518.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0518.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0518.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:41:45] ------------------------------------------
[00:41:45] ------------------------------------------
[00:41:45]  |     //~^ ERROR attribute should be applied to function
[00:41:45] 13 LL | | }
[00:41:45] -    | |_- not a function
[00:41:45] +    | |_- not a function or closure
[00:41:45] 15 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] + error[E0518]: attribute should be applied to function or closure
[00:41:45] 18    |
[00:41:45] 18    |
[00:41:45] 19 LL |     mod inner { #![inline="2100"] }
[00:41:45] 
[00:41:45] -    |     ------------^^^^^^^^^^^^^^^^^-- not a function
[00:41:45] +    |     ------------^^^^^^^^^^^^^^^^^-- not a function or closure
[00:41:45] 21 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] + error[E0518]: attribute should be applied to function or closure
[00:41:45] 24    |
[00:41:45] 24    |
[00:41:45] 25 LL |     #[inline = "2100"] struct S;
[00:41:45] 
[00:41:45] -    |     ^^^^^^^^^^^^^^^^^^ --------- not a function
[00:41:45] +    |     ^^^^^^^^^^^^^^^^^^ --------- not a function or closure
[00:41:45] 27 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] + error[E0518]: attribute should be applied to function or closure
[00:41:45] 30    |
[00:41:45] 30    |
[00:41:45] 31 LL |     #[inline = "2100"] type T = S;
[00:41:45] 
[00:41:45] -    |     ^^^^^^^^^^^^^^^^^^ ----------- not a function
[00:41:45] +    |     ^^^^^^^^^^^^^^^^^^ ----------- not a function or closure
[00:41:45] 33 
[00:41:45] - error[E0518]: attribute should be applied to function
[00:41:45] +rc/test/ui/feature-gate/issue-43106-gating-of-inline.rs","byte_start":1164,"byte_end":1175,"line_start":32,"line_end":32,"column_start":24,"column_end":35,"is_primary":false,"text":[{"text":"    #[inline = \"2100\"] type T = S;","highlight_start":24,"highlight_end":35}],"label":"not a function or closure","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs","byte_start":1145,"byte_end":1163,"line_start":32,"line_end":32,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    #[inline = \"2100\"] type T = S;","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0518]: attribute should be applied to function or closure\n  --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs:32:5\n   |\nLL |     #[inline = \"2100\"] type T = S;\n   |     ^^^^^^^^^^^^^^^^^^ ----------- not a function or closure\n\n"}
[00:41:45] {"message":"attribute should be applied to function or closure","code":{"code":"E0518","explanation":"\nThis error indicates that an `#[inline(..)]` attribute was incorrectly placed\non something other than a function or method.\n\nExamples of erroneous code:\n\n