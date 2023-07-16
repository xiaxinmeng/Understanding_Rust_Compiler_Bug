plain
Resolving deltas: 100% (612195/612195), completed with 4844 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:42:06] .........................................................................i..........................
[00:42:12] ................i.............................F.....................................................
---
[00:42:48] .............................................................................................i......
[00:42:55] .................................................................i..................................
---
[00:43:17] 1 error: missing `fn`, `type`, or `const` for impl-item declaration
[00:43:17] -   --> $DIR/issue-40006.rs:11:9
[00:43:17] +   --> $DIR/issue-40006.rs:13:5
[00:43:17] 3    |
[00:43:17] - LL |   impl X { //~ ERROR cannot be made into an object
[00:43:17] -    |  _________^
[00:43:17] - LL | | //~^ ERROR missing
[00:43:17] - LL | |     Y
[00:43:17] -    | |____^ missing `fn`, `type`, or `const`
[00:43:17] + LL |     Y
[00:43:17] +    |     ^ missing `fn`, `type`, or `const`
[00:43:17] 9
[00:43:17] 10 error: missing `fn`, `type`, or `const` for trait-item declaration
[00:43:17] -   --> $DIR/issue-40006.rs:18:10
[00:43:17] +   --> $DIR/issue-40006.rs:19:5
[00:43:17] 12    |
[00:43:17] - LL |   trait X { //~ ERROR missing
[00:43:17] -    |  __________^
[00:43:17] - LL | |     X() {}
[00:43:17] -    | |____^ missing `fn`, `type`, or `const`
[00:43:17] + LL |     X() {}
[00:43:17] +    |     ^ missing `fn`, `type`, or `const`
[00:43:17] 17
[00:43:17] 18 error: expected `[`, found `#`
[00:43:17] 19   --> $DIR/issue-40006.rs:20:17
[00:43:17]
[00:43:17] 22    |                 ^ expected `[`
[00:43:17] 23
[00:43:17] 24 error: missing `fn`, `type`, or `const` for trait-item declaration
[00:43:17] -   --> $DIR/issue-40006.rs:20:21
[00:43:17] +   --> $DIR/issue-40006.rs:22:5
[00:43:17] 26    |
[00:43:17] - LL |       fn xxx() { ### } //~ ERROR missing
[00:43:17] -    |  _____________________^
[00:43:17] - LL | |     //~^ ERROR expected
[00:43:17] - LL | |     L = M; //~ ERROR missing
[00:43:17] -    | |____^ missing `fn`, `type`, or `const`
[00:43:17] + LL |     L = M; //~ ERROR missing
[00:43:17] +    |     ^ missing `fn`, `type`, or `const`
[00:43:17] 32
[00:43:17] 33 error: missing `fn`, `type`, or `const` for trait-item declaration
[00:43:17] -   --> $DIR/issue-40006.rs:22:11
[00:43:17] +   --> $DIR/issue-40006.rs:23:5
[00:43:17] 35    |
[00:43:17] - LL |       L = M; //~ ERROR missing
[00:43:17] -    |  ___________^
[00:43:17] - LL | |     Z = { 2 + 3 }; //~ ERROR expected one of
[00:43:17] -    | |____^ missing `fn`, `type`, or `const`
[00:43:17] + LL |     Z = { 2 + 3 }; //~ ERROR expected one of
[00:43:17] +    |     ^ missing `fn`, `type`, or `const`
[00:43:17] 40
[00:43:17] 41 error: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`
[00:43:17] 42   --> $DIR/issue-40006.rs:23:18
[00:43:17]
[00:43:17] 51    |         ^ expected one of `!` or `::` here
[00:43:17] 52
[00:43:17] 53 error: missing `fn`, `type`, or `const` for impl-item declaration
[00:43:17] -   --> $DIR/issue-40006.rs:28:8
[00:43:17] +   --> $DIR/issue-40006.rs:28:9
[00:43:17] 55    |
[00:43:17] 56 LL |     pub hello_method(&self) { //~ ERROR missing
[00:43:17] -    |        ^ missing `fn`, `type`, or `const`
[00:43:17] +    |         ^^^^^^^^^^^^ missing `fn`, `type`, or `const`
[00:43:17] 58
[00:43:17] 59 error[E0038]: the trait `X` cannot be made into an object
[00:43:17] 60   --> $DIR/issue-40006.rs:11:6
[00:43:17]
[00:43:17]
[00:43:17] The actual stderr differed from the expected stderr.
[00:43:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006.stderr
[00:43:17] To update references, run this command from build directory:
[00:43:17] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'did_you_mean/issue-40006.rs'
[00:43:17]
[00:43:17] error: 1 errors occurred comparing output.
[00:43:17] status: exit code: 101
[00:43:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40006.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:43:17] {"message":"missing `fn`, `type`, or `const` for impl-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":539,"byte_end":540,"line_start":13,"line_end":13,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    Y","highlight_start":5,"highlight_end":6}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:13:5\n   |\nLL |     Y\n   |     ^ missing `fn`, `type`, or `const`\n\n"}
[00:43:17] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":587,"byte_end":588,"line_start":19,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    X() {}","highlight_start":5,"highlight_end":6}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:19:5\n   |\nLL |     X() {}\n   |     ^ missing `fn`, `type`, or `const`\n\n"}
[00:43:17] {"message":"expected `[`, found `#`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":610,"byte_end":611,"line_start":20,"line_end":20,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    fn xxx() { ### } //~ ERROR missing","highlight_start":17,"highlight_end":18}],"label":"expected `[`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected `[`, found `#`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:20:17\n   |\nLL |     fn xxx() { ### } //~ ERROR missing\n   |                 ^ expected `[`\n\n"}
[00:43:17] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":661,"byte_end":662,"line_start":22,"line_end":22,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    L = M; //~ ERROR missing","highlight_start":5,"highlight_end":6}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:22:5\n   |\nLL |     L = M; //~ ERROR missing\n   |     ^ missing `fn`, `type`, or `const`\n\n"}
[00:43:17] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":690,"byte_end":691,"line_start":23,"line_end":23,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_start":5,"highlight_end":6}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:23:5\n   |\nLL |     Z = { 2 + 3 }; //~ ERROR expected one of\n   |     ^ missing `fn`, `type`, or `const`\n\n"}
[00:43:17] {"message":"expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":703,"byte_end":704,"line_start":23,"line_end":23,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    Z = { 2 + 3 }; //~ ERROR expected one of","highlight_start":18,"highlight_end":19}],"label":"expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}` here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:23:18\n   |\nLL |     Z = { 2 + 3 }; //~ ERROR expected one of\n   |                  ^ expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}` here\n\n"}
[00:43:17] {"message":"expected one of `!` or `::`, found `(`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":739,"byte_end":740,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    ::Y (); //~ ERROR expected one of","highlight_start":9,"highlight_end":10}],"label":"expected one of `!` or `::` here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!` or `::`, found `(`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:24:9\n   |\nLL |     ::Y (); //~ ERROR expected one of\n   |         ^ expected one of `!` or `::` here\n\n"}
[00:43:17] {"message":"missing `fn`, `type`, or `const` for impl-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":789,"byte_end":801,"line_start":28,"line_end":28,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"    pub hello_method(&self) { //~ ERROR missing","highlight_start":9,"highlight_end":21}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:28:9\n   |\nLL |     pub hello_method(&self) { //~ ERROR missing\n   |         ^^^^^^^^^^^^ missing `fn`, `type`, or `const`\n\n"}
[00:43:17] {"message":"the trait `X` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n