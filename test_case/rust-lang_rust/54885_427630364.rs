plain
[00:48:09] .................................................................................................... 2500/4563
[00:48:14] .................................................................................................... 2600/4563
[00:48:17] .................................................................................................... 2700/4563
[00:48:20] .................................................................................................... 2800/4563
[00:48:24] ..........................................................................F......................... 2900/4563
[00:48:30] ........................................................................................i.i..ii..... 3100/4563
[00:48:33] .................................................................................................... 3200/4563
[00:48:37] .................................................................................................... 3300/4563
[00:48:40] ..................................i................................................................. 3400/4563
---
[00:49:07] ................................................i................................................... 4200/4563
[00:49:10] .................................................................................................... 4300/4563
[00:49:13] .................................................................................................... 4400/4563
rue,
[00:49:17] +       "text": [
[00:49:17] +         {
[00:49:17] +           "text": "        if (break { return true }) {",
[00:49:17] +           "highlight_start": 12,
[00:49:17] +           "highlight_end": 35
[00:49:17] +       ],
[00:49:17] +       ],
[00:49:17] +       "label": null,
[00:49:17] +       "suggested_replacement": null,
[00:49:17] +       "suggestion_applicability": null,
[00:49:17] +       "expansion": null
[00:49:17] +   ],
[00:49:17] +   ],
[00:49:17] +   "children": [
[00:49:17] +     {
[00:49:17] +       "message": "remove these parentheses",
[00:49:17] +       "code": null,
[00:49:17] +       "level": "help",
[00:49:17] +       "spans": [
[00:49:17] +         {
[00:49:17] +           "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:49:17] +           "byte_start": 1137,
[00:49:17] +           "byte_end": 1160,
[00:49:17] +           "line_start": 32,
[00:49:17] +           "line_end": 32,
[00:49:17] +           "column_start": 12,
[00:49:17] +           "column_end": 35,
[00:49:17] +           "is_primary": true,
[00:49:17] +           "text": [
[00:49:17] +             {
[00:49:17] +               "text": "        if (break { return true }) {",
[00:49:17] +               "highlight_start": 12,
[00:49:17] +               "highlight_end": 35
[00:49:17] +           ],
[00:49:17] +           ],
[00:49:17] +           "label": null,
[00:49:17] +           "suggested_replacement": "break { return true } ",
[00:49:17] +           "suggestion_applicability": "MachineApplicable",
[00:49:17] +           "expansion": null
[00:49:17] +       ],
[00:49:17] +       ],
[00:49:17] +       "children": [],
[00:49:17] +       "rendered": null
[00:49:17] +   ],
[00:49:17] +   ],
[00:49:17] +   "rendered": "warning: unnecessary parentheses around `if` condition
[00:49:17] +   --> $DIR/unused_parens_json_suggestion.rs:32:12
[00:49:17] +    |
[00:49:17] + LL |         if (break { return true }) {
[00:49:17] 104 
[00:49:17] 105 "
[00:49:17] 106 }
[00:49:17] 
[00:49:17] 
[00:49:17] 
[00:49:17] The actual stderr differed from the expected stderr.
[00:49:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/unused_parens_json_suggestion.stderr
[00:49:17] To update references, rerun the tests and pass the `--bless` flag
[00:49:17] To only update this specific test, also pass `--test-args lint/unused_parens_json_suggestion.rs`
[00:49:17] error: 1 errors occurred comparing output.
[00:49:17] status: exit code: 0
[00:49:17] status: exit code: 0
[00:49:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/auxiliary" "-A" "unused"
[00
