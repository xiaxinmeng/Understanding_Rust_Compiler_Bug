plain
Resolving deltas: 100% (612167/612167), completed with 4847 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:39:29] .........................................................................i..........................
[00:39:35] ................i.............................F.....................................................
---
[00:40:10] .............................................................................................i......
[00:40:17] .................................................................i..................................
---
[00:40:38] 5    |  _________^
[00:40:38] 6 LL | | //~^ ERROR missing
[00:40:38] 7 LL | |     Y
[00:40:38] -    | |____^ missing `fn`, `type`, or `const`
[00:40:38] +    | |____^ `fn`, `type`, or `const`
[00:40:38] 9
[00:40:38] 10 error: missing `fn`, `type`, or `const` for trait-item declaration
[00:40:38] 11   --> $DIR/issue-40006.rs:18:10
[00:40:38]
[00:40:38] 13 LL |   trait X { //~ ERROR missing
[00:40:38] 14    |  __________^
[00:40:38] 15 LL | |     X() {}
[00:40:38] -    | |____^ missing `fn`, `type`, or `const`
[00:40:38] +    | |____^ `fn`, `type`, or `const`
[00:40:38] 17
[00:40:38] 18 error: expected `[`, found `#`
[00:40:38] 19   --> $DIR/issue-40006.rs:20:17
[00:40:38]
[00:40:38] 28    |  _____________________^
[00:40:38] 29 LL | |     //~^ ERROR expected
[00:40:38] 30 LL | |     L = M; //~ ERROR missing
[00:40:38] -    | |____^ missing `fn`, `type`, or `const`
[00:40:38] +    | |____^ `fn`, `type`, or `const`
[00:40:38] 32
[00:40:38] 33 error: missing `fn`, `type`, or `const` for trait-item declaration
[00:40:38] 34   --> $DIR/issue-40006.rs:22:11
[00:40:38]
[00:40:38] 36 LL |       L = M; //~ ERROR missing
[00:40:38] 37    |  ___________^
[00:40:38] 38 LL | |     Z = { 2 + 3 }; //~ ERROR expected one of
[00:40:38] -    | |____^ missing `fn`, `type`, or `const`
[00:40:38] +    | |____^ `fn`, `type`, or `const`
[00:40:38] 40
[00:40:38] 41 error: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`
[00:40:38] 42   --> $DIR/issue-40006.rs:23:18
[00:40:38]
[00:40:38] 54   --> $DIR/issue-40006.rs:28:8
[00:40:38] 55    |
[00:40:38] 56 LL |     pub hello_method(&self) { //~ ERROR missing
[00:40:38] -    |        ^ missing `fn`, `type`, or `const`
[00:40:38] +    |        ^ `fn`, `type`, or `const`
[00:40:38] 58
[00:40:38] 59 error[E0038]: the trait `X` cannot be made into an object
[00:40:38] 60   --> $DIR/issue-40006.rs:11:6
[00:40:38]
[00:40:38]
[00:40:38] The actual stderr differed from the expected stderr.
[00:40:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006.stderr
[00:40:38] To update references, run this command from build directory:
[00:40:38] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-40006.rs'
[00:40:38]
[00:40:38] error: 1 errors occurred comparing output.
[00:40:38] status: exit code: 101
[00:40:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40006.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj"text":"trait X { //~ ERROR missing","highlight_start":10,"highlight_end":28},{"text":"    X() {}","highlight_start":1,"highlight_end":5}],"label":"`fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:18:10\n   |\nLL |   trait X { //~ ERROR missing\n   |  __________^\nLL | |     X() {}\n   | |____^ `fn`, `type`, or `const`\n\n"}
[00:40:38] {"message":"expected `[`, found `#`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":610,"byte_end":611,"line_start":20,"line_end":20,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    fn xxx() { ### } //~ ERROR missing","highlight_start":17,"highlight_end":18}],"label":"expected `[`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected `[`, found `#`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:20:17\n   |\nLL |     fn xxx() { ### } //~ ERROR missing\n   |                 ^ expected `[`\n\n"}
[00:40:38] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":614,"byte_end":661,"line_start":20,"line_end":22,"column_start":21,"column_end":5,"is_primary":true,"text":[{"text":"    fn xxx() { ### } //~ ERROR missing","highlight_start":21,"highlight_end":39},{"text":"    //~^ ERROR expected","highlight_start":1,"highlight_end":24},{"text":"    L = M; //~ ERROR missing","highlight_start":1,"highlight_end":5}],"label":"`fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:20:21\n   |\nLL |       fn xxx() { ### } //~ ERROR missing\n   |  _____________________^\nLL | |     //~^ ERROR expected\nLL | |     L = M; //~ ERROR missing\n   | |____^ `fn`, `type`, or `const`\n\n"}
[00:40:38] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":667,"byte_end":690,"line_start":22,"line_end":23,"column_start":11,"column_end":5,"is_primary":true,"text":[{"text":"    L = M; //~ ERROR missing","highlight_start":11,"highlight_end":29},{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_start":1,"highlight_end":5}],"label":"`fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:22:11\n   |\nLL |       L = M; //~ ERROR missing\n   |  ___________^\nLL | |     Z = { 2 + 3 }; //~ ERROR expected one of\n   | |____^ `fn`, `type`, or `const`\n\n"}
[00:40:38] {"message":"expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":703,"byte_end":704,"line_start":23,"line_end":23,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_start":18,"highlight_end":19}],"label":"expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}` here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:23:18\n   |\nLL |     Z = { 2 + 3 }; //~ ERROR expected one of\n   |                  ^ expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}` here\n\n"}
[00:40:38] {"message":"expected one of `!` or `::`, found `(`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":739,"byte_end":740,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    ::Y (); //~ ERROR expected one of","highlight_start":9,"highlight_end":10}],"label":"expected one of `!` or `::` here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!` or `::`, found `(`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:24:9\n   |\nLL |     ::Y (); //~ ERROR expected one of\n   |         ^ expected one of `!` or `::` here\n\n"}
[00:40:38] {"message":"missing `fn`, `type`, or `const` for impl-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":788,"byte_end":789,"line_start":28,"line_end":28,"column_start":8,"column_end":9,"is_primary":true,"text":[{"text":"    pub hello_method(&self) { //~ E types.\n\n### Method has no receiver\n\nMethods that do not take a `self` parameter can't be called since there won't be\na way to get a pointer to the method table for them.\n\n