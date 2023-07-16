\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs","byte_start":39,"byte_end":47,"line_start":5,"line_end":5,"column_start":5,"column_end":13,"is_primary":false,"text":[{"text":"use std::fmt;","highlight_start":5,"highlight_end":13}],"label":"not an extern crate passed with `--extern`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs","byte_start":116,"byte_end":119,"line_start":8,"line_end":8,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"use fmt::Write; //~ ERROR imports can only refer to extern crate names","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(uniform_paths)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"this import refers to the module imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs","byte_start":39,"byte_end":47,"line_start":5,"line_end":5,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::fmt;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0658]: imports can only refer to extern crate names passed with `--extern` on stable channel (see issue #53130)\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs:8:5\n   |\nLL | use std::fmt;\n   |     -------- not an extern crate passed with `--extern`\n...\nLL | use fmt::Write; //~ ERROR imports can only refer to extern crate names\n   |     ^^^\n   |\n   = help: add #![feature(uniform_paths)] to the crate attributes to enable\nnote: this import refers to the module imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs:5:5\n   |\nLL | use std::fmt;\n   |     ^^^^^^^^\n\n"}
[00:49:46] {"message":"unused import: `fmt::Write`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs","byte_start":116,"byte_end":126,"line_start":8,"line_end":8,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"use fmt::Write; //~ ERROR imports can only refer to extern crate names","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs","byte_start":25,"byte_end":31,"line_start":3,"line_end":3,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![deny(unused)]","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(unused_imports)] implied by #[deny(unused)]","code":null,"leols/compiletest/src/runtest.rs:3252:9
[00:49:46] ---- [ui] ui/span/multispan-import-lint.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 
[00:49:46] 10 LL | #![warn(unused)]
[00:49:46] 12    = note: #[warn(unused_imports)] implied by #[warn(unused)]
[00:49:46] + help: remove the unused imports
[00:49:46] +    |
[00:49:46] +    |
[00:49:46] + LL | use std::cmp::{min};
[00:49:46] 13 
[00:49:46] 14 
[00:49:46] 
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/multispan-import-lint/multispan-import-lint.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args span/multispan-import-lint.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 0
[00:49:46] status: exit code: 0
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/multispan-import-lint.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/multispan-import-lint/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/multispan-import-lint/auxiliary" "-A" "unused"
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] ------------------------------------------
[00:49:46] stderr:
[00:49:46] stderr:
[00:49:46] ------------------------------------------
[00:49:46] {"message":"unused imports: `Eq`, `Ord`, `PartialEq`, `PartialOrd`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/multispan-import-lint.rs","byte_start":517,"byte_end":519,"line_start":15,"line_end":15,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"use std::cmp::{Eq, Ord, min, PartialEq, PartialOrd};","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/multispan-import-lint.rs","byte_start":521,"byte_end":524,"line_start":15,"line_end":15,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"use std::cmp::{Eq, Ord, min, PartialEq, PartialOrd};","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/multispan-import-lint.rs","byte_start":531,"byte_end":540,"line_start":15,"line_end":15,"column_start":30,"column_end":39,"is_primary":true,"text":[{"text":"use std::cmp::{Eq, Ord, min, PartialEq, PartialOrd};","highlight_start":30,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/multispan-import-lint.rs","byte_start":542,"byte_end":552,"line_start":15,"line_end":15,"column_start":41,"column_end":51,"is_primary":true,"text":[{"text":"use std::cmp::{Eq, Or,"rendered":null}],"rendered":"warning: unused imports: `Eq`, `Ord`, `PartialEq`, `PartialOrd`\n  --> /checkout/src/test/ui/span/multispan-import-lint.rs:15:16\n   |\nLL | use std::cmp::{Eq, Ord, min, PartialEq, PartialOrd};\n   |                ^^  ^^^       ^^^^^^^^^  ^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/span/multispan-import-lint.rs:13:9\n   |\nLL | #![warn(unused)]\n   |         ^^^^^^\n   = note: #[warn(unused_imports)] implied by #[warn(unused)]\nhelp: remove the unused imports\n   |\nLL | use std::cmp::{min};\n   |               -- --\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/span/multispan-import-lint.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/use/use-nested-groups-unused-imports.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/use-nested-groups-unused-imports.rs:26:11
[00:49:46] 3    |
[00:49:46] 4 LL | use foo::{Foo, bar::{baz::{}, foobar::*}, *};
[00:49:46] -    |           ^^^        ^^^^^^^  ^^^^^^^^^   ^
[00:49:46] +    | ----------^^^--------^^^^^^^--^^^^^^^^^---^-- help: remove the whole `use` item
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/use-nested-groups-unused-imports.rs:13:9
[00:49:46] 
[00:49:46] 14   --> $DIR/use-nested-groups-unused-imports.rs:28:24
[00:49:46] 14   --> $DIR/use-nested-groups-unused-imports.rs:28:24
[00:49:46] 15    |
[00:49:46] 16 LL | use foo::bar::baz::{*, *};
[00:49:46] +    |                      --^
[00:49:46] +    |                  sed-imports/auxiliary" "-A" "unused"
[00:49:46] stdout:
[00:49:46] ------------------------------------------
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] ------------------------------------------
[00:49:46] stderr:
[00:49:46] ------------------------------------------
[00:49:46] {"message":"unused imports: `*`, `Foo`, `baz::{}`, `foobar::*`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":703,"byte_end":706,"line_start":26,"line_end":26,"column_start":11,"column_end":14,"is_primary":true,"text":[{"text":"use foo::{Foo, bar::{baz::{}, foobar::*}, *};","highlight_start":11,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":714,"byte_end":721,"line_start":26,"line_end":26,"column_start":22,"column_end":29,"is_primary":true,"text":[{"text":"use foo::{Foo, bar::{baz::{}, foobar::*}, *};","highlight_start":22,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":723,"byte_end":732,"line_start":26,"line_end":26,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"use foo::{Foo, bar::{baz::{}, foobar::*}, *};","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","est/ui/use/use-nested-groups-unused-imports.rs:13:9\n   |\nLL | #![deny(unused_imports)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:49:46] {"message":"unused import: `*`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":828,"byte_end":829,"line_start":28,"line_end":28,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"use foo::bar::baz::{*, *};","highlight_start":24,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the unused import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":826,"byte_end":829,"line_start":28,"line_end":28,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"use foo::bar::baz::{*, *};","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `*`\n  --> /checkout/src/test/ui/use/use-nested-groups-unused-imports.rs:28:24\n   |\nLL | use foo::bar::baz::{*, *};\n   |                      --^\n   |                      |\n   |                      help: remove the unused import\n\n"}
[00:49:46] {"message":"unused import: `foo::{}`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":870,"byte_end":877,"line_start":30,"line_end":30,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"use foo::{};","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/use/use-nested-groups-unused-imports.rs","byte_start":866,"byte_end":878,"line_start":30,"line_end":30,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"use foo::{};","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `foo::{}`\n  --> /checkout/src/test/ui/use/use-nested-groups-unused-imports.rs:30:5\n   |\nLL | use foo::{};\n   | ----^^^^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/use/use-nested-groups-unused-imports.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
---
35624 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
35616 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
34948 ./src/llvm/test/tools
33452 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
32756 ./src/libco|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5

