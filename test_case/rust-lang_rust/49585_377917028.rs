plain
Resolving deltas: 100% (613552/613552), completed with 4905 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:42:41] .........................................................................i..........................
[00:42:47] ................i...................................................................................
---
[00:43:23] ................................F...........................................................i.......
[00:43:30] ................................................................i...................................
---
[00:43:52] 8   "spans": [
[00:43:52] 9     {
[00:43:52] 10       "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:43:52] -       "byte_start": 1056,
[00:43:52] -       "byte_end": 1069,
[00:43:52] +       "byte_start": 1043,
[00:43:52] +       "byte_end": 1056,
[00:43:52] 13       "line_start": 25,
[00:43:52] 14       "line_end": 25,
[00:43:52] 15       "column_start": 14,
[00:43:52]
[00:43:52] 35       "spans": [
[00:43:52] 36         {
[00:43:52] 37           "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:43:52] -           "byte_start": 902,
[00:43:52] -           "byte_end": 915,
[00:43:52] +           "byte_start": 889,
[00:43:52] +           "byte_end": 902,
[00:43:52] 40           "line_start": 20,
[00:43:52] 41           "line_end": 20,
[00:43:52] 42           "column_start": 9,
[00:43:52]
[00:43:52] 64       "spans": [
[00:43:52] 65         {
[00:43:52] 66           "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:43:52] -           "byte_start": 1056,
[00:43:52] -           "byte_end": 1069,
[00:43:52] +           "byte_start": 1043,
[00:43:52] +           "byte_end": 1056,
[00:43:52] 69           "line_start": 25,
[00:43:52] 70           "line_end": 25,
[00:43:52] 71           "column_start": 14,
[00:43:52]
[00:43:52]
[00:43:52] The actual stderr differed from the expected stderr.
[00:43:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stderr
[00:43:52] To update references, run this command from build directory:
[00:43:52] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/unused_parens_json_suggestion.rs'
[00:43:52]
[00:43:52] error: 1 errors occurred comparing output.
[00:43:52] status: exit code: 0
[00:43:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:43:52]   "message": "unnecessary parentheses around assigned value",
[00:43:52]   "code": {
[00:43:52]     "code": "unused_parens",
[00:43:52]     "explanation": null
[00:43:52]   },
[00:43:52]   "level": "warning",
[00:43:52]   "spans": [
[00:43:52]     {
[00:43:52]       "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:43:52]       "byte_start": 1043,
[00:43:52]       "byte_end": 1056,
[00:43:52]       "line_start": 25,
[00:43:52]       "line_end": 25,
[00:43:52]       "column_start": 14,
[00:43:52]       "column_end": 27,
[00:43:52]       "is_primary": true,
[00:43:52]       "text": [
[00:43:52]         {
[00:43:52]           "text": "    let _a = (1 / (2 + 3));",
[00:43:52]           "highlight_start": 14,
[00:43:52]           "highlight_end": 27
[00:43:52]         }
[00:43:52]       ],
[00:43:52]       "label": null,
[00:43:52]       "suggested_replacement": null,
[00:43:52]       "expansion": null
[00:43:52]     }
[00:43:52]   ],
[00:43:52]   "children": [
[00:43:52]     {
[00:43:52]       "message": "lint level defined here",
[00:43:52]       "code": null,
[00:43:52]       "level": "note",
[00:43:52]       "spans": [
[00:43:52]         {
[00:43:52]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:43:52]           "byte_start": 889,
[00:43:52]           "byte_end": 902,
[00:43:52]           "line_start": 20,
[00:43:52]           "line_end": 20,
[00:43:52]           "column_start": 9,
[00:43:52]           "column_end": 22,
[00:43:52]           "is_primary": true,
[00:43:52]           "text": [
[00:43:52]             {
[00:43:52]               "text": "#![warn(unused_parens)]",
[00:43:52]               "highlight_start": 9,
[00:43:52]               "highlight_end": 22
[00:43:52]             }
[00:43:52]           ],
[00:43:52]           "label": null,
[00:43:52]           "suggested_replacement": null,
[00:43:52]           "expansion": null
[00:43:52]         }
[00:43:52]       ],
[00:43:52]       "children": [],
[00:43:52]       "rendered": null
[00:43:52]     },
[00:43:52]     {
[00:43:52]       "message": "remove these parentheses",
[00:43:52]       "code": null,
[00:43:52]       "level": "help",
[00:43:52]       "spans": [
[00:43:52]         {
[00:43:52]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:43:52]           "byte_start": 1043,
[00:43:52]           "byte_end": 1056,
[00:43:52]           "line_start": 25,
[00:43:52]           "line_end": 25,
[00:43:52]           "column_start": 14,
[00:43:52]           "column_end": 27,
[00:43:52]           "is_primary": true,
[00:43:52]           "text": [
[00:43:52]             {
[00:43:52]               "text": "    let _a = (1 / (2 + 3));",
[00:43:52]               "highlight_start": 14,
[00:43:52]               "highlight_end": 27
[00:43:52]             }
[00:43:52]           ],
[00:43:52]           "label": null,
[00:43:52]           "suggested_replacement": "1 / (2 + 3)",
[00:43:52]           "expansion": null
[00:43:52]         }
[00:43:52]       ],
[00:43:52]       "children": [],
[00:43:52]       "rendered": null
[00:43:52]     }
[00:43:52]   ],
[00:43:52]   "rendered": "warning: unnecessary parentheses around assigned value\n  --> /checkout/src/test/ui/lint/unused_parens_json_suggestion.rs:25:14\n   |\nLL |     let _a = (1 / (2 + 3));\n   |              ^^^^^^^^^^^^^ help: remove these parentheses\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/unused_parens_json_suggestion.rs:20:9\n   |\nLL | #![warn(unused_parens)]\n   |         ^^^^^^^^^^^^^\n\n"
[00:43:52] }
---
[00:43:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_6
