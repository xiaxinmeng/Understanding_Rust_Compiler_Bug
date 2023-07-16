plain
................................ii.................................................................. 8300/12704
.................iiii............................................................................... 8400/12704
......................................i.......................................i..................... 8500/12704
..............................................i..................................................... 8600/12704
...........................................F........................................................ 8700/12704
.i............................F.........F........................................................... 8800/12704
.................................................................................................... 9000/12704
.................................................................................................... 9000/12704
.....................................F.................F............................................ 9100/12704
.................................................................................................... 9300/12704
.................................................................................................... 9400/12704
..................iiii.iiiii..................................................................ii.... 9500/12704
...........i.........................................................ii............................. 9600/12704
---

+ error: mismatched closing delimiter: `}`
+   --> $DIR/issue-10636-2.rs:5:15
+    |
+ LL | pub fn trace_option(option: Option<isize>) {
+    |                                            - closing delimiter possibly meant for this
+ LL |     option.map(|some| 42;
+ ...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | }
+    | ^ mismatched closing delimiter
+    | ^ mismatched closing delimiter
+ 
1 error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
3    |

12 LL | }
13    | ^ expected expression
---
To only update this specific test, also pass `--test-args parser/issues/issue-10636-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-10636-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-10636-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-10636-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-10636-2.rs:5:15
   |
   |
LL | pub fn trace_option(option: Option<isize>) {
   |                                            - closing delimiter possibly meant for this
LL |     option.map(|some| 42;
...
LL | }
   | ^ mismatched closing delimiter


error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
   |
   |
LL |     option.map(|some| 42;
   |               ^         ^ help: `)` may belong here
   |               unclosed delimiter

error: expected expression, found `)`
  --> /checkout/src/test/ui/parser/issues/issue-10636-2.rs:8:1
---

+ error: mismatched closing delimiter: `)`
+   --> $DIR/issue-60075.rs:4:31
+    |
+ LL |     fn qux() -> Option<usize> {
+ LL |         let _ = if true {
+ LL |         });
+    |          ^ mismatched closing delimiter
+ 
+ 
1 error: expected one of `.`, `;`, `?`, `else`, or an operator, found `}`
3    |

25 LL |         });
26    |          ^ mismatched closing delimiter
---
To only update this specific test, also pass `--test-args parser/issues/issue-60075.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-60075.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-60075" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-60075/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-60075.rs:4:31
   |
   |
LL |     fn qux() -> Option<usize> {
LL |         let _ = if true {
LL |         });
   |          ^ mismatched closing delimiter


error: expected one of `.`, `;`, `?`, `else`, or an operator, found `}`
   |
LL |         });
LL |         });
   |          ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: non-item in item list
  --> /checkout/src/test/ui/parser/issues/issue-60075.rs:6:11
   |
LL | trait T {
---

error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/issues/issue-60075.rs:4:31
   |
LL |     fn qux() -> Option<usize> {
LL |         let _ = if true {
LL |         });
   |          ^ mismatched closing delimiter

---
- error: aborting due to 3 previous errors
+ error: mismatched closing delimiter: `]`
+   --> $DIR/issue-63116.rs:3:14
+    |
+ LL | impl W <s(f;Y(;]
+    |              ^ ^ mismatched closing delimiter
+    |              unclosed delimiter
+ 
+ error: aborting due to 4 previous errors
24 
---
To only update this specific test, also pass `--test-args parser/issues/issue-63116.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-63116.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-63116" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-63116/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:18
   |
   |
LL | impl W <s(f;Y(;]
   |          -       ^
   |          unclosed delimiter


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `;`
   |
   |
LL | impl W <s(f;Y(;]
   |            ^ expected one of 7 possible tokens
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:14
   |
   |
LL | impl W <s(f;Y(;]
   |              ^ ^ mismatched closing delimiter
   |              unclosed delimiter

error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:14
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:14
   |
LL | impl W <s(f;Y(;]
   |              ^ ^ mismatched closing delimiter
   |              unclosed delimiter

error: aborting due to 4 previous errors
------------------------------------------
---
- error: aborting due to previous error
+ error: mismatched closing delimiter: `}`
+   --> $DIR/unclosed_delim_mod.rs:5:7
+    |
+ LL | pub fn new() -> Result<Value, ()> {
+ LL |     Ok(Value {
+    |       ^ unclosed delimiter
+ LL |     }
+ LL | }
+ LL | }
+    | ^ mismatched closing delimiter
+ 
+ error: mismatched closing delimiter: `}`
+   --> $DIR/unclosed_delim_mod.rs:5:7
+    |
+ LL | pub fn new() -> Result<Value, ()> {
+ LL |     Ok(Value {
+    |       ^ unclosed delimiter
+ LL |     }
+ LL | }
---
To only update this specific test, also pass `--test-args parser/unclosed_delim_mod.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed_delim_mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed_delim_mod" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed_delim_mod/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
---
11 
+ error: mismatched closing delimiter: `}`
+   --> $DIR/unclosed_delim_mod.rs:5:7
+    |
+ LL | pub fn new() -> Result<Value, ()> {
+ LL |     Ok(Value {
+    |       ^ unclosed delimiter
+ LL |     }
+ LL | }
+ LL | }
+    | ^ mismatched closing delimiter
+ 
+ error: mismatched closing delimiter: `}`
+   --> $DIR/unclosed_delim_mod.rs:5:7
+    |
+ LL | pub fn new() -> Result<Value, ()> {
+ LL |     Ok(Value {
+    |       ^ unclosed delimiter
+ LL |     }
+ LL | }
---
To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20
   |
LL |     let _: usize = unclosed_delim_mod::new();
   |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found enum `Result`
   |            expected due to this
   |
   = note: expected type `usize`
              found enum `Result<Value, ()>`
---

+ error: mismatched closing delimiter: `}`
+   --> $DIR/token-error-correct-3.rs:13:21
+    |
+ LL |         if !is_directory(path.as_ref()) {
+ LL |
+ LL |
+ LL |             callback(path.as_ref();
+ ...
+ LL |         } else {
+    |         ^ mismatched closing delimiter
+ 
+ 
+ error: mismatched closing delimiter: `}`
+   --> $DIR/token-error-correct-3.rs:13:21
+    |
+ LL |         if !is_directory(path.as_ref()) {
+ LL |
+ LL |
+ LL |             callback(path.as_ref();
+ ...
+ LL |         } else {
+    |         ^ mismatched closing delimiter
+ 
+ 
1 error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
3    |


20 LL |         if !is_directory(path.as_ref()) {
22 
- error: aborting due to 3 previous errors
+ error: aborting due to 5 previous errors
24 
24 
25 For more information about this error, try `rustc --explain E0425`.
26 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3/token-error-correct-3.stderr
To only update this specific test, also pass `--test-args resolve/token-error-correct-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/token-error-correct-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:13:21
   |
   |
LL |         if !is_directory(path.as_ref()) {
   |                                         - closing delimiter possibly meant for this
LL |             //~^ ERROR cannot find function `is_directory`
LL |             callback(path.as_ref();
...
LL |         } else {
   |         ^ mismatched closing delimiter


error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:13:21
   |
LL |         if !is_directory(path.as_ref()) {
   |                                         - closing delimiter possibly meant for this
LL |             //~^ ERROR cannot find function `is_directory`
LL |             callback(path.as_ref();
...
LL |         } else {
   |         ^ mismatched closing delimiter


error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
   |
LL |             callback(path.as_ref();
LL |             callback(path.as_ref();
   |                     ^             ^ help: `)` may belong here
   |                     unclosed delimiter


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `)`
   |
   |
LL |             fs::create_dir_all(path.as_ref()).map(|()| true)
   |                                                             - expected one of `.`, `;`, `?`, `}`, or an operator
   |         ^ unexpected token

error[E0425]: cannot find function `is_directory` in this scope
  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:11:13
  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:11:13
   |
LL |         if !is_directory(path.as_ref()) {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0425`.
