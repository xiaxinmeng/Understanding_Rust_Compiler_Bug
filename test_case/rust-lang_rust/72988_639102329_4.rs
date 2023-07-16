\n"
  "level": "error",
  "spans": [
    {
      "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
---
      "column_end": 16,
      "is_primary": true,
      "text": [
        {
          "text": "    let x: Iter;",
          "highlight_start": 12,
          "highlight_end": 16
      ],
      "label": "not found in this scope",
      "suggested_replacement": null,
      "suggestion_applicability": null,
      "suggestion_applicability": null,
      "expansion": null
    }
  ],
  "children": [
    {
      "message": "did you mean one of these?:",
      "code": null,
      "level": "help",
      "spans": [
          "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 560,
          "byte_end": 564,
          "line_start": 13,
          "line_start": 13,
          "line_end": 13,
          "column_start": 12,
          "column_end": 16,
          "is_primary": true,
          "text": [
            {
              "text": "    let x: Iter;",
              "highlight_start": 12,
              "highlight_end": 16
          ],
          "label": null,
          "suggested_replacement": "std::io::BufWriter",
          "suggestion_applicability": "MaybeIncorrect",
---
          "column_end": 16,
          "is_primary": true,
          "text": [
            {
              "text": "    let x: Iter;",
              "highlight_start": 12,
              "highlight_end": 16
          ],
          "label": null,
          "suggested_replacement": "std::io::LineWriter",
          "suggestion_applicability": "MaybeIncorrect",
---
    },
    {
      "message": "consider importing one of these items",
      "code": null,
      "level": "help",
      "spans": [
          "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
          "byte_start": 537,
          "byte_end": 537,
          "line_start": 12,
---
      "children": [],
      "rendered": null
    }
  ],
  "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m/checkout/src/test/ui/lint/use_suggestion_json.rs:13:12\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: did you mean one of these?:\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: std::io::BufWriter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;14m^^^^^^^^^^^^^^^^^^\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: std::io::LineWriter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;14m^^^^^^^^^^^^^^^^^^^\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: consider importing one of these items\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_set::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::hash_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m     and 8 other candidates\u001b[0m\n\n"
{
  "message": "aborting due to previous error",
  "code": null,
  "level": "error",
  "level": "error",
  "spans": [],
  "children": [],
  "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m\n\n"
{
  "message": "For more information about this error, try `rustc --explain E0412`.",
  "code": null,
  "level": "failure-note",
  "level": "failure-note",
  "spans": [],
  "children": [],
  "rendered": "\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m\n"
thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/no-implicit-prelude-nested.rs stdout ----
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude-nested/no-implicit-prelude-nested.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args no-implicit-prelude-nested.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-implicit-prelude-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude-nested" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0405]: cannot find trait `ToString` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:14:14
   |
LL |         impl ToString for Test {} //~ ERROR cannot find trait `ToString` in this scope
   |
help: consider importing one of these items
   |
LL |         use std::prelude::v1::ToString;
LL |         use std::prelude::v1::ToString;
   |
LL |         use std::string::ToString;
   |

error[E0405]: cannot find trait `Writer` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:15:14
   |
LL |         impl Writer for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |
help: did you mean one of these?:
   |
   |
LL |         impl std::io::BufWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |              ^^^^^^^^^^^^^^^^^^
LL |         impl std::io::LineWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope

error[E0425]: cannot find function `drop` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:18:13
   |
---

error[E0405]: cannot find trait `ToString` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:26:10
   |
LL |     impl ToString for Test {} //~ ERROR cannot find trait `ToString` in this scope
   |
help: consider importing one of these items
   |
LL |     use std::prelude::v1::ToString;
LL |     use std::prelude::v1::ToString;
   |
LL |     use std::string::ToString;
   |

error[E0405]: cannot find trait `Writer` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:27:10
   |
LL |     impl Writer for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |
help: did you mean one of these?:
   |
   |
LL |     impl std::io::BufWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |          ^^^^^^^^^^^^^^^^^^
LL |     impl std::io::LineWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope

error[E0425]: cannot find function `drop` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:30:9
   |
---

error[E0405]: cannot find trait `ToString` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:41:14
   |
LL |         impl ToString for Test {} //~ ERROR cannot find trait `ToString` in this scope
   |
help: consider importing one of these items
   |
LL |         use std::prelude::v1::ToString;
LL |         use std::prelude::v1::ToString;
   |
LL |         use std::string::ToString;
   |

error[E0405]: cannot find trait `Writer` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:42:14
   |
LL |         impl Writer for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |
help: did you mean one of these?:
   |
   |
LL |         impl std::io::BufWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |              ^^^^^^^^^^^^^^^^^^
LL |         impl std::io::LineWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope

error[E0425]: cannot find function `drop` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude-nested.rs:45:13
   |
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude/no-implicit-prelude.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args no-implicit-prelude.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-implicit-prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-implicit-prelude/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0405]: cannot find trait `ToString` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude.rs:13:6
   |
LL | impl ToString for Test {} //~ ERROR cannot find trait `ToString` in this scope
   |
help: consider importing one of these items
   |
LL | use std::prelude::v1::ToString;
LL | use std::prelude::v1::ToString;
   |
LL | use std::string::ToString;
   |

error[E0405]: cannot find trait `Writer` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude.rs:14:6
   |
LL | impl Writer for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |
help: did you mean one of these?:
   |
   |
LL | impl std::io::BufWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope
   |      ^^^^^^^^^^^^^^^^^^
LL | impl std::io::LineWriter for Test {} //~ ERROR cannot find trait `Writer` in this scope

error[E0425]: cannot find function `drop` in this scope
  --> /checkout/src/test/ui/no-implicit-prelude.rs:17:5
   |
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-16058/issue-16058.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-16058.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-16058.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-16058" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-16058/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-assoc-suggestions/resolve-assoc-suggestions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/resolve-assoc-suggestions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-assoc-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-assoc-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-assoc-suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
5    |                       ^^^ not a value
+    |
+ help: did you mean `std::num::NonZeroU16`?
+    |
+ LL |     std::mem::size_of(std::num::NonZeroU16);
6 
7 error[E0412]: cannot find type `u8` in the crate root
8   --> $DIR/resolve-primitive-fallback.rs:8:14

---
15 LL | use std::primitive::u8;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback/resolve-primitive-fallback.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/resolve-primitive-fallback.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-primitive-fallback.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-primitive-fallback/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
24    |             ^^^^^^^ not found in this scope
25    |
+ help: did you mean `std::collections::HashMap`?
+    |
+ LL |     let y1: std::collections::HashMap;
26 help: consider importing one of these items
27    |
28 LL | use std::collections::HashMap;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion/use_suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/use_suggestion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/use_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type or module `GooMap`
   |
   |
LL |     let x2 = GooMap::new(); //~ ERROR failed to resolve
   |              ^^^^^^ use of undeclared type or module `GooMap`
error[E0433]: failed to resolve: use of undeclared type or module `HashMap`
  --> /checkout/src/test/ui/resolve/use_suggestion.rs:2:14
   |
LL |     let x1 = HashMap::new(); //~ ERROR failed to resolve
---

error[E0412]: cannot find type `HashMap` in this scope
  --> /checkout/src/test/ui/resolve/use_suggestion.rs:5:13
   |
LL |     let y1: HashMap; //~ ERROR cannot find type
   |
help: did you mean `std::collections::HashMap`?
   |
   |
LL |     let y1: std::collections::HashMap; //~ ERROR cannot find type
help: consider importing one of these items
   |
LL | use std::collections::HashMap;
   |
   |
LL | use std::collections::hash_map::HashMap;
   |

error[E0412]: cannot find type `GooMap` in this scope
   |
   |
LL |     let y2: GooMap; //~ ERROR cannot find type

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0412, E0433.
---

---- [ui] ui/resolve/use_suggestion_placement.rs stdout ----
diff of stderr:

26 LL |     type Dict<K, V> = HashMap<K, V>;
28    |
+ help: did you mean `std::collections::HashMap`?
+    |
+    |
+ LL |     type Dict<K, V> = std::collections::HashMap<K, V>;
29 help: consider importing one of these items
30    |
31 LL | use std::collections::HashMap;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement/use_suggestion_placement.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/use_suggestion_placement.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/use_suggestion_placement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/use_suggestion_placement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0412]: cannot find type `HashMap` in this scope
  --> /checkout/src/test/ui/resolve/use_suggestion_placement.rs:27:23
   |
LL |     type Dict<K, V> = HashMap<K, V>; //~ ERROR cannot find
   |
help: did you mean `std::collections::HashMap`?
   |
   |
LL |     type Dict<K, V> = std::collections::HashMap<K, V>; //~ ERROR cannot find
help: consider importing one of these items
   |
LL | use std::collections::HashMap;
   |
---

---- [ui] ui/rust-2018/issue-52202-use-suggestions.rs stdout ----
diff of stderr:

4 LL |     let _d = Drain {};
6    |
+ help: did you mean `std::string::Drain`?
+    |
+    |
+ LL |     let _d = std::string::Drain {};
7 help: consider importing one of these items
8    |
9 LL | use crate::plumbing::Drain;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions/issue-52202-use-suggestions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/issue-52202-use-suggestions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0422]: cannot find struct, variant or union type `Drain` in this scope
  --> /checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs:11:14
   |
LL |     let _d = Drain {};
   |
help: did you mean `std::string::Drain`?
   |
   |
LL |     let _d = std::string::Drain {};
help: consider importing one of these items
   |
LL | use crate::plumbing::Drain;
   |
---
---- [ui] ui/tool-attributes/tool-attributes-misplaced-1.rs stdout ----
diff of stderr:

33    |
34 LL | type B = rustfmt::skip;
+    |
+ help: did you mean `std::iter::Skip`?
+    |
+    |
+ LL | type B = rustfmt::std::iter::Skip;
36 
37 error[E0423]: expected value, found tool module `rustfmt`
38   --> $DIR/tool-attributes-misplaced-1.rs:14:5

---
50 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-1/tool-attributes-misplaced-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args tool-attributes/tool-attributes-misplaced-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find derive macro `rustfmt` in this scope
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:4:10
   |
LL | #[derive(rustfmt)] //~ ERROR cannot find derive macro `rustfmt` in this scope

error: cannot find derive macro `rustfmt` in this scope
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:4:10
   |
   |
LL | #[derive(rustfmt)] //~ ERROR cannot find derive macro `rustfmt` in this scope

error: cannot find attribute `rustfmt` in this scope
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:9:3
   |
   |
LL | #[rustfmt] //~ ERROR cannot find attribute `rustfmt` in this scope

error: cannot find macro `rustfmt` in this scope
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:15:5
   |
   |
LL |     rustfmt!(); //~ ERROR cannot find macro `rustfmt` in this scope
   |     ^^^^^^^

error[E0573]: expected type, found tool module `rustfmt`
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:1:10
   |
LL | type A = rustfmt; //~ ERROR expected type, found tool module `rustfmt`

error[E0573]: expected type, found tool attribute `rustfmt::skip`
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:2:10
   |
   |
LL | type B = rustfmt::skip; //~ ERROR expected type, found tool attribute `rustfmt::skip`
   |
help: did you mean `std::iter::Skip`?
   |
   |
LL | type B = rustfmt::std::iter::Skip; //~ ERROR expected type, found tool attribute `rustfmt::skip`

error[E0423]: expected value, found tool module `rustfmt`
  --> /checkout/src/test/ui/tool-attributes/tool-attributes-misplaced-1.rs:14:5
   |
---

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:59:23
Build completed unsuccessfully in 0:59:23
== clock drift check ==
  local time: Thu Jun  4 20:35:32 UTC 2020
  network time: Thu, 04 Jun 2020 20:35:33 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72988/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72988/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (5359) (python)
##[section]Finishing: Finalize Job
