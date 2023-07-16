plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12578 tests
.................................................................................................... 100/12578
...........................................iiiii.iiiiiii..........i.i..............F.i.F.i.......... 200/12578
.................................................................................................... 400/12578
.................................................................................................... 500/12578
......................................i............................................................. 600/12578
.................................................................................................... 700/12578
---
.........................................................................iii........................ 12500/12578
..............................................................................
failures:

---- [ui] ui/asm/naked-functions-unused.rs#x86_64 stdout ----

5    |                                ^ help: if this is intentional, prefix it with an underscore: `_a`
6    |
7 note: the lint level is defined here
---
11    |         ^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64/naked-functions-unused.x86_64.stderr
To only update this specific test, also pass `--test-args asm/naked-functions-unused.rs`


error in revision `x86_64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "x86_64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:16:32
   |
LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
   |                                ^ help: if this is intentional, prefix it with an underscore: `_a`
note: the lint level is defined here
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:5:9
   |
LL | #![deny(unused)]
LL | #![deny(unused)]
   |         ^^^^^^
   = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:16:42
   |
   |
LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
   |                                          ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:25:38
   |
   |
LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
   |                                      ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:25:48
   |
   |
LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
   |                                                ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:31:41
   |
   |
LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
   |                                         ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:31:51
   |
   |
LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
   |                                                   ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:39:40
   |
   |
LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
   |                                        ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:39:50
   |
   |
LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
   |                                                  ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:45:43
   |
   |
LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
   |                                           ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:45:53
   |
   |
LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
   |                                                     ^ help: if this is intentional, prefix it with an underscore: `_b`
error: aborting due to 10 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/naked-functions.rs stdout ----
diff of stderr:

1 error: asm with the `pure` option must have at least one output
-   --> $DIR/naked-functions.rs:111:14
+   --> $DIR/naked-functions.rs:110:14
3    |
4 LL |     asm!("", options(readonly, nostack), options(pure));

6 
7 error: patterns not allowed in naked function parameters
-   --> $DIR/naked-functions.rs:21:5
---
13 error: patterns not allowed in naked function parameters
-   --> $DIR/naked-functions.rs:23:5
+   --> $DIR/naked-functions.rs:22:5
15    |
16 LL |     &b: &i32,

18 
19 error: patterns not allowed in naked function parameters
-   --> $DIR/naked-functions.rs:25:6
-   --> $DIR/naked-functions.rs:25:6
+   --> $DIR/naked-functions.rs:24:6
21    |
22 LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

24 
25 error: patterns not allowed in naked function parameters
-   --> $DIR/naked-functions.rs:27:5
-   --> $DIR/naked-functions.rs:27:5
+   --> $DIR/naked-functions.rs:26:5
27    |
28 LL |     P { x, y }: P,

30 
31 error: referencing function parameters is not allowed in naked functions
-   --> $DIR/naked-functions.rs:36:5
-   --> $DIR/naked-functions.rs:36:5
+   --> $DIR/naked-functions.rs:35:5
33    |
34 LL |     a + 1
35    |     ^

37    = help: follow the calling convention in asm block to use parameters
38 
39 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:34:1
+   --> $DIR/naked-functions.rs:33:1
41    |
42 LL | / pub unsafe extern "C" fn inc(a: u32) -> u32 {
43 LL | |
48    | |_^
49 
50 error: referencing function parameters is not allowed in naked functions
-   --> $DIR/naked-functions.rs:42:31
-   --> $DIR/naked-functions.rs:42:31
+   --> $DIR/naked-functions.rs:41:31
52    |
53 LL |     asm!("/* {0} */", in(reg) a, options(noreturn));


56    = help: follow the calling convention in asm block to use parameters
57 
58 error[E0787]: only `const` and `sym` operands are supported in naked functions
-   --> $DIR/naked-functions.rs:42:23
+   --> $DIR/naked-functions.rs:41:23
60    |
61 LL |     asm!("/* {0} */", in(reg) a, options(noreturn));

63 
63 
64 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:48:1
+   --> $DIR/naked-functions.rs:47:1
66    |
67 LL | / pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
68 LL | |
72    | |_^
73 
73 
74 error[E0787]: only `const` and `sym` operands are supported in naked functions
-   --> $DIR/naked-functions.rs:65:10
+   --> $DIR/naked-functions.rs:64:10
76    |
77 LL |          in(reg) a,

87    |          ^^^^^^^^^^
88 
88 
89 error[E0787]: asm in naked functions must use `noreturn` option
-   --> $DIR/naked-functions.rs:63:5
+   --> $DIR/naked-functions.rs:62:5
91    |
92 LL | /     asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
93 LL | |
99    | |_____^
100 
100 
101 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:54:1
+   --> $DIR/naked-functions.rs:53:1
103    |
104 LL | / pub unsafe extern "C" fn unsupported_operands() {
105 LL | |
119    | |_^
120 
120 
121 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:77:1
+   --> $DIR/naked-functions.rs:76:1
123    |
124 LL | / pub extern "C" fn missing_assembly() {
125 LL | |
127    | |_^
128 
128 
129 error[E0787]: asm in naked functions must use `noreturn` option
-   --> $DIR/naked-functions.rs:84:5
+   --> $DIR/naked-functions.rs:83:5
132 LL |     asm!("");
133    |     ^^^^^^^^

134 
134 
135 error[E0787]: asm in naked functions must use `noreturn` option
-   --> $DIR/naked-functions.rs:86:5
+   --> $DIR/naked-functions.rs:85:5
138 LL |     asm!("");
139    |     ^^^^^^^^

140 
140 
141 error[E0787]: asm in naked functions must use `noreturn` option
-   --> $DIR/naked-functions.rs:88:5
+   --> $DIR/naked-functions.rs:87:5
144 LL |     asm!("");
145    |     ^^^^^^^^

146 
146 
147 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:82:1
+   --> $DIR/naked-functions.rs:81:1
149    |
150 LL | / pub extern "C" fn too_many_asm_blocks() {
151 LL | |
163    | |_^
164 
165 error: referencing function parameters is not allowed in naked functions
-   --> $DIR/naked-functions.rs:97:11
-   --> $DIR/naked-functions.rs:97:11
+   --> $DIR/naked-functions.rs:96:11
167    |
168 LL |         *&y
169    |           ^

171    = help: follow the calling convention in asm block to use parameters
172 
173 error[E0787]: naked functions must contain a single asm block
-   --> $DIR/naked-functions.rs:95:5
+   --> $DIR/naked-functions.rs:94:5
175    |
176 LL | /     pub extern "C" fn inner(y: usize) -> usize {
177 LL | |
182    | |_____^
183 
183 
184 error[E0787]: asm options unsupported in naked functions: `nomem`, `preserves_flags`
-   --> $DIR/naked-functions.rs:105:5
+   --> $DIR/naked-functions.rs:104:5
186    |
187 LL |     asm!("", options(nomem, preserves_flags, noreturn));

189 
189 
190 error[E0787]: asm options unsupported in naked functions: `nostack`, `pure`, `readonly`
-   --> $DIR/naked-functions.rs:111:5
+   --> $DIR/naked-functions.rs:110:5
192    |
193 LL |     asm!("", options(readonly, nostack), options(pure));

195 
195 
196 error[E0787]: asm in naked functions must use `noreturn` option
-   --> $DIR/naked-functions.rs:111:5
+   --> $DIR/naked-functions.rs:110:5
198    |
199 LL |     asm!("", options(readonly, nostack), options(pure));

201 
201 
202 error[E0787]: asm options unsupported in naked functions: `may_unwind`
-   --> $DIR/naked-functions.rs:119:5
+   --> $DIR/naked-functions.rs:118:5
204    |
205 LL |     asm!("", options(noreturn, may_unwind));

207 
208 warning: Rust ABI is unsupported in naked functions
-   --> $DIR/naked-functions.rs:124:15
-   --> $DIR/naked-functions.rs:124:15
+   --> $DIR/naked-functions.rs:123:15
210    |
211 LL | pub unsafe fn default_abi() {


214    = note: `#[warn(undefined_naked_function_abi)]` on by default
216 warning: Rust ABI is unsupported in naked functions
-   --> $DIR/naked-functions.rs:130:15
+   --> $DIR/naked-functions.rs:129:29
218    |
218    |
- LL | pub unsafe fn rust_abi() {
-    |               ^^^^^^^^
+ LL | pub unsafe extern "Rust" fn rust_abi() {
221 
222 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:170:1
+   --> $DIR/naked-functions.rs:169:1
---
228 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:177:1
+   --> $DIR/naked-functions.rs:176:1
230    |
231 LL | #[inline(always)]

233 
234 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:184:1
-   --> $DIR/naked-functions.rs:184:1
+   --> $DIR/naked-functions.rs:183:1
236    |
237 LL | #[inline(never)]

239 
240 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:191:1
---
246 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:193:1
+   --> $DIR/naked-functions.rs:192:1
248    |
249 LL | #[inline(always)]

251 
252 error: naked functions cannot be inlined
-   --> $DIR/naked-functions.rs:195:1
-   --> $DIR/naked-functions.rs:195:1
+   --> $DIR/naked-functions.rs:194:1
254    |
255 LL | #[inline(never)]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/naked-functions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/naked-functions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/naked-functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: asm with the `pure` option must have at least one output
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:20:5
   |
   |
LL |     mut a: u32,
   |     ^^^^^

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:22:5
   |
LL |     &b: &i32,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:24:6
   |
   |
LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:26:5
   |
   |
LL |     P { x, y }: P,

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:35:5
   |
   |
LL |     a + 1
   |     ^
   |
   = help: follow the calling convention in asm block to use parameters

error[E0787]: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc(a: u32) -> u32 {
LL | |     //~^ ERROR naked functions must contain a single asm block
LL | |     a + 1
   | |     ----- non-asm is unsupported in naked functions
LL | |     //~^ ERROR referencing function parameters is not allowed in naked functions
LL | | }

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:41:31
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   |
   = help: follow the calling convention in asm block to use parameters

error[E0787]: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));


error[E0787]: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
LL | |     //~^ ERROR naked functions must contain a single asm block
LL | |     (|| a + 1)()
   | |     ------------ non-asm is unsupported in naked functions
LL | | }


error[E0787]: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |          in(reg) a,
   |          ^^^^^^^^^
LL |          //~^ ERROR only `const` and `sym` operands are supported in naked functions
LL |          inlateout(reg) b,
   |          ^^^^^^^^^^^^^^^^
LL |          inout(reg) c,
   |          ^^^^^^^^^^^^
LL |          lateout(reg) d,
   |          ^^^^^^^^^^^^^^
LL |          out(reg) e,


error[E0787]: asm in naked functions must use `noreturn` option
   |
   |
LL | /     asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
LL | |          //~^ ERROR asm in naked functions must use `noreturn` option
LL | |          in(reg) a,
LL | |          //~^ ERROR only `const` and `sym` operands are supported in naked functions
LL | |          sym G,
LL | |     );
   | |_____^


error[E0787]: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn unsupported_operands() {
LL | |     //~^ ERROR naked functions must contain a single asm block
LL | |     let mut a = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut b = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut c = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut d = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut e = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     );
LL | | }
   | |_^


error[E0787]: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn missing_assembly() {
LL | |     //~^ ERROR naked functions must contain a single asm block
LL | | }


error[E0787]: asm in naked functions must use `noreturn` option
   |
LL |     asm!("");
   |     ^^^^^^^^


error[E0787]: asm in naked functions must use `noreturn` option
   |
LL |     asm!("");
   |     ^^^^^^^^


error[E0787]: asm in naked functions must use `noreturn` option
   |
LL |     asm!("");
   |     ^^^^^^^^


error[E0787]: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn too_many_asm_blocks() {
LL | |     //~^ ERROR naked functions must contain a single asm block
LL | |     asm!("");
LL | |     //~^ ERROR asm in naked functions must use `noreturn` option
LL | |     asm!("");
   | |     -------- multiple asm blocks are unsupported in naked functions
LL | |     //~^ ERROR asm in naked functions must use `noreturn` option
LL | |     asm!("");
   | |     -------- multiple asm blocks are unsupported in naked functions
LL | |     //~^ ERROR asm in naked functions must use `noreturn` option
LL | |     asm!("", options(noreturn));
   | |     --------------------------- multiple asm blocks are unsupported in naked functions
LL | | }

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:96:11
   |
   |
LL |         *&y
   |           ^
   |
   = help: follow the calling convention in asm block to use parameters

error[E0787]: naked functions must contain a single asm block
   |
   |
LL | /     pub extern "C" fn inner(y: usize) -> usize {
LL | |         //~^ ERROR naked functions must contain a single asm block
LL | |         *&y
   | |         --- non-asm is unsupported in naked functions
LL | |         //~^ ERROR referencing function parameters is not allowed in naked functions
LL | |     }


error[E0787]: asm options unsupported in naked functions: `nomem`, `preserves_flags`
   |
   |
LL |     asm!("", options(nomem, preserves_flags, noreturn));


error[E0787]: asm options unsupported in naked functions: `nostack`, `pure`, `readonly`
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));


error[E0787]: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));


error[E0787]: asm options unsupported in naked functions: `may_unwind`
   |
   |
LL |     asm!("", options(noreturn, may_unwind));

warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:123:15
   |
   |
LL | pub unsafe fn default_abi() {
   |
   |
   = note: `#[warn(undefined_naked_function_abi)]` on by default
warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:129:29
   |
   |
LL | pub unsafe extern "Rust" fn rust_abi() {

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:169:1
   |
   |
LL | #[inline]
   | ^^^^^^^^^

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:176:1
   |
LL | #[inline(always)]

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:183:1
   |
   |
LL | #[inline(never)]

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:190:1
   |
   |
LL | #[inline]
   | ^^^^^^^^^

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:192:1
   |
LL | #[inline(always)]

error: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:194:1
   |
   |
LL | #[inline(never)]

error: aborting due to 30 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0787`.
For more information about this error, try `rustc --explain E0787`.

------------------------------------------


---- [ui] ui/rfc-2091-track-caller/error-with-naked.rs stdout ----
diff of stderr:

1 error[E0736]: cannot use `#[track_caller]` with `#[naked]`
-   --> $DIR/error-with-naked.rs:6:1
3    |
3    |
4 LL | #[track_caller]

6 
6 
7 error[E0736]: cannot use `#[track_caller]` with `#[naked]`
-   --> $DIR/error-with-naked.rs:15:5
9    |
9    |
10 LL |     #[track_caller]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked/error-with-naked.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked/error-with-naked.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-naked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-naked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL | #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`


error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL |     #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0736`.
