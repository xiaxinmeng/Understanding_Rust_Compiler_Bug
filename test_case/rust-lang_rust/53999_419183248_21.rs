\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs","byte_start":709,"byte_end":714,"line_start":23,"line_end":23,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        crate struct XX;","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(crate_visibility_modifier)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: `crate` visibility modifier is experimental (see issue #45388)\n  --> /checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs:23:9\n   |\nLL |         crate struct XX;\n   |         ^^^^^\n   |\n   = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable\n\n"}
[00:46:26] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous e^^^^^^^^^^^
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] + LL |     crate fn b() {}
[00:46:26] +    |
[00:46:26] +    |
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] 14 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-nested-paths.rs:31:13
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-paths.rs:23:5
[00:46:26] 17    |
[00:46:26] - LL |         use foo::{self as x, c};
[00:46:26] -    |             ^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{self as x, c}`
[00:46:26] + LL |     crate fn c() {}
[00:46:26] 20    |
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] - error: aborting due to 2 previous errors
[00:46:26] + error: aborting due to 3 previous errors
[00:46:26] 25 
[00:46:26] + For more information about this error, try `rustc --explain E0658`.
[00:46:26] + For more information about this error, try `rustc --explain E0658`.
[00:46:26] 26 
[00:46:26] 
[00:46:26] 
[00:46:26] The actual stderr differed from the expected stderr.
[00:46:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/edition-lint-nested-paths.stderr
[00:46:26] diff of fixed:
[00:46:26] 
[00:46:26] 13 #![feature(rust_2018_preview)]
[00:46:26] 14 #![deny(absolute_paths_not_starting_with_crate)]
[00:46:26] 15 
[00:46:26] - use crate::foo::{a, b};
[00:46:26] + use foo::{a, b};
[00:46:26] 17 //~^ ERROR absolute paths must start with
[00:46:26] 18 //~| this was previously accepted
[00:46:26] 
[00:46:26] 28     b();
[00:46:26] 29 
[00:46:26] 30     {
[00:46:26] 30     {
[00:46:26] -         use crate::foo::{self as x, c};
[00:46:26] +         use foo::{self as x, c};
[00:46:26] 32         //~^ ERROR absolute paths must start with
[00:46:26] 33         //~| this was previously accepted
[00:46:26] 34         x::a();
[00:46:26] 
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/edition-lint-nested-paths.fixed
[00:46:26] To update references, rerun the tests and pass the `--bless` flag
[00:46:26] To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-paths.rs`
[00:46:26] error: 2 errors occurred comparing output.
[00:46:26] status: exit code: 1
[00:46:26] status: exit code: 1
[00:46:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/auxiliary" "-A" "unused"
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] ------------------------------------------
[00:46:26] stderr:
[00:46:26] stderr:
[00:46:26] ------------------------------------------
[00:46:26] {"message":"`crate` visibility modifier is experimental (see issue #45388)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n