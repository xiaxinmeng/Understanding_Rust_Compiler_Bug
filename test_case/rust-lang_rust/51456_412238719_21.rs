\n\nAnother case that causes this error is when a type is imported into a parent\nmodule. To fix this, you can follow the suggestion and use File directly or\n`use super::File;` which will import the types from the parent namespace. An\nexample that causes this error rendered":"error[E0425]: cannot find value `LOG10_2` in module `std::f64`\n  --> /checkout/src/test/ui/issue-50599.rs:13:48\n   |\nLL |     const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value\n   |                                                ^^^^^^^ not found in `std::f64`\n\n"}
[00:49:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:33] {"message":"For more information about this error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0425`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/issue-50599.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/lint/use_suggestion_json.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 93       "expansion": null
[00:49:33] 95   ],
[00:49:33] 95   ],
[00:49:33] -   "children": [
[00:49:33] -     {
[00:49:33] -       "message": "possible candidates are found in other modules, you can import them into scope",
[00:49:33] -       "code": null,
[00:49:33] -       "level": "help",
[00:49:33] -       "spans": [
[00:49:33] -         {
[00:49:33] -           "file_name": "$DIR/use_suggestion_json.rs",
[00:49:33] -           "byte_start": 884,
[00:49:33] -           "byte_end": 884,
[00:49:33] -           "line_start": 20,
[00:49:33] -           "line_end": 20,
[00:49:33] -           "column_start": 1,
[00:49:33] -           "column_end": 1,
[00:49:33] -           "is_primary": true,
[00:49:33] -           "text": [
[00:49:33] -             {
[00:49:33] -               "text": "fn main() {",
[00:49:33] -               "highlight_start": 1,
[00:49:33] -               "highlight_end": 1
[00:49:33] -           ],
[00:49:33] -           ],
[00:49:33] -           "label": null,
[00:49:33] -           "suggested_replacement": "use std::collections::binary_heap::Iter;
[00:49:33] - ",
[00:49:33] - ",
[00:49:33] -           "suggestion_applicability": "Unspecified",
[00:49:33] -           "expansion": null
[00:49:33] -         {
[00:49:33] -         {
[00:49:33] -           "file_name": "$DIR/use_suggestion_json.rs",
[00:49:33] -           "byte_start": 884,
[00:49:33] -           "byte_end": 884,
[00:49:33] -           "line_start": 20,
[00:49:33] -           "line_end": 20,
[00:49:33] -           "column_start": 1,
[00:49:33] -           "column_end": 1,
[00:49:33] -           "is_primary": true,
[00:49:33] -           "text": [
[00:49:33] -             {
[00:49:33] -               "text": "fn main() {",
[00:49:33] -               "highlight_start": 1,
[00:49:33] -               "highlight_end": 1
[00:49:33] -           ],
[00:49:33] -           ],
[00:49:33] -           "label": null,
[00:49:33] -           "suggested_replacement": "use std::collections::btree_map::Iter;
[00:49:33] - ",
[00:49:33] - ",
[00:49:33] -           "suggestion_applicability": "Unspecified",
[00:49:33] -           "expansion": null
[00:49:33] -         {
[00:49:33] -         {
[00:49:33] -                 "highlight_start": 1,
[00:49:33] -               "highlight_end": 1
[00:49:33] -           ],
[00:49:33] -           ],
[00:49:33] -           "label": null,
[00:49:33] -           "suggested_replacement": "use std::collections::linked_list::Iter;
[00:49:33] - ",
[00:49:33] - ",
[00:49:33] -           "suggestion_applicability": "Unspecified",
[00:49:33] -           "expansion": null
[00:49:33] -         {
[00:49:33] -         {
[00:49:33] -           "file_name": "$DIR/use_suggestion_json.rs",
[00:49:33] -           "byte_start": 884,
[00:49:33] -           "byte_end": 884,
[00:49:33] -           "line_start": 20,
[00:49:33] -           "line_end": 20,
[00:49:33] -           "column_start": 1,
[00:49:33] -           "column_end": 1,
[00:49:33] -           "is_primary": true,
[00:49:33] -           "text": [
[00:49:33] -             {
[00:49:33] -               "text": "fn main() {",
[00:49:33] -               "highlight_start": 1,
[00:49:33] -               "highlight_end": 1
[00:49:33] -           ],
[00:49:33] -           ],
[00:49:33] -           "label": null,
[00:49:33] -           "suggested_replacement": "use std::collections::vec_deque::Iter;
[00:49:33] - ",
[00:49:33] - ",
[00:49:33] -           "suggestion_applicability": "Unspecified",
[00:49:33] -           "expansion": null
[00:49:33] -         {
[00:49:33] -         {
[00:49:33] -           "file_name": "$DIR/use_suggestion_json.rs",
[00:49:33] -           "byte_start": 884,
[00:49:33] -           "byte_end": 884,
[00:49:33] -           "line_start": 20,
[00:49:33] -           "line_end": 20,
[00:49:33] {
[00:49:33] {
[00:49:33]   "message": "cannot find type `Iter` in this scope",
[00:49:33]   "code": {
[00:49:33]     "code": "E0412",
[00:49:33]     "explanation": "\nThe type name used is not in scope.\n\nErroneous code examples:\n\n