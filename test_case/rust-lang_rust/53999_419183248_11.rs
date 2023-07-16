\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs","byte_start":625,"byte_end":630,"line_start":21,"line_end":21,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    crate struct Baz { }","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(crate_visibility_modifier)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: `crate` visibility modifier is experimental (see issue #45388)\n  --> /checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs:21:5\n   |\nLL |     crate struct Baz { }\n   |     ^^^^^\n   |\n   = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable\n\n"}
[00:46:26] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:26] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] thread '[ui] ui/rust-2018/edition-lint-fully-qualified-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:26] 
[00:46:26] 
[00:46:26] ---- [ui] ui/rust-2018/edition-lint-nested-empty-paths.rs stdout ----
[00:46:26] diff of stderr:
[00:46:26] 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-nested-empty-paths.rs:27:5
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-empty-paths.rs:18:1
[00:46:26] 3    |
[00:46:26] - LL | use foo::{bar::{baz::{}}};
[00:46:26] -    |     ^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}}}`
[00:46:26] + LL | crate mod foo {
[00:46:26] 6    |
[00:46:26] - note: lint level defined here
[00:46:26] -   --> $DIR/edition-lint-nested-empty-paths.rs:14:9
[00:46:26] -   --> $DIR/edition-lint-nested-empty-paths.rs:14:9
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] + 
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-empty-paths.rs:19:5
[00:46:26] 9    |
[00:46:26] - LL | #![deny(absolute_paths_not_starting_with_crate)]
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] + LL |     crate mod bar {
[00:46:26] +    |
[00:46:26] +    |
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] 14 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-nested-empty-paths.rs:31:5
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-empty-paths.rs:20:9
[00:46:26] 17    |
[00:46:26] - LL | use foo::{bar::{XX, baz::{}}};
[00:46:26] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{XX, baz::{}}}`
[00:46:26] + LL |         crate mod baz { }
[00:46:26] 20    |
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] 23 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-nested-empty-paths.rs:35:5
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-empty-paths.rs:21:9
[00:46:26] 26    |
[00:46:26] - LL | use foo::{bar::{baz::{}, baz1::{}}};
[00:46:26] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}, baz1::{}}}`
[00:46:26] + LL |         crate mod baz1 { }
[00:46:26] 29    |
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] - error: aborting due to 3 previous errors
[00:46:26] - error: aborting due to 3 previous errors
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-nested-empty-paths.rs:23:9
[00:46:26] +    |
[00:46:26] + LL |         crate struct XX;
[00:46:26] +    |
[00:46:26] +    |
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] + error: aborting due to 5 previous errors
[00:46:26] + 
[00:46:26] + For more information about this error, try `rustc --explain E0658`.
[00:46:26] 35 
---
[00:46:26] 
[00:46:26] 24     }
[00:46:26] 25 }
[00:46:26] 26 
[00:46:26] - use crate::foo::{bar::{baz::{}}};
[00:46:26] + use foo::{bar::{baz::{}}};
[00:46:26] 28 //~^ ERROR absolute paths must start with
[00:46:26] 29 //~| WARN this was previously accepted
[00:46:26] 
[00:46:26] 
[00:46:26] - use crate::foo::{bar::{XX, baz::{}}};
[00:46:26] + use foo::{bar::{XX, baz::{}}};
[00:46:26] 32 //~^ ERROR absolute paths must start with
[00:46:26] 33 //~| WARN this was previously accepted
[00:46:26] 
[00:46:26] 
[00:46:26] - use crate::foo::{bar::{baz::{}, baz1::{}}};
[00:46:26] + use foo::{bar::{baz::{}, baz1::{}}};
[00:46:26] 36 //~^ ERROR absolute paths must start with
[00:46:26] 37 //~| WARN this was previously accepted
[00:46:26] 
[00:46:26] 
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/edition-lint-nested-empty-paths.fixed
[00:46:26] To update references, rerun the tests and pass the `--bless` flag
[00:46:26] To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-empty-paths.rs`
[00:46:26] error: 2 errors occurred comparing output.
[00:46:26] status: exit code: 1
[00:46:26] status: exit code: 1
[00:46:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/auxiliary" "-A" "unused"
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] ------------------------------------------
[00:46:26] stderr:
[00:46:26] stderr:
[00:46:26] ------------------------------------------
[00:46:26] {"message":"`crate` visibility modifier is experimental (see issue #45388)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n