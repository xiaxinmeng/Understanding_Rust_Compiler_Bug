plain
.................................................................................................... 8100/12099
............................................................................i....................... 8200/12099
.........i..........................................F.................i............................. 8300/12099
.................................................................................................... 8400/12099
..........i...............................................................F............F............ 8500/12099
................................................................F...............i....F.............. 8600/12099
..............................FF.................................................................... 8700/12099
.................................................................................................... 8900/12099
.................................................................................................... 9000/12099
...............................................................................iiii.iiiii........... 9100/12099
......................................................ii...............i............................ 9200/12099
---

---- [ui] ui/did_you_mean/issue-40396.rs stdout ----
diff of stderr:

31 LL |     (0..13).collect::<Vec<i32>();
33 
33 
- error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `,`
+ error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
36    |
37 LL |     let x = std::collections::HashMap<i128, i128>::new();

-    |                                           ^ expected one of 7 possible tokens
-    |                                           ^ expected one of 7 possible tokens
+    |                                           ^ expected one of 8 possible tokens
39    |
40 help: use `::<...>` instead of `<...>` to specify type or const arguments
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/issue-40396.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/issue-40396.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-40396.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40396.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:2:20
   |
LL |     (0..13).collect<Vec<i32>>();
   |                    ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     (0..13).collect::<Vec<i32>>();

error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:5:8
   |
   |
LL |     Vec<i32>::new();
   |        ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
LL |     Vec::<i32>::new();
   |        ^^

error: comparison operators cannot be chained
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:8:20
   |
LL |     (0..13).collect<Vec<i32>();
   |                    ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     (0..13).collect::<Vec<i32>();


error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:11:43
   |
LL |     let x = std::collections::HashMap<i128, i128>::new(); //~ ERROR expected one of
   |                                           ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     let x = std::collections::HashMap::<i128, i128>::new(); //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:15:39
   |
LL |         std::collections::HashMap<i128, i128>::new() //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |         std::collections::HashMap::<i128, i128>::new() //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:20:39
   |
LL |         std::collections::HashMap<i128, i128>::new(); //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |         std::collections::HashMap::<i128, i128>::new(); //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:25:39
   |
LL |         std::collections::HashMap<i128, i128>::new(1, 2); //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |         std::collections::HashMap::<i128, i128>::new(1, 2); //~ ERROR expected one of

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:13:17
   |
   |
LL |     let x: () = 42; //~ ERROR mismatched types
   |            --   ^^ expected `()`, found integer
   |            expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:18:17
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:18:17
   |
LL |     let x: () = 42; //~ ERROR mismatched types
   |            --   ^^ expected `()`, found integer
   |            expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:22:21
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:22:21
   |
LL |         let x: () = 42; //~ ERROR mismatched types
   |                --   ^^ expected `()`, found integer
   |                expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:27:21
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:27:21
   |
LL |         let x: () = 32; //~ ERROR mismatched types
   |                --   ^^ expected `()`, found integer
   |                expected due to this

error: aborting due to 11 previous errors

---

---- [ui] ui/parser/attr-stmt-expr-attr-bad.rs stdout ----
diff of stderr:

12 LL | #[cfg(FALSE)] fn e() { let _ = [#[attr]]; }
13    |                                        ^ expected expression
14 
- error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
+ error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
17    |
17    |
18 LL | #[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
-    |                                   ^ expected one of 7 possible tokens
+    |                                   ^ expected one of 8 possible tokens
20 
21 error: an inner attribute is not permitted in this context
21 error: an inner attribute is not permitted in this context
22   --> $DIR/attr-stmt-expr-attr-bad.rs:11:36

70    |
71    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
72 
- error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
+ error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
75    |
75    |
76 LL | #[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
-    |                                  ^ expected one of 7 possible tokens
+    |                                  ^ expected one of 8 possible tokens
78 
79 error: an inner attribute is not permitted in this context
79 error: an inner attribute is not permitted in this context
80   --> $DIR/attr-stmt-expr-attr-bad.rs:25:35

372 LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
374 
374 
- error: expected one of `.`, `;`, `?`, or an operator, found `#`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
377    |
377    |
378 LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }

-    |                                  ^ expected one of `.`, `;`, `?`, or an operator
+    |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
381 error: unexpected token: `#`
382   --> $DIR/attr-stmt-expr-attr-bad.rs:103:34


384 LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
386 
386 
- error: expected one of `.`, `;`, `?`, or an operator, found `#`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
389    |
389    |
390 LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }

-    |                                  ^ expected one of `.`, `;`, `?`, or an operator
+    |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
393 error: expected statement after outer attribute
394   --> $DIR/attr-stmt-expr-attr-bad.rs:108:37



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/attr-stmt-expr-attr-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/attr-stmt-expr-attr-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:5:36
   |
LL | #[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: expected expression, found `]`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:7:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = [#[attr]]; }
   |                                        ^ expected expression

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
   |                                   ^ expected one of 8 possible tokens
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:36
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: expected expression, found `)`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:44
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
   |                                            ^ expected expression
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:38
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: expected expression, found `)`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:46
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
   |                                              ^ expected expression
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:17:36
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = 0 + #![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:19:33
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = !#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:21:33
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = -#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
   |                                  ^ expected one of 8 possible tokens
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:25:35
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] foo; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:27:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] foo; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:29:35
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] {foo}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:31:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] {foo}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: expected expression, found `..`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:33:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..#[attr] 0; }

error: expected expression, found `..`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:35:40
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..; }

error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:37:41
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &#![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:39:45
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &mut #![attr] 0; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 #[attr] {}; }
   |                                --   ^^^^^^^ -- the attributes are attached to this branch
   |                                |    help: remove the attributes
   |                                the branch belongs to this `if`

error: an inner attribute is not permitted in this context
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:43:38
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} #[attr] else {}; }
   |                                        ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] {}; }
   |                                        ---- ^^^^^^^ -- the attributes are attached to this branch
   |                                        |    help: remove the attributes
   |                                        |    help: remove the attributes
   |                                        the branch belongs to this `else`
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:49:46
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
   |                                        ---- ^^^^^^^ ------- the attributes are attached to this branch
   |                                        |    help: remove the attributes
   |                                        |    help: remove the attributes
   |                                        the branch belongs to this `else`

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 #[attr] {}; }
   |                                             --   ^^^^^^^ -- the attributes are attached to this branch
   |                                             |    help: remove the attributes
   |                                             the branch belongs to this `if`

error: an inner attribute is not permitted in this context
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:55:51
   |
LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 #[attr] {}; }
   |                                |            |
   |                                |            help: remove the attributes
   |                                the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:59:46
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} #[attr] else {}; }
   |                                                ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] {}; }
   |                                                ---- ^^^^^^^ -- the attributes are attached to this branch
   |                                                |    help: remove the attributes
   |                                                |    help: remove the attributes
   |                                                the branch belongs to this `else`
error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:65:54
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
   |                                                ---- ^^^^^^^ --------------- the attributes are attached to this branch
   |                                                |    help: remove the attributes
   |                                                |    help: remove the attributes
   |                                                the branch belongs to this `else`

error: outer attributes are not allowed on `if` and `else` branches
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 #[attr] {}; }
   |                                                     |            |
   |                                                     |            help: remove the attributes
   |                                                     the branch belongs to this `if`


error: an inner attribute is not permitted in this context
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:71:67
   |
LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 {#![attr]}; }
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted following an outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:74:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted following an outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:76:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted following an outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:78:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted following an outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:80:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted following an outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:82:32
   |
   |
LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
   |                        ------- ^^^^^^^^ not permitted following an outer attribute
   |                        previous outer attribute
   |
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:88:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
   |                                   ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:91:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
   |                                   ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error: unexpected token: `#`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:94:39
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=-#[attr] 10 => () } }

error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:96:35
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
   |                                   ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
   |                                      ^ expected one of `=>`, `if`, or `|`
error: unexpected token: `#`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:100:34
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }


error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
   |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: unexpected token: `#`
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:103:34
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }


error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
   |
   |
LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
   |                                  ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: expected statement after outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:108:37
   |
   |
LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr]; } } }

error: expected statement after outer attribute
  --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:110:37
   |
   |
LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr] } } }

error: aborting due to 53 previous errors

For more information about this error, try `rustc --explain E0586`.
For more information about this error, try `rustc --explain E0586`.

------------------------------------------


---- [ui] ui/parser/issue-72253.rs stdout ----
diff of stderr:

- error: expected one of `.`, `;`, `?`, or an operator, found `,`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `,`
3    |
3    |
4 LL |         .arg("1")

-    |                  - expected one of `.`, `;`, `?`, or an operator
---
To only update this specific test, also pass `--test-args parser/issue-84117.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-84117.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-84117" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-84117/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `>`, a const expression, lifetime, or type, found `}`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |                                         ------------              ^ expected one of `>`, a const expression, lifetime, or type
   |                                         |          |
   |                                         |          help: use `=` if you meant to assign
   |                                         while parsing the type for `inner_local`

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |                                                                 ^ expected one of 8 possible tokens

error: expected one of `,`, `:`, `=`, or `>`, found `}`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |         ------------ help: use `=` if you meant to assign          - expected one of `,`, `:`, `=`, or `>`
   |         while parsing the type for `outer_local`
...
LL | }
   | ^ unexpected token
   | ^ unexpected token

error: expected one of `>`, a const expression, lifetime, or type, found `}`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |                                         ------------              ^ expected one of `>`, a const expression, lifetime, or type
   |                                         |          |
   |                                         |          help: use `=` if you meant to assign
   |                                         while parsing the type for `inner_local`

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |                                                                 ^ expected one of 8 possible tokens

error: expected one of `!`, `.`, `::`, `;`, `?`, `else`, `{`, or an operator, found `,`
   |
   |
LL |     let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
   |                                 ^ expected one of 8 possible tokens
error: aborting due to 6 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/macro/issue-37234.rs stdout ----
diff of stderr:

- error: expected one of `.`, `;`, `?`, or an operator, found `""`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `""`
3    |
4 LL |         let x = 5 "";


-    |                   ^^ expected one of `.`, `;`, `?`, or an operator
+    |                   ^^ expected one of `.`, `;`, `?`, `else`, or an operator
7 LL |     failed!();
8    |     ---------- in this macro invocation



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/issue-37234/issue-37234.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro/issue-37234.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro/issue-37234.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/issue-37234" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/issue-37234/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `""`
   |
   |
LL |         let x = 5 ""; //~ ERROR found `""`
   |                   ^^ expected one of `.`, `;`, `?`, `else`, or an operator
LL |     failed!();
   |     ---------- in this macro invocation
   |
   |
   = note: this error originates in the macro `failed` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/missing-semicolon.rs stdout ----
diff of stderr:

- error: expected one of `.`, `;`, `?`, or an operator, found keyword `let`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found keyword `let`
3    |
3    |
4 LL |         $( let x = $e1 )*;

-    |            ^^^ expected one of `.`, `;`, `?`, or an operator
+    |            ^^^ expected one of `.`, `;`, `?`, `else`, or an operator
6 ...
7 LL | fn main() { m!(0, 0; 0, 0); }


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-semicolon/missing-semicolon.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-semicolon/missing-semicolon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/missing-semicolon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/missing-semicolon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-semicolon" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-semicolon/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `.`, `;`, `?`, `else`, or an operator, found keyword `let`
   |
   |
LL |         $( let x = $e1 )*; //~ ERROR expected one of `.`, `;`, `?`, or
   |            ^^^ expected one of `.`, `;`, `?`, `else`, or an operator
...
LL | fn main() { m!(0, 0; 0, 0); }
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/range-3.rs stdout ----
diff of stderr:

- error: expected one of `.`, `;`, `?`, or an operator, found `..`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `..`
3    |
4 LL |     let r = 1..2..3;


-    |                 ^^ expected one of `.`, `;`, `?`, or an operator
+    |                 ^^ expected one of `.`, `;`, `?`, `else`, or an operator
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-3/range-3.stderr
To only update this specific test, also pass `--test-args parser/range-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/range-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `..`
   |
LL |     let r = 1..2..3;
LL |     let r = 1..2..3;
   |                 ^^ expected one of `.`, `;`, `?`, `else`, or an operator
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/range-4.rs stdout ----
diff of stderr:

- error: expected one of `.`, `;`, `?`, or an operator, found `..`
+ error: expected one of `.`, `;`, `?`, `else`, or an operator, found `..`
3    |
4 LL |     let r = ..1..2;


-    |                ^^ expected one of `.`, `;`, `?`, or an operator
+    |                ^^ expected one of `.`, `;`, `?`, `else`, or an operator
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-4/range-4.stderr
To only update this specific test, also pass `--test-args parser/range-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/range-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/range-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `..`
   |
LL |     let r = ..1..2;
LL |     let r = ..1..2;
   |                ^^ expected one of `.`, `;`, `?`, `else`, or an operator
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11988 passed; 8 failed; 103 ignored; 0 measured; 0 filtered out; finished in 123.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:46
