
-	"
+	    "explanation": null
71	  },
72	  "level": "error",
73	  "spans": [

89	      ],
90	      "label": "not found in this scope",
91	      "suggested_replacement": null,
+	      "suggestion_approximate": null,
92	      "expansion": null
93	    }
94	  ],

118	          "suggested_replacement": "use std::collections::binary_heap::Iter;
119	
120	",
+	          "suggestion_approximate": null,
121	          "expansion": null
122	        },
123	        {

140	          "suggested_replacement": "use std::collections::btree_map::Iter;
141	
142	",
+	          "suggestion_approximate": null,
143	          "expansion": null
144	        },
145	        {

162	          "suggested_replacement": "use std::collections::btree_set::Iter;
163	
164	",
+	          "suggestion_approximate": null,
165	          "expansion": null
166	        },
167	        {

184	          "suggested_replacement": "use std::collections::hash_map::Iter;
185	
186	",
+	          "suggestion_approximate": null,
187	          "expansion": null
188	        },
189	        {

206	          "suggested_replacement": "use std::collections::hash_set::Iter;
207	
208	",
+	          "suggestion_approximate": null,
209	          "expansion": null
210	        },
211	        {

228	          "suggested_replacement": "use std::collections::linked_list::Iter;
229	
230	",
+	          "suggestion_approximate": null,
231	          "expansion": null
232	        },
233	        {

250	          "suggested_replacement": "use std::collections::vec_deque::Iter;
251	
252	",
+	          "suggestion_approximate": null,
253	          "expansion": null
254	        },
255	        {

272	          "suggested_replacement": "use std::option::Iter;
273	
274	",
+	          "suggestion_approximate": null,
275	          "expansion": null
276	        },
277	        {

294	          "suggested_replacement": "use std::path::Iter;
295	
296	",
+	          "suggestion_approximate": null,
297	          "expansion": null
298	        },
299	        {

316	          "suggested_replacement": "use std::result::Iter;
317	
318	",
+	          "suggestion_approximate": null,
319	          "expansion": null
320	        },
321	        {

338	          "suggested_replacement": "use std::slice::Iter;
339	
340	",
+	          "suggestion_approximate": null,
341	          "expansion": null
342	        },
343	        {

360	          "suggested_replacement": "use std::sync::mpsc::Iter;
361	
362	",
+	          "suggestion_approximate": null,
363	          "expansion": null
364	        }
365	      ],


The actual stderr differed from the expected stderr.
Actual stderr saved to /nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stderr
To update references, run this command from build directory:
/nobackup/rust/src/test/ui/update-references.sh '/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui' 'lint/use_suggestion_json.rs'

error: 1 errors occurred comparing output.
status: exit code: 101
command: "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stage1-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/nobackup/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json.stage1-x86_64-unknown-linux-gnu.aux" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{
  "message": "cannot find type `Iter` in this scope",
  "code": {
    "code": "E0412",
    "explanation": null
  },
  "level": "error",
  "spans": [
    {
      "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
      "byte_start": 907,
      "byte_end": 911,
      "line_start": 21,
      "line_end": 21,
      "column_start": 12,
      "column_end": 16,
      "is_primary": true,
      "text": [
        {
          "text": "    let x: Iter;",
          "highlight_start": 12,
          "highlight_end": 16
        }
      ],
      "label": "not found in this scope",
      "suggested_replacement": null,
      "suggestion_approximate": null,
      "expansion": null
    }
  ],
  "children": [
    {
      "message": "possible candidates are found in other modules, you can import them into scope",
      "code": null,
      "level": "help",
      "spans": [
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::binary_heap::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::btree_map::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::btree_set::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::hash_map::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::hash_set::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::linked_list::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::collections::vec_deque::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::option::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::path::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::result::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::slice::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        },
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 884,
          "byte_end": 884,
          "line_start": 20,
          "line_end": 20,
          "column_start": 1,
          "column_end": 1,
          "is_primary": true,
          "text": [
            {
              "text": "fn main() {",
              "highlight_start": 1,
              "highlight_end": 1
            }
          ],
          "label": null,
          "suggested_replacement": "use std::sync::mpsc::Iter;\n\n",
          "suggestion_approximate": null,
          "expansion": null
        }
      ],
      "children": [],
      "rendered": null
    }
  ],
  "rendered": "error[E0412]: cannot find type `Iter` in this scope\n  --> /nobackup/rust/src/test/ui/lint/use_suggestion_json.rs:21:12\n   |\n21 |     let x: Iter;\n   |            ^^^^ not found in this scope\nhelp: possible candidates are found in other modules, you can import them into scope\n   |\n20 | use std::collections::binary_heap::Iter;\n   |\n20 | use std::collections::btree_map::Iter;\n   |\n20 | use std::collections::btree_set::Iter;\n   |\n20 | use std::collections::hash_map::Iter;\n   |\nand 8 other candidates\n\n"
}
{
  "message": "aborting due to previous error",
  "code": null,
  "level": "error",
  "spans": [],
  "children": [],
  "rendered": "error: aborting due to previous error\n\n"
}

------------------------------------------

thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9

---- [ui] ui/lint/unused_parens_json_suggestion.rs stdout ----
	diff of stderr:

24	      ],
25	      "label": null,
26	      "suggested_replacement": null,
+	      "suggestion_approximate": null,
27	      "expansion": null
28	    }
29	  ],

51	          ],
52	          "label": null,
53	          "suggested_replacement": null,
+	          "suggestion_approximate": null,
54	          "expansion": null
55	        }
56	      ],

80	          ],
81	          "label": null,
82	          "suggested_replacement": "1 / (2 + 3)",
+	          "suggestion_approximate": null,
83	          "expansion": null
84	        }
85	      ],


The actual stderr differed from the expected stderr.
Actual stderr saved to /nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stderr
To update references, run this command from build directory:
/nobackup/rust/src/test/ui/update-references.sh '/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui' 'lint/unused_parens_json_suggestion.rs'

error: 1 errors occurred comparing output.
status: exit code: 0
command: "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage1-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/nobackup/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "pretty-json" "-Zunstable-options" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion.stage1-x86_64-unknown-linux-gnu.aux" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{
  "message": "unnecessary parentheses around assigned value",
  "code": {
    "code": "unused_parens",
    "explanation": null
  },
  "level": "warning",
  "spans": [
    {
      "file_name": "/nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs",
      "byte_start": 1056,
      "byte_end": 1069,
      "line_start": 25,
      "line_end": 25,
      "column_start": 14,
      "column_end": 27,
      "is_primary": true,
      "text": [
        {
          "text": "    let _a = (1 / (2 + 3));",
          "highlight_start": 14,
          "highlight_end": 27
        }
      ],
      "label": null,
      "suggested_replacement": null,
      "suggestion_approximate": null,
      "expansion": null
    }
  ],
  "children": [
    {
      "message": "lint level defined here",
      "code": null,
      "level": "note",
      "spans": [
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs",
          "byte_start": 902,
          "byte_end": 915,
          "line_start": 20,
          "line_end": 20,
          "column_start": 9,
          "column_end": 22,
          "is_primary": true,
          "text": [
            {
              "text": "#![warn(unused_parens)]",
              "highlight_start": 9,
              "highlight_end": 22
            }
          ],
          "label": null,
          "suggested_replacement": null,
          "suggestion_approximate": null,
          "expansion": null
        }
      ],
      "children": [],
      "rendered": null
    },
    {
      "message": "remove these parentheses",
      "code": null,
      "level": "help",
      "spans": [
        {
          "file_name": "/nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs",
          "byte_start": 1056,
          "byte_end": 1069,
          "line_start": 25,
          "line_end": 25,
          "column_start": 14,
          "column_end": 27,
          "is_primary": true,
          "text": [
            {
              "text": "    let _a = (1 / (2 + 3));",
              "highlight_start": 14,
              "highlight_end": 27
            }
          ],
          "label": null,
          "suggested_replacement": "1 / (2 + 3)",
          "suggestion_approximate": null,
          "expansion": null
        }
      ],
      "children": [],
      "rendered": null
    }
  ],
  "rendered": "warning: unnecessary parentheses around assigned value\n  --> /nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs:25:14\n   |\n25 |     let _a = (1 / (2 + 3));\n   |              ^^^^^^^^^^^^^ help: remove these parentheses\n   |\nnote: lint level defined here\n  --> /nobackup/rust/src/test/ui/lint/unused_parens_json_suggestion.rs:20:9\n   |\n20 | #![warn(unused_parens)]\n   |         ^^^^^^^^^^^^^\n\n"
}

------------------------------------------

thread '[ui] ui/lint/unused_parens_json_suggestion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9


failures:
    [ui] ui/explain.rs
    [ui] ui/lint/unused_parens_json_suggestion.rs
    [ui] ui/lint/use_suggestion_json.rs

test result: FAILED. 0 passed; 3 failed; 1233 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22


command did not execute successfully: "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/nobackup/rust/src/test/ui" "--build-base" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/nobackup/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/nobackup/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /nobackup/rust/build/bootstrap/debug/bootstrap test --stage 1 src/test/ui src/test/compile-fail src/test/run-pass
Build completed unsuccessfully in 0:00:06
