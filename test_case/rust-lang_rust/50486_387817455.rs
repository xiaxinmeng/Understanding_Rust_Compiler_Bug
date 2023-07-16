plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Running apt-get update by default has been disabled.
You can opt into running apt-get update by setting this in your .travis.yml file:
  apt:
travis_fold:start:git.checkout
travis_time:start:00375fb5
$ git clone --depth=2 https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:45:25] ....................................................................................................
[00:45:29] ....................................................................................................
[00:45:35] ....................................................................................................
[00:45:40] ....................................................................................................
[00:45:46] ...............................................................................FF...................
[00:45:57] ..................i.................................................................................
[00:46:03] .....................................ii.............................................................
[00:46:10] ....................................................................................................
[00:46:16] .................i....................................................................
[00:46:16] .................i....................................................................
[00:46:16] failures:
[00:46:16] 
[00:46:16] ---- [ui] ui/lint/unused_parens_json_suggestion.rs stdout ----
[00:46:16]  diff of stderr:
[00:46:16] 
[00:46:16] 24       ],
[00:46:16] 25       "label": null,
[00:46:16] 26       "suggested_replacement": null,
[00:46:16] +       "suggestion_applicability": null,
[00:46:16] 27       "expansion": null
[00:46:16] 29   ],
[00:46:16] 
[00:46:16] 51           ],
[00:46:16] 51           ],
[00:46:16] 52           "label": null,
[00:46:16] 53           "suggested_replacement": null,
[00:46:16] +           "suggestion_applicability": null,
[00:46:16] 54           "expansion": null
[00:46:16] 56       ],
[00:46:16] 
[00:46:16] 80           ],
[00:46:16] 80           ],
[00:46:16] 81           "label": null,
[00:46:16] 82           "suggested_replacement": "1 / (2 + 3)",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 83           "expansion": null
[00:46:16] 85       ],
[00:46:16] 
[00:46:16] 
[00:46:16] The actual stderr differed from the expected stderr.
[00:46:16] The actual stderr differed from the expected stderr.
[00:46:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stderr
[00:46:16] To update references, run this command from build directory:
[00:46:16] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/unused_parens_json_suggestion.rs'
[00:46:16] error: 1 errors occurred comparing output.
[00:46:16] status: exit code: 0
[00:46:16] status: exit code: 0
[00:46:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:46:16] ------------------------------------------
[00:46:16] 
[00:46:16] ------------------------------------------
[00:46:16] stderr:
[00:46:16] stderr:
[00:46:16] ------------------------------------------
[00:46:16] {
[00:46:16]   "message": "unnecessary parentheses around assigned value",
[00:46:16]   "code": {
[00:46:16]     "code": "unused_parens",
[00:46:16]     "explanation": null
[00:46:16]   },
[00:46:16]   "level": "warning",
[00:46:16]   "spans": [
[00:46:16]     {
[00:46:16]       "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:46:16]       "byte_start": 1043,
[00:46:16]       "byte_end": 1056,
[00:46:16]       "line_start": 25,
[00:46:16]       "line_end": 25,
[00:46:16]       "column_start": 14,
[00:46:16]       "column_end": 27,
[00:46:16]       "is_primary": true,
[00:46:16]       "text": [
[00:46:16]         {
[00:46:16]           "text": "    let _a = (1 / (2 + 3));",
[00:46:16]           "highlight_start": 14,
[00:46:16]           "highlight_end": 27
[00:46:16]       ],
[00:46:16]       ],
[00:46:16]       "label": null,
[00:46:16]       "suggested_replacement": null,
[00:46:16]       "suggestion_applicability": null,
[00:46:16]       "expansion": null
[00:46:16]   ],
[00:46:16]   ],
[00:46:16]   "children": [
[00:46:16]     {
[00:46:16]       "message": "lint level defined here",
[00:46:16]       "code": null,
[00:46:16]       "level": "note",
[00:46:16]       "spans": [
[00:46:16]         {
[00:46:16]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:46:16]           "byte_start": 889,
[00:46:16]           "byte_end": 902,
[00:46:16]           "line_start": 20,
[00:46:16]           "line_end": 20,
[00:46:16]           "column_start": 9,
[00:46:16]           "column_end": 22,
[00:46:16]           "is_primary": true,
[00:46:16]           "text": [
[00:46:16]             {
[00:46:16]               "text": "#![warn(unused_parens)]",
[00:46:16]               "highlight_start": 9,
[00:46:16]               "highlight_end": 22
[00:46:16]           ],
[00:46:16]           ],
[00:46:16]           "label": null,
[00:46:16]           "suggested_replacement": null,
[00:46:16]           "suggestion_applicability": null,
[00:46:16]           "expansion": null
[00:46:16]       ],
[00:46:16]       ],
[00:46:16]       "children": [],
[00:46:16]       "rendered": null
[00:46:16]     {
[00:46:16]     {
[00:46:16]       "message": "remove these parentheses",
[00:46:16]       "code": null,
[00:46:16]       "level": "help",
[00:46:16]       "spans": [
[00:46:16]         {
[00:46:16]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:46:16]           "byte_start": 1043,
[00:46:16]           "byte_end": 1056,
[00:46:16]           "line_start": 25,
[00:46:16]           "line_end": 25,
[00:46:16]           "column_start": 14,
[00:46:16]           "column_end": 27,
[00:46:16]           "is_primary": true,
[00:46:16]           "text": [
[00:46:16]             {
[00:46:16]               "text": "    let _a = (1 / (2 + 3));",
[00:46:16]               "highlight_start": 14,
[00:46:16]               "highlight_end": 27
[00:46:16]           ],
[00:46:16]           ],
[00:46:16]           "label": null,
[00:46:16]           "suggested_replacement": "1 / (2 + 3)",
[00:46:16]           "suggestion_applicability": "Unspecified",
[00:46:16]           "expansion": null
[00:46:16]       ],
[00:46:16]       ],
[00:46:16]       "children": [],
[00:46:16]       "rendered": null
[00:46:16]   ],
[00:46:16]   ],
[00:46:16]   "rendered": "warning: unnecessary parentheses around assigned value\n  --> /checkout/src/test/ui/lint/unused_parens_json_suggestion.rs:25:14\n   |\nLL |     let _a = (1 / (2 + 3));\n   |              ^^^^^^^^^^^^^ help: remove these parentheses\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/unused_parens_json_suggestion.rs:20:9\n   |\nLL | #![warn(unused_parens)]\n   |         ^^^^^^^^^^^^^\n\n"
[00:46:16] }
[00:46:16] ------------------------------------------
[00:46:16] 
[00:46:16] thread '[ui] ui/lint/unused_parens_json_suggestion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:46:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:16] 
[00:46:16] ---- [ui] ui/lint/use_suggestion_json.rs stdout ----
[00:46:16]  diff of stderr:
[00:46:16] 
[00:46:16] 89       ],
[00:46:16] 90       "label": "not found in this scope",
[00:46:16] 91       "suggested_replacement": null,
[00:46:16] +       "suggestion_applicability": null,
[00:46:16] 92       "expansion": null
[00:46:16] 94   ],
[00:46:16] 
[00:46:16] 
[00:46:16] 118           "suggested_replacement": "use std::collections::binary_heap::Iter;
[00:46:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:46:16] 120 ",
[00:46:16] 120 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 121           "expansion": null
[00:46:16] 123         {
[00:46:16] 
[00:46:16] 
[00:46:16] 140           "suggested_replacement": "use std::collections::btree_map::Iter;
[00:46:16] 142 ",
[00:46:16] 142 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 143           "expansion": null
[00:46:16] 145         {
[00:46:16] 
[00:46:16] 
[00:46:16] 162           "suggested_replacement": "use std::collections::btree_set::Iter;
[00:46:16] 164 ",
[00:46:16] 164 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 165           "expansion": null
[00:46:16] 167         {
[00:46:16] 
[00:46:16] 
[00:46:16] 184           "suggested_replacement": "use std::collections::hash_map::Iter;
[00:46:16] 186 ",
[00:46:16] 186 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 187           "expansion": null
[00:46:16] 189         {
[00:46:16] 
[00:46:16] 
[00:46:16] 206           "suggested_replacement": "use std::collections::hash_set::Iter;
[00:46:16] 208 ",
[00:46:16] 208 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 209           "expansion": null
[00:46:16] 211         {
[00:46:16] 
[00:46:16] 
[00:46:16] 228           "suggested_replacement": "use std::collections::linked_list::Iter;
[00:46:16] 230 ",
[00:46:16] 230 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 231           "expansion": null
[00:46:16] 233         {
[00:46:16] 
[00:46:16] 
[00:46:16] 250           "suggested_replacement": "use std::collections::vec_deque::Iter;
[00:46:16] 252 ",
[00:46:16] 252 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 253           "expansion": null
[00:46:16] 255         {
[00:46:16] 
[00:46:16] 
[00:46:16] 272           "suggested_replacement": "use std::option::Iter;
[00:46:16] 274 ",
[00:46:16] 274 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 275           "expansion": null
[00:46:16] 277         {
[00:46:16] 
[00:46:16] 
[00:46:16] 294           "suggested_replacement": "use std::path::Iter;
[00:46:16] 296 ",
[00:46:16] 296 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 297           "expansion": null
[00:46:16] 299         {
[00:46:16] 
[00:46:16] 
[00:46:16] 316           "suggested_replacement": "use std::result::Iter;
[00:46:16] 318 ",
[00:46:16] 318 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 319           "expansion": null
[00:46:16] 321         {
[00:46:16] 
[00:46:16] 
[00:46:16] 338           "suggested_replacement": "use std::slice::Iter;
[00:46:16] 340 ",
[00:46:16] 340 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 341           "expansion": null
[00:46:16] 343         {
[00:46:16] 
[00:46:16] 
[00:46:16] 360           "suggested_replacement": "use std::sync::mpsc::Iter;
[00:46:16] 362 ",
[00:46:16] 362 ",
[00:46:16] +           "suggestion_applicability": "Unspecified",
[00:46:16] 363           "expansion": null
[00:46:16] 365       ],
[00:46:16] 
[00:46:16] 
[00:46:16] The actual stderr differed from the expected stderr.
[00:46:16] The actual stderr differed from the expected stderr.
[00:46:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stderr
[00:46:16] To update references, run this command from build directory:
[00:46:16] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/use_suggestion_json.rs'
[00:46:16] error: 1 errors occurred comparing output.
[00:46:16] status: exit code: 101
[00:46:16] status: exit code: 101
[00:46:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/use_suggestion_json.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:46:16] ------------------------------------------
[00:46:16] 
[00:46:16] ------------------------------------------
[00:46:16] stderr:
[00:46:16] stderr:
[00:46:16] ------------------------------------------
[00:46:16] {
[00:46:16]   "message": "cannot find type `Iter` in this scope",
[00:46:16]   "code": {
[00:46:16]     "code": "E0412",
[00:46:16]     "explanation": "\nThe type name used is not in scope.\n\nErroneous code examples:\n\n