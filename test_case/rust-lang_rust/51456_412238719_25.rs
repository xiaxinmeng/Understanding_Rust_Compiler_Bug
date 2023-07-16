\n"
[00:49:33]   },
[00:49:33]   "level": "error",
[00:49:33]   "spans": [
[00:49:33]     {
[00:49:33]       "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
[00:49:33]       "byte_start": 907,
[00:49:33]       "byte_end": 911,
[00:49:33]       "line_start": 21,
[00:49:33]       "line_end": 21,
[00:49:33]       "column_start": 12,
[00:49:33]       "column_end": 16,
[00:49:33]       "is_primary": true,
[00:49:33]       "text": [
[00:49:33]         {
[00:49:33]           "text": "    let x: Iter;",
[00:49:33]           "highlight_start": 12,
[00:49:33]           "highlight_end": 16
[00:49:33]       ],
[00:49:33]       ],
[00:49:33]       "label": "not found in this scope",
[00:49:33]       "suggested_replacement": null,
[00:49:33]       "suggestion_applicability": null,
[00:49:33]       "expansion": null
[00:49:33]   ],
[00:49:33]   ],
[00:49:33]   "children": [],
[00:49:33]   "rendered": "error[E0412]: cannot find type `Iter` in this scope\n  --> /checkout/src/test/ui/lint/use_suggestion_json.rs:21:12\n   |\nLL |     let x: Iter;\n   |            ^^^^ not found in this scope\n\n"
[00:49:33] }
[00:49:33] {
[00:49:33]   "message": "aborting due to previous error",
[00:49:33]   "code": null,
[00:49:33]   "level": "error",
[00:49:33]   "spans": [],
[00:49:33]   "children": [],
[00:49:33]   "rendered": "error: aborting due to previous error\n\n"
[00:49:33] }
[00:49:33] {
[00:49:33]   "message": "For more information about this error, try `rustc --explain E0412`.",
[00:49:33]   "code": null,
[00:49:33]   "level": "",
[00:49:33]   "spans": [],
[00:49:33]   "children": [],
[00:49:33]   "rendered": "For more information about this error, try `rustc --explain E0412`.\n"
[00:49:33] }
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/resolve/issue-16058.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 3    |
[00:49:33] 4 LL |         Result {
[00:49:33] - help: possible better candidates are found in other modules, you can import them into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use std::fmt::Result;
[00:49:33] -    |
[00:49:33] - LL | use std::io::Result;
[00:49:33] -    |
[00:49:33] - LL | use std::thread::Result;
[00:49:33] 14 
[00:49:33] 15 error: aborting due to previous error
[00:49:33] 16 
[00:49:33] 
[00:49:33] 
[00:49:33] 
[00:49:33] The actual stderr differed from the expected stderr.
[00:49:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-16058/issue-16058.stderr
[00:49:33] To update references, rerun the tests and pass the `--bless` flag
[00:49:33] To only update this specific test, also pass `--test-args resolve/issue-16058.rs`
[00:49:33] error: 1 errors occurred comparing output.
[00:49:33] status: exit code: 1
[00:49:33] status: exit code: 1
[00:49:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-16058.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-16058/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/te.rs","byte_start":644,"byte_end":645,"line_start":17,"line_end":17,"column_start":31,"column_end":32,"is_primary":true,"text":[{"text":"    let _ = namespaced_enums::B(10);","highlight_start":31,"highlight_end":32}],"label":"not found in `namespaced_enums`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find function `B` in module `namespaced_enums`\n  --> /checkout/src/test/ui/resolve/enums-are-namespaced-xc.rs:17:31\n   |\nLL |     let _ = namespaced_enums::B(10);\n   |                               ^ not found in `namespaced_enums`\n\n"}
[00:49:33] {"message":"cannot find struct, variant or union type `C` in module `namespaced_enums`","code":{"code":"E0422","explanation":"\nYou are trying to use an identifier that is either undefined or not a struct.\nErroneous code example:\n\n