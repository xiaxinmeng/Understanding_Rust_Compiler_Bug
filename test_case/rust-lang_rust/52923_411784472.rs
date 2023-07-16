plain
[00:43:30] ....................................................................................................
[00:43:34] .....................i..............................................................................
[00:43:36] ...............................i....................................................................
[00:43:39] ....................................................................................................
[00:43:43] ....FF..............................................................................................
[00:43:49] .............................................i....
[00:43:49] failures:
[00:43:49] 
[00:43:49] ---- [ui] ui/rust-2018/edition-lint-paths.rs stdout ----
[00:43:49] ---- [ui] ui/rust-2018/edition-lint-paths.rs stdout ----
[00:43:49] diff of stderr:
[00:43:49] 
[00:43:49] 13    = note: for more information, see issue TBD
[00:43:49] 14 
[00:43:49] 15 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:43:49] -   --> $DIR/edition-lint-paths.rs:28:9
[00:43:49] -    |
[00:43:49] - LL |     use bar;
[00:43:49] -    |         ^^^ help: use `crate`: `crate::bar`
[00:43:49] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:43:49] -    = note: for more information, see issue TBD
[00:43:49] - 
[00:43:49] - 
[00:43:49] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:43:49] -   --> $DIR/edition-lint-paths.rs:33:9
[00:43:49] -    |
[00:43:49] - LL |     use {Bar as SomethingElse, main};
[00:43:49] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::{Bar as SomethingElse, main}`
[00:43:49] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:43:49] -    = note: for more information, see issue TBD
[00:43:49] - 
[00:43:49] - 
[00:43:49] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:43:49] 34   --> $DIR/edition-lint-paths.rs:45:5
[00:43:49] 35    |
[00:43:49] 36 LL | use bar::Bar;
[00:43:49] 40    = note: for more information, see issue TBD
[00:43:49] 41 
[00:43:49] 41 
[00:43:49] 42 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:43:49] -   --> $DIR/edition-lint-paths.rs:57:9
[00:43:49] -    |
[00:43:49] - LL |     use *;
[00:43:49] -    |         ^ help: use `crate`: `crate::*`
[00:43:49] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:43:49] -    = note: for more information, see issue TBD
[00:43:49] - 
[00:43:49] - 
[00:43:49] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:43:49] 52   --> $DIR/edition-lint-paths.rs:62:6
[00:43:49] 53    |
[00:43:49] 54 LL | impl ::foo::SomeTrait for u32 { }
[00:43:49] 66    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:43:49] 67    = note: for more information, see issue TBD
[00:43:49] 68 
[00:43:49] - error: aborting due to 7 previous errors
---
[00:43:49] 
[00:43:49] 25     use super::bar::Bar2;
[00:43:49] 26     use crate::bar::Bar3;
[00:43:49] 27 
[00:43:49] -     use crate::bar;
[00:43:49] +     use bar;
[00:43:49] 29     //~^ ERROR absolute
[00:43:49] 30     //~| WARN this was previously accepted
[00:43:49] 31     use crate::{bar as something_else};
[00:43:49] 32 
[00:43:49] 32 
[00:43:49] -     use crate::{Bar as SomethingElse, main};
[00:43:49] +     use {Bar as SomethingElse, main};
[00:43:49] 34     //~^ ERROR absolute
[00:43:49] 35     //~| WARN this was previously accepted
[00:43:49] 
[00:43:49] 54 }
[00:43:49] 55 
[00:43:49] 55 
[00:43:49] 56 mod baz {
[00:43:49] -     use crate::*;
[00:43:49] +     use *;
[00:43:49] 58     //~^ ERROR absolute
[00:43:49] 59     //~| WARN this was previously accepted
[00:43:49] 
[00:43:49] 
[00:43:49] The actual fixed differed from the expected fixed.
[00:43:49] The actual fixed differed from the expected fixed.
[00:43:49] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/edition-lint-paths.fixed
[00:43:49] To update references, rerun the tests and pass the `--bless` flag
[00:43:49] To only update this specific test, also pass `--test-args rust-2018/edition-lint-paths.rs`
[00:43:49] error: 2 errors occurred comparing output.
[00:43:49] status: exit code: 1
[00:43:49] status: exit code: 1
[00:43:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/auxiliary" "-A" "unused"
[00:43:49] ------------------------------------------
[00:43:49] 
[00:43:49] ------------------------------------------
[00:43:49] stderr:
[00:43:49] stderr:
[00:43:49] ------------------------------------------
[00:43:49] {"message":"absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition","code":{"code":"absolute_paths_not_starting_with_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":701,"byte_end":711,"line_start":22,"line_end":22,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"    use ::bar::Bar;","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":557,"byte_end":595,"line_start":15,"line_end":15,"column_start":9,"column_end":47,"is_primary":true,"text":[{"text":"#![deny(absolute_paths_not_starting_with_crate)]","highlight_start":9,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue TBD","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `crate`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":701,"byte_end":711,"line_start":22,"line_end":22,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"    use ::bar::Bar;","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":"crate::bar::Bar","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:22:9\n   |\nLL |     use ::bar::Bar;\n   |         ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:15:9\n   |\nLL | #![deny(absolute_paths_not_starting_with_crate)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue TBD\n\n"}
[00:43:49] {"message":"absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition","code":{"code":"absolute_paths_not_starting_with_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1185,"byte_end":1193,"line_start":45,"line_end":45,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use bar::Bar;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue TBD","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `crate`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1185,"byte_end":1193,"line_start":45,"line_end":45,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use bar::Bar;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":"crate::bar::Bar","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:45:5\n   |\nLL | use bar::Bar;\n   |     ^^^^^^^^ help: use `crate`: `crate::bar::Bar`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue TBD\n\n"}
[00:43:49] {"message":"absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition","code":{"code":"absolute_paths_not_starting_with_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1473,"byte_end":1489,"line_start":62,"line_end":62,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"impl ::foo::SomeTrait for u32 { }","highlight_start":6,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue TBD","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `crate`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1473,"byte_end":1489,"line_start":62,"line_end":62,"column_start":6,"column_end":22,"is_primary":true,"text":[{"text":"impl ::foo::SomeTrait for u32 { }","highlight_start":6,"highlight_end":22}],"label":null,"suggested_replacement":"crate::foo::SomeTrait","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:62:6\n   |\nLL | impl ::foo::SomeTrait for u32 { }\n   |      ^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::SomeTrait`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue TBD\n\n"}
[00:43:49] {"message":"absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition","code":{"code":"absolute_paths_not_starting_with_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1586,"byte_end":1596,"line_start":67,"line_end":67,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let x = ::bar::Bar;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue TBD","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `crate`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-paths.rs","byte_start":1586,"byte_end":1596,"line_start":67,"line_end":67,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let x = ::bar::Bar;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"crate::bar::Bar","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:67:13\n   |\nLL |     let x = ::bar::Bar;\n   |             ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue TBD\n\n"}
[00:43:49] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:43:49] ------------------------------------------
[00:43:49] 
[00:43:49] thread '[ui] ui/rust-2018/edition-lint-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:43:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:43:49] 
[00:43:49] ---- [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs stdout ----
[00:43:49] diff of stderr:
[00:43:49] 
[00:43:49] + error[E0432]: unresolved import `edition_lint_paths`
[00:43:49] +   --> $DIR/extern-crate-idiomatic-in-2018.rs:30:9
[00:43:49] +    |
[00:43:49] + LL |     use edition_lint_paths::foo;
[00:43:49] +    |         ^^^^^^^^^^^^^^^^^^ Did you mean `self::edition_lint_paths`?
[00:43:49] 1 error: unused extern crate
[00:43:49] 2   --> $DIR/extern-crate-idiomatic-in-2018.rs:22:1
[00:43:49] 3    |
[00:43:49] 
[00:43:49] 
[00:43:49] 17 LL | extern crate edition_lint_paths as bar;
[00:43:49] 18    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[00:43:49] - error: aborting due to 2 previous errors
[00:43:49] + error: aborting due to 3 previous errors
[00:43:49] 21 
[00:43:49] + For more information about this error, try `rustc --explain E0432`.
[00:43:49] + For more information about this error, try `rustc --explain E0432`.
[00:43:49] 22 
[00:43:49] 
[00:43:49] 
[00:43:49] The actual stderr differed from the expected stderr.
[00:43:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.stderr
[00:43:49] To update references, rerun the tests and pass the `--bless` flag
[00:43:49] To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic-in-2018.rs`
[00:43:49] error: 1 errors occurred comparing output.
[00:43:49] status: exit code: 1
[00:43:49] status: exit code: 1
[00:43:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/auxiliary" "-A" "unused"
[00:43:49] ------------------------------------------
[00:43:49] 
[00:43:49] ------------------------------------------
[00:43:49] stderr:
[00:43:49] stderr:
[00:43:49] ------------------------------------------
[00:43:49] {"message":"unresolved import `edition_lint_paths`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n