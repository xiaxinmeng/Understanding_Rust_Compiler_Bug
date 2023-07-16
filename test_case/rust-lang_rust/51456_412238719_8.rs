\n\nPlease verify that the name wasn'tper-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/hygiene/globs.rs","byte_start":1722,"byte_end":1723,"line_start":75,"line_end":75,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"                f //~ ERROR cannot find function `f` in this scope","highlight_start":17,"highlight_end":18}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/hygiene/globs.rs","byte_start":797,"byte_end":803,"line_start":36,"line_end":36,"column_start":1,"column_end":7,"is_primary":false,"text":[{"text":"n!(f);","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"n!","def_site_span":{"file_name":"/checkout/src/test/ui/hygiene/globs.rs","byte_start":804,"byte_end":1871,"line_start":37,"line_end":82,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro n($i:ident) {","highlight_start":1,"highlight_end":20},{"text":"    mod foo {","highlight_start":1,"highlight_end":14},{"text":"        pub fn $i() -> u32 { 0 }","highlight_start":1,"highlight_end":33},{"text":"        pub fn f() {}","highlight_start":1,"highlight_end":22},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        mod test {","highlight_start":1,"highlight_end":19},{"text":"            use super::*;","highlight_start":1,"highlight_end":26},{"text":"            fn g() {","highlight_start":1,"highlight_end":21},{"text":"                ong_bounds.rs","byte_start":681,"byte_end":686,"line_start":20,"line_end":20,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"fn wants_display(g: impl Debug) { } //~ ERROR cannot find","highlight_start":26,"highlight_end":31}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0405]: cannot find trait `Debug` in this scope\n  --> /checkout/src/test/ui/impl-trait/universal_wrong_bounds.rs:20:26\n   |\nLL | fn wants_display(g: impl Debug) { } //~ ERROR cannot find\n   |                          ^^^^^ not found in this scope\n\n"}
[00:49:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:33] {"message":"For more information about this error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0405`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/impl-trait/universal_wrong_bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/issue-17546.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 5    |                 --------^^^^^^^^^^^^^^^^
[00:49:33] 6    |                 |
[00:49:33] 7    |                 did you mean `Result`?
[00:49:33] -    |                 help: you can try using the variant's enum: `foo::MyEnum`
[00:49:33] 10 error[E0573]: expected type, found variant `Result`
[00:49:33] 11   --> $DIR/issue-17546.rs:32:17
[00:49:33] 
[00:49:33] 12    |
[00:49:33] 12    |
[00:49:33] 13 LL |     fn new() -> Result<foo::MyEnum, String> {
[00:49:33] - help: possible better candidates are found in other modules, you can import them into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL |     use std::fmt::Result;
[00:49:33] -    |
[00:49:33] - LL |     use std::io::Result;
[00:49:33] -    |
[00:49:33] - LL |     use std::prelude::v1::Result;
[00:49:33] -    |
[00:49:33] - LL |     use std::result::Result;
[00:49:33] - and 1 other candidates
[00:49:33] 26 
[00:49:33] 27 error[E0573]: expected type, found variant `Result`
[00:49:33] 28   --> $DIR/issue-17546.rs:38:13
[00:49:33] 28   --> $DIR/issue-17546.rs:38:13
[00:49:33] 
[00:49:33] 29    |
[00:49:33] 30 LL | fn new() -> Result<foo::MyEnum, String> {
[00:49:33] - help: possible better candidates are found in other modules, you can import them into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use std::fmt::Result;
[00:49:33] -    |
[00:49:33] - LL | use std::io::Result;
[00:49:33] -    |
[00:49:33] - LL | use std::prelude::v1::Result;
[00:49:33] -    |
[00:49:33] - LL | use std::result::Result;
[00:49:33] - and 1 other candidates
[00:49:33] 43 
[00:49:33] 44 error[E0573]: expected type, found variant `NoResult`
[00:49:33] 45   --> $DIR/issue-17546.rs:43:15
[00:49:33] 45   --> $DIR/issue-17546.rs:43:15
[00:49:33] 
[00:49:33] 48    |               --------^^^^^^^^^^^^^^^^^^^^^
[00:49:33] 49    |               |
[00:49:33] 50    |               did you mean `Result`?
[00:49:33] -    |               help: you can try using the variant's enum: `foo::MyEnum`
[00:49:33] 53 error: aborting due to 4 previous errors
[00:49:33] 54 
[00:49:33] 
[00:49:33] 
[00:49:33] 
[00:49:33] The actual stderr differed from the expected stderr.
[00:49:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17546/issue-17546.stderr
[00:49:33] To update references, rerun the tests and pass the `--bless` flag
[00:49:33] To only update this specific test, also pass `--test-args issue-17546.rs`
[00:49:33] error: 1 errors occurred comparing output.
[00:49:33] status: exit code: 1
[00:49:33] status: exit code: 1
[00:49:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-17546.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17546/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17546/auxiliary" "-A" "unused"
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] ------------------------------------------
[00:49:33] stderr:
[00:49:33] stderr:
[00:49:33] ------------------------------------------
[00:49:33] {"message":"expected type, found variant `NoResult`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-17546.rs","byte_start":660,"byte_end":668,"lin.rs","byte_start":573,"byte_end":576,"line_start":16,"line_end":16,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"    Foo(Foo<T>) //~ ERROR cannot find type `Foo` in this scope","highlight_start":9,"highlight_end":12}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0412]: cannot find type `Foo` in this scope\n  --> /checkout/src/test/ui/issue-35075.rs:16:9\n   |\nLL |     Foo(Foo<T>) //~ ERROR cannot find type `Foo` in this scope\n   |         ^^^ not found in this scope\n\n"}
[00:49:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:33] {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0412`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/issue-35075.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/issue-35675.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 2   --> $DIR/issue-35675.rs:17:29
[00:49:33] 3    |
[00:49:33] 4 LL | fn should_return_fruit() -> Apple {
[00:49:33] -    |                             |
[00:49:33] -    |                             not found in this scope
[00:49:33] -    |                             not found in this scope
[00:49:33] -    |                             help: you can try using the variant's enum: `Fruit`
[00:49:33] 9 
[00:49:33] 9 
[00:49:33] 10 error[E0425]: cannot find function `Apple` in this scope
[00:49:33] 
[00:49:33] 12    |
[00:49:33] 12    |
[00:49:33] 13 LL |     Apple(5)
[00:49:33] - help: possible candidate is found in another module, you can import it into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use Fruit::Apple;
[00:49:33] 19 
[00:49:33] 19 
[00:49:33] 20 error[E0573]: expected type, found variant `Fruit::Apple`
[00:49:33] 
[00:49:33] 22    |
[00:49:33] 22    |
[00:49:33] 23 LL | fn should_return_fruit_too() -> Fruit::Apple {
[00:49:33] -    |                                 |
[00:49:33] -    |                                 not a type
[00:49:33] -    |                                 not a type
[00:49:33] -    |                                 help: you can try using the variant's enum: `Fruit`
[00:49:33] 28 
[00:49:33] 28 
[00:49:33] 29 error[E0425]: cannot find function `Apple` in this scope
[00:49:33] 
[00:49:33] 31    |
[00:49:33] 31    |
[00:49:33] 32 LL |     Apple(5)
[00:49:33] - help: possible candidate is found in another module, you can import it into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use Fruit::Apple;
[00:49:33] 38 
[00:49:33] 39 error[E0573]: expected type, found variant `Ok`
[00:49:33] 40   --> $DIR/issue-35675.rs:29:13
[00:49:33] 
[00:49:33] 
[00:49:33] 41    |
[00:49:33] 42 LL | fn foo() -> Ok {
[00:49:33] 43    |             ^^ not a type
[00:49:33] -    |
[00:49:33] -    = help: there is an enum variant `std::prelude::v1::Ok`, try using `std::prelude::v1`?
[00:49:33] -    = help: there is an enum variant `std::result::Result::Ok`, try using `std::result::Result`?
[00:49:33] 47 
[00:49:33] 48 error[E0412]: cannot find type `Variant3` in this scope
[00:49:33] 
[00:49:33] 50    |
[00:49:33] 50    |
[00:49:33] 51 LL | fn bar() -> Variant3 {
[00:49:33] -    |             |
[00:49:33] -    |             not found in this scope
[00:49:33] -    |             not found in this scope
[00:49:33] -    |             help: you can try using the variant's enum: `x::Enum`
[00:49:33] 56 
[00:49:33] 56 
[00:49:33] 57 error[E0573]: expected type, found variant `Some`
[00:49:33] 
[00:49:33] 59    |
[00:49:33] 59    |
[00:49:33] 60 LL | fn qux() -> Some {
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] -    = help: there is an enum variant `std::prelude::v1::Option::Some`, try using `std::prelude::v1::Option`?
[00:49:33] -    = help: there is an enum variant `std::prelude::v1::Some`, try using `std::prelude::v1`?
[00:49:33] 66 error: aborting due to 7 previous errors
[00:49:33] 67 
[00:49:33] 
[00:49:33] 
[00:49:33] 
[00:49:33] The actual stderr differed from the expected stderr.
[00:49:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-35675/issue-35675.^^^^^^ not found in this scope\n\n"}
[00:49:33] {"message":"expected type, found variant `Some`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-35675.rs","byte_start":1125,"byte_end":1129,"line_start":38,"line_end":38,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"fn qux() -> Some {","highlight_start":13,"highlight_end":17}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expected type, found variant `Some`\n  --> /checkout/src/test/ui/issue-35675.rs:38:13\n   |\nLL | fn qux() -> Some {\n   |             ^^^^ not a type\n\n"}
[00:49:33] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:49:33] {"message":"Some errors occurred: E0412, E0425, E0573.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0412, E0425, E0573.\n"}
[00:49:33] {"message":"For more information about an error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0412`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/issue-35675.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/issue-37534.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 3    |
[00:49:33] 4 LL | struct Foo<T: ?Hash> { }
[00:49:33]","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default. Only `?Sized` is supported\n  --> /checkout/src/test/ui/issue-37534.rs:11:12\n   |\nLL | struct Foo<T: ?Hash> { }\n   |            ^\n\n"}
[00:49:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:49:33] {"message":"parameter `T` is never used","code":{"code":"E0392","explanation":"\nThis error indicates that a type or lifetime parameter has been declared\nbut not actually used. Here is an example that demonstrates the error:\n\n