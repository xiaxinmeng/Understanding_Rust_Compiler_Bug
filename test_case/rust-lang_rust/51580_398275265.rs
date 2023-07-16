plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:42:41] 
[00:42:41] running 1507 tests
[00:42:46] .............................................................................................i......
[00:42:51] .......................................................i............................F...............
[00:42:59] ....................................................................................................
[00:43:02] ....................................................................................................
[00:43:05] ....................................................................................................
[00:43:10] ....................................................................................................
[00:43:10] ....................................................................................................
[00:43:15] ....................................................................................................
[00:43:20] ....................................................................................................
[00:43:26] ....................................................................................................
[00:43:31] ........i...............................................................................i...........
[00:43:36] ....................................................................................................
[00:43:42] ....................................................................................................
[00:43:48] ....................................................................................................
, `type`, `unsafe`, or `}`, found `;`
[00:43:54] + error: expected one of `async`, `const`, `extern`, `fn`, `type`, `unsafe`, or `}`, found `;`
[00:43:54] 42   --> $DIR/issue-40006.rs:23:18
[00:43:54] 43    |
[00:43:54] 44 LL |     Z = { 2 + 3 }; //~ ERROR expected one of
[00:43:54] 
[00:43:54] -    |                  ^ expected one of `const`, `extern`, `fn`, `type`, `unsafe`, or `}` here
[00:43:54] +    |                  ^ expected one of 7 possible tokens here
[00:43:54] 46 
[00:43:54] 47 error: expected one of `!` or `::`, found `(`
[00:43:54] 48   --> $DIR/issue-40006.rs:24:9
[00:43:54] 
[00:43:54] The actual stderr differed from the expected stderr.
[00:43:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/issue-40006.stderr
[00:43:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/issue-40006.stderr
[00:43:54] To update references, rerun the tests and pass the `--bless` flag
[00:43:54] To only update this specific test, also pass `--test-args did_you_mean/issue-40006.rs`
[00:43:54] error: 1 errors occurred comparing output.
[00:43:54] status: exit code: 101
[00:43:54] status: exit code: 101
[00:43:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40006.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/auxiliary" "-A" "unused"
[00:43:54] ------------------------------------------
[00:43:54] 
[00:43:54] ------------------------------------------
[00:43:54] stderr:
[00:43:54] stderr:
[00:43:54] ------------------------------------------
[00:43:54] {"message":"missing `fn`, `type`, or `const` for impl-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":475,"byte_end":539,"line_start":11,"line_end":13,"column_start":9,"column_end":5,"is_primary":true,"text":[{"text":"impl X { //~ ERROR cannot be made into an object","highlight_start":9,"highlight_end":49},{"text":"//~^ ERROR missing","highlight_start":1,"highlight_end":19},{"text":"    Y","highlight_start":1,"highlight_end":5}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:11:9\n   |\nLL |   impl X { //~ ERROR cannot be made into an object\n   |  _________^\nLL | | //~^ ERROR missing\nLL | |     Y\n   | |____^ missing `fn`, `type`, or `const`\n\n"}
[00:43:54] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":564,"byte_end":587,"line_start":18,"line_end":19,"column_start":10,"column_end":5,"is_primary":true,"text":[{"text":"trait X { //~ ERROR missing","highlight_start":10,"highlight_end":28},{"text":"    X() {}","highlight_start":1,"highlight_end":5}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for trait-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:18:10\n   |\nLL |   trait X { //~ ERROR missing\n   |  __________^\nLL | |     X() {}\n   | |____^ missing `fn`, `type`, or `const`\n\n"}
[00:43:54] {"message":"expected `[`, found `#`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":610,"byte_end":611,"line_start":20,"line_end":20,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    fn xxx() { ### } //~ ERROR missing","highlight_start":17,"highlight_end":18}],"label":"expected `[`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected `[`, found `#`\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:20:17\n   |\nLL |     fn xxx() { ### } //~ ERROR missing\n   |                 ^ expected `[`\n\n"}
[00:43:54] {"message":"missing `fn`, `type`, or `const` for trait-item declaration","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":614,"byte_end":661,"line_start":20,"line_end":22,"column_start":21,"column_end":5,"is_primary":true,"text":[{"text":"    fn xxx() { ### } //~ ERROR missing","highlight_start":21,"highlight_end":39},{"text":"    //~^ ERROR expected","highlight_start":1,"highlight_end":24},{"text":"    L = M; //~ ERROR missing","highlight_start":1,"highlight_end":5}],"label":"missing `fn[{"text":"    pub hello_method(&self) { //~ ERROR missing","highlight_start":8,"highlight_end":9}],"label":"missing `fn`, `type`, or `const`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing `fn`, `type`, or `const` for impl-item declaration\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:28:8\n   |\nLL |     pub hello_method(&self) { //~ ERROR missing\n   |        ^ missing `fn`, `type`, or `const`\n\n"}
[00:43:54] {"message":"the trait `X` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like theods. With such a bound, one can still call `foo()` on types implementing\nthat trait that aren't behind trait objects.\n\n### Method has generic type parameters\n\nAs mentioned before, trait objects contain pointers to method tables. So, if we\nhave:\n\n