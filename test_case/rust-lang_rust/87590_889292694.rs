plain
    Finished release [optimized] target(s) in 24.80s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12067 tests
..........F......................................................................................... 100/12067
.............................................ii...........ii..F...............F..................... 200/12067
.................................................................................................... 400/12067
............................................................................F....................... 500/12067
.................................................................................................... 600/12067
................................................i................................................... 700/12067
---
.................................................................................................... 3100/12067
.................................................................................................... 3200/12067
.................................................................................................... 3300/12067
.................................................................................................... 3400/12067
.........................F...F.FF.F................................................................. 3500/12067
.................................i........i.........i............................................... 3600/12067
.....F..F........................................................................................... 3700/12067
..................i............................................i.................................... 3900/12067
.................................................................................................... 4000/12067
.................................................................................................... 4100/12067
.................................................................................................... 4200/12067
---
..........ii.ii.......i...i......................................................................... 6500/12067
.........................................................i....i..................................... 6600/12067
...i..............i............i.................................................................... 6700/12067
......................................................................i............................. 6800/12067
.........................................................F...FFFFFFFFFFFFFFFFFFFFF......F........... 6900/12067
...........................i........................................................................ 7100/12067
...............F.................................................................................... 7200/12067
.................................................................................................... 7300/12067
.................................................................................................... 7400/12067
.................................................................................................... 7400/12067
.................ii................i..i..ii......................................................... 7500/12067
.................................................................................................... 7600/12067
.................................................................................................... 7700/12067
.................................................................................................... 7800/12067
.....................................................................i..ii.......................... 7900/12067
.....................................ii............................................................. 8000/12067
.................................................................................................... 8100/12067
.........................................iF......................................................... 8200/12067
..........................................................................i......................... 8400/12067
.................................................................................................... 8500/12067
............................................i....................................................... 8600/12067
.................................................................................................... 8700/12067
---
......i............................................................................................. 11400/12067
.................................................................................................... 11500/12067
.................................................................................................... 11600/12067
.................................................................................................... 11700/12067
.................................F.F................................................................ 11800/12067
........................................................i.i......................................... 12000/12067
...................................................................
failures:


---- [ui] ui/abi/abi-sysv64-register-usage.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov rdi, 1;
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args abi/abi-sysv64-register-usage.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/abi-sysv64-register-usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov rdi, 1;
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---

---- [ui] ui/asm/naked-functions.rs stdout ----
diff of stderr:

4 LL |     asm!("", options(readonly, nostack), options(pure));
6 
6 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/naked-functions.rs:111:5
+    |
+ LL |     llvm_asm!("");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
7 error: patterns not allowed in naked function parameters
---
To only update this specific test, also pass `--test-args asm/naked-functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: asm with `pure` option must have at least one output
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("");
   |
   = note: `#[warn(deprecated)]` on by default

error: patterns not allowed in naked function parameters
---

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:15:5
   |
LL |     &b: &i32,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:17:6
   |
   |
LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:19:5
   |
   |
LL |     P { x, y }: P,

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:29:5
   |
   |
LL |     a + 1
   |     ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     a + 1
   | |     ----- non-asm is unsupported in naked functions
LL | |     //~^ ERROR referencing function parameters is not allowed in naked functions
LL | | }
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:35:31
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   |
   = help: follow the calling convention in asm block to use parameters

warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     (|| a + 1)()
   | |     ------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |          in(reg) a,
...
...
LL |          inlateout(reg) b,
   |          ^^^^^^^^^^^^^^^^
LL |          inout(reg) c,
   |          ^^^^^^^^^^^^
LL |          lateout(reg) d,
   |          ^^^^^^^^^^^^^^
LL |          out(reg) e,
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL | /     asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
LL | |          //~^ WARN asm in naked functions must use `noreturn` option
LL | |          //~| WARN this was previously accepted
LL | |          in(reg) a,
LL | |          sym G,
LL | |     );
   | |______^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn unsupported_operands() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
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
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn missing_assembly() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn too_many_asm_blocks() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     asm!("");
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("", options(noreturn));
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   | |     ---------------------------- multiple asm blocks are unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:101:11
   |
LL |         *&y
   |           ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | /     pub extern "C" fn inner(y: usize) -> usize {
LL | |         //~^ WARN naked functions must contain a single asm block
LL | |         //~| WARN this was previously accepted
LL | |         *&y
   | |         --- non-asm is unsupported in naked functions
LL | |         //~^ ERROR referencing function parameters is not allowed in naked functions
LL | |     }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: the LLVM-style inline assembly is unsupported in naked functions
   |
   |
LL |     llvm_asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = help: use the new asm! syntax specified in RFC 2873
   = note: this warning originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: naked functions must contain a single asm block
   |
   |
LL | / unsafe extern "C" fn llvm() -> ! {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     llvm_asm!("");
LL | |     core::hint::unreachable_unchecked();
LL | |     core::hint::unreachable_unchecked();
   | |     ------------------------------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nomem`, `preserves_flags`
   |
   |
LL |     asm!("", options(nomem, preserves_flags, noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nostack`, `pure`, `readonly`
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:135:15
   |
LL | pub unsafe fn default_abi() {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:142:29
   |
LL | pub unsafe extern "Rust" fn rust_abi() {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---
---- [ui] ui/asm/rustfix-asm.rs stdout ----

error: fixed code is still producing diagnostics
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/rustfix-asm.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/rustfix-asm/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/rustfix-asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
  --> /checkout/src/test/ui/asm/rustfix-asm.fixed:10:9
   |
LL |         llvm_asm!("" :: "r" (x));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
  --> /checkout/src/test/ui/asm/rustfix-asm.fixed:12:9
   |
LL |         llvm_asm!("" : "=r" (y));

warning: 2 warnings emitted



------------------------------------------


---- [ui] ui/ast-json/ast-json-ice.rs#expand stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!(""::::); }
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-ice.expand/ast-json-ice.expand.stderr
To only update this specific test, also pass `--test-args ast-json/ast-json-ice.rs`


error in revision `expand`: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":258,"hi":265},"segments":[{"ident":{"name":"feature","span":{"lo":258,"hi":265}},"id":2,"args":null}],"tokens":null},"args":{"variant":"Delimited","fields":[{"open":{"lo":265,"hi":266},"close":{"lo":274,"hi":275}},"Parenthesis",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["llvm_asm",false]},"span":{"lo":266,"hi":274}}]},"Alone"]]}]},"tokens":null},{"0":[[{"variant":"Token","fields":[{"kind":"Pound","span":{"lo":255,"hi":256}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Not","span":{"lo":256,"hi":257}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":257,"hi":258},"close":{"lo":275,"hi":276}},"Bracket",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["feature",false]},"span":{"lo":258,"hi":265}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":265,"hi":266},"close":{"lo":274,"hi":275}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["llvm_asm",false]},"span":{"lo":266,"hi":274}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"id":null,"style":"Inner","span":{"lo":255,"hi":276}}],"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":5,"args":null}],"tokens":null},"args":"Empty","tokens":null},null]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":4,"span":{"lo":0,"hi":0},"vis":{"kind":"Inherited","span":{"lo":0,"hi":0},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":6,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":7,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":8,"args":null},{"ident":{"name":"rust_2015","span":{"lo":0,"hi":0}},"id":9,"args":null}],"tokens":null},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":11,"args":null}],"tokens":null},"args":"Empty","tokens":null},null]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":10,"span":{"lo":0,"hi":0},"vis":{"kind":"Inherited","span":{"lo":0,"hi":0},"tokens":null},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":12,"span":{"lo":278,"hi":326},"vis":{"kind":"Inherited","span":{"lo":278,"hi":278},"tokens":null},"ident":{"name":"V","span":{"lo":283,"hi":284}},"kind":{"variant":"Enum","fields":[{"variants":[{"attrs":{"0":null},"id":13,"span":{"lo":291,"hi":297},"vis":{"kind":"Inherited","span":{"lo":291,"hi":291},"tokens":null},"ident":{"name":"A","span":{"lo":291,"hi":292}},"data":{"variant":"Tuple","fields":[[{"attrs":{"0":null},"id":14,"span":{"lo":293,"hi":296},"vis":{"kind":"Inherited","span":{"lo":293,"hi":293},"tokens":null},"ident":null,"ty":{"id":15,"kind":{"variant":"Path","fields":[null,{"span":{"lo":293,"hi":296},"segments":[{"ident":{"name":"i32","span":{"lo":293,"hi":296}},"id":16,"args":null}],"tokens":null}]},"span":{"lo":293,"hi":296},"tokens":null},"is_placeholder":false}],17]},"disr_expr":null,"is_placeholder":false},{"attrs":{"0":null},"id":18,"span":{"lo":303,"hi":324},"vis":{"kind":"Inherited","span":{"lo":303,"hi":303},"tokens":null},"ident":{"name":"B","span":{"lo":303,"hi":304}},"data":{"variant":"Struct","fields":[[{"attrs":{"0":null},"id":19,"span":{"lo":307,"hi":322},"vis":{"kind":"Inherited","span":{"lo":307,"hi":307},"tokens":null},"ident":{"name":"f","span":{"lo":307,"hi":308}},"ty":{"id":20,"kind":{"variant":"Array","fields":[{"id":21,"kind":{"variant":"Path","fields":[null,{"span":{"lo":311,"hi":314},"segments":[{"ident":{"name":"i64","span":{"lo":311,"hi":314}},"id":22,"args":null}],"tokens":null}]},"span":{"lo":311,"hi":314},"tokens":null},{"id":23,"value":{"id":24,"kind":{"variant":"Binary","fields":[{"node":"Add","span":{"lo":318,"hi":319}},{"id":25,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"3","suffix":null},"kind":{"variant":"Int","fields":[3,"Unsuffixed"]},"span":{"lo":316,"hi":317}}]},"span":{"lo":316,"hi":317},"attrs":{"0":null},"tokens":null},{"id":26,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"4","suffix":null},"kind":{"variant":"Int","fields":[4,"Unsuffixed"]},"span":{"lo":320,"hi":321}}]},"span":{"lo":320,"hi":321},"attrs":{"0":null},"tokens":null}]},"span":{"lo":316,"hi":321},"attrs":{"0":null},"tokens":null}}]},"span":{"lo":310,"hi":322},"tokens":null},"is_placeholder":false}],false]},"disr_expr":null,"is_placeholder":false}]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":284,"hi":284}},"span":{"lo":284,"hi":284}}]},"tokens":null},{"attrs":[],"id":27,"span":{"lo":328,"hi":434},"vis":{"kind":"Inherited","span":{"lo":328,"hi":328},"tokens":null},"ident":{"name":"X","span":{"lo":334,"hi":335}},"kind":{"variant":"Trait","fields":[{"0":"No","1":"No","2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":335,"hi":335}},"span":{"lo":335,"hi":335}},"3":[],"4":[{"attrs":[],"id":28,"span":{"lo":342,"hi":354},"vis":{"kind":"Inherited","span":{"lo":342,"hi":342},"tokens":null},"ident":{"name":"Output","span":{"lo":347,"hi":353}},"kind":{"variant":"TyAlias","fields":[{"0":"Final","1":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":353,"hi":353}},"span":{"lo":353,"hi":353}},"2":[],"3":null}]},"tokens":null},{"attrs":[],"id":29,"span":{"lo":359,"hi":390},"vis":{"kind":"Inherited","span":{"lo":359,"hi":359},"tokens":null},"ident":{"name":"read","span":{"lo":362,"hi":366}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[{"attrs":{"0":null},"ty":{"id":32,"kind":{"variant":"Rptr","fields":[null,{"ty":{"id":33,"kind":"ImplicitSelf","span":{"lo":367,"hi":372},"tokens":null},"mutbl":"Not"}]},"span":{"lo":367,"hi":372},"tokens":null},"pat":{"id":31,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"self","span":{"lo":368,"hi":372}},null]},"span":{"lo":367,"hi":372},"tokens":null},"id":30,"span":{"lo":367,"hi":372},"is_placeholder":false}],"output":{"variant":"Ty","fields":[{"id":34,"kind":{"variant":"Path","fields":[null,{"span":{"lo":377,"hi":389},"segments":[{"ident":{"name":"Self","span":{"lo":377,"hi":381}},"id":35,"args":null},{"ident":{"name":"Output","span":{"lo":383,"hi":389}},"id":36,"args":null}],"tokens":null}]},"span":{"lo":377,"hi":389},"tokens":null}]}},"span":{"lo":359,"hi":390}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":389,"hi":389}},"span":{"lo":366,"hi":366}},"3":null}]},"tokens":null},{"attrs":[],"id":37,"span":{"lo":395,"hi":432},"vis":{"kind":"Inherited","span":{"lo":395,"hi":395},"tokens":null},"ident":{"name":"write","span":{"lo":398,"hi":403}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[{"attrs":{"0":null},"ty":{"id":40,"kind":{"variant":"Rptr","fields":[null,{"ty":{"id":41,"kind":"ImplicitSelf","span":{"lo":404,"hi":413},"tokens":null},"mutbl":"Mut"}]},"span":{"lo":404,"hi":413},"tokens":null},"pat":{"id":39,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"self","span":{"lo":409,"hi":413}},null]},"span":{"lo":404,"hi":413},"tokens":null},"id":38,"span":{"lo":404,"hi":413},"is_placeholder":false},{"attrs":{"0":null},"ty":{"id":44,"kind":{"variant":"Path","fields":[null,{"span":{"lo":418,"hi":430},"segments":[{"ident":{"name":"Self","span":{"lo":418,"hi":422}},"id":45,"args":null},{"ident":{"name":"Output","span":{"lo":424,"hi":430}},"id":46,"args":null}],"tokens":null}]},"span":{"lo":418,"hi":430},"tokens":null},"pat":{"id":43,"kind":"Wild","span":{"lo":415,"hi":416},"tokens":null},"id":42,"span":{"lo":415,"hi":430},"is_placeholder":false}],"output":{"variant":"Default","fields":[{"lo":431,"hi":431}]}},"span":{"lo":395,"hi":432}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":431,"hi":431}},"span":{"lo":403,"hi":403}},"3":null}]},"tokens":null}]}]},"tokens":null},{"attrs":[],"id":47,"span":{"lo":436,"hi":506},"vis":{"kind":"Inherited","span":{"lo":436,"hi":436},"tokens":null},"ident":{"name":"call_println","span":{"lo":449,"hi":461}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":462,"hi":463},"close":{"lo":505,"hi":506}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":468,"hi":469},"close":{"lo":477,"hi":478}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":469,"hi":470}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["y",false]},"span":{"lo":470,"hi":471}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":471,"hi":472}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["ident",false]},"span":{"lo":472,"hi":477}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":479,"hi":481}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":482,"hi":483},"close":{"lo":503,"hi":504}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["println",false]},"span":{"lo":484,"hi":491}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Not","span":{"lo":491,"hi":492}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":492,"hi":493},"close":{"lo":501,"hi":502}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"{}","suffix":null}]},"span":{"lo":493,"hi":497}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":497,"hi":498}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":499,"hi":500}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["y",false]},"span":{"lo":500,"hi":501}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":48,"span":{"lo":508,"hi":776},"vis":{"kind":"Inherited","span":{"lo":508,"hi":508},"tokens":null},"ident":{"name":"main","span":{"lo":511,"hi":515}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":518,"hi":518}]}},"span":{"lo":508,"hi":517}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":517,"hi":517}},"span":{"lo":515,"hi":515}},"3":{"stmts":[{"id":127,"kind":{"variant":"Expr","fields":[{"id":50,"kind":{"variant":"Block","fields":[{"stmts":[{"id":126,"kind":{"variant":"Semi","fields":[{"id":74,"kind":{"variant":"LlvmInlineAsm","fields":[{"asm":"","asm_str_style":"Cooked","outputs":[],"inputs":[],"clobbers":[],"volatile":true,"alignstack":false,"dialect":"Att"}]},"span":{"lo":664,"hi":682},"attrs":{"0":null},"tokens":null}]},"span":{"lo":664,"hi":682}}],"id":51,"rules":{"variant":"Unsafe","fields":["UserProvided"]},"span":{"lo":655,"hi":684},"tokens":null},null]},"span":{"lo":655,"hi":684},"attrs":{"0":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":526,"hi":529},"segments":[{"ident":{"name":"cfg","span":{"lo":526,"hi":529}},"id":52,"args":null}],"tokens":null},"args":{"variant":"Delimited","fields":[{"open":{"lo":529,"hi":530},"close":{"lo":648,"hi":649}},"Parenthesis",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["any",false]},"span":{"lo":530,"hi":533}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":533,"hi":534},"close":{"lo":647,"hi":648}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":534,"hi":545}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":546,"hi":547}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86","suffix":null}]},"span":{"lo":548,"hi":553}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":553,"hi":554}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":563,"hi":574}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":575,"hi":576}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86_64","suffix":null}]},"span":{"lo":577,"hi":585}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":585,"hi":586}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":595,"hi":606}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":607,"hi":608}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"arm","suffix":null}]},"span":{"lo":609,"hi":614}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":614,"hi":615}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":624,"hi":635}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":636,"hi":637}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"aarch64","suffix":null}]},"span":{"lo":638,"hi":647}}]},"Alone"]]}]},"Alone"]]}]},"tokens":null},{"0":[[{"variant":"Token","fields":[{"kind":"Pound","span":{"lo":524,"hi":525}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":525,"hi":526},"close":{"lo":649,"hi":650}},"Bracket",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["cfg",false]},"span":{"lo":526,"hi":529}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":529,"hi":530},"close":{"lo":648,"hi":649}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["any",false]},"span":{"lo":530,"hi":533}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":533,"hi":534},"close":{"lo":647,"hi":648}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":534,"hi":545}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":546,"hi":547}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86","suffix":null}]},"span":{"lo":548,"hi":553}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":553,"hi":554}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":563,"hi":574}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":575,"hi":576}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86_64","suffix":null}]},"span":{"lo":577,"hi":585}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":585,"hi":586}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":595,"hi":606}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":607,"hi":608}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"arm","suffix":null}]},"span":{"lo":609,"hi":614}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":614,"hi":615}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":624,"hi":635}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":636,"hi":637}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"aarch64","suffix":null}]},"span":{"lo":638,"hi":647}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"id":null,"style":"Outer","span":{"lo":524,"hi":650}}]},"tokens":null}]},"span":{"lo":655,"hi":684}},{"id":128,"kind":{"variant":"Local","fields":[{"id":53,"pat":{"id":54,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"x","span":{"lo":694,"hi":695}},null]},"span":{"lo":694,"hi":695},"tokens":null},"ty":{"id":55,"kind":{"variant":"Paren","fields":[{"id":56,"kind":{"variant":"Path","fields":[null,{"span":{"lo":698,"hi":701},"segments":[{"ident":{"name":"i32","span":{"lo":698,"hi":701}},"id":57,"args":null}],"tokens":null}]},"span":{"lo":698,"hi":701},"tokens":null}]},"span":{"lo":697,"hi":702},"tokens":null},"init":{"id":58,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"35","suffix":null},"kind":{"variant":"Int","fields":[35,"Unsuffixed"]},"span":{"lo":705,"hi":707}}]},"span":{"lo":705,"hi":707},"attrs":{"0":null},"tokens":null},"span":{"lo":690,"hi":708},"attrs":{"0":null},"tokens":null}]},"span":{"lo":690,"hi":708}},{"id":129,"kind":{"variant":"Local","fields":[{"id":59,"pat":{"id":60,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"y","span":{"lo":717,"hi":718}},null]},"span":{"lo":717,"hi":718},"tokens":null},"ty":null,"init":{"id":61,"kind":{"variant":"Binary","fields":[{"node":"Add","span":{"lo":732,"hi":733}},{"id":62,"kind":{"variant":"Cast","fields":[{"id":63,"kind":{"variant":"Path","fields":[null,{"span":{"lo":721,"hi":722},"segments":[{"ident":{"name":"x","span":{"lo":721,"hi":722}},"id":64,"args":null}],"tokens":null}]},"span":{"lo":721,"hi":722},"attrs":{"0":null},"tokens":null},{"id":65,"kind":{"variant":"Path","fields":[null,{"span":{"lo":726,"hi":731},"segments":[{"ident":{"name":"i64","span":{"lo":726,"hi":729}},"id":66,"args":{"variant":"AngleBracketed","fields":[{"span":{"lo":729,"hi":731},"args":[]}]}}],"tokens":null}]},"span":{"lo":726,"hi":731},"tokens":null}]},"span":{"lo":721,"hi":731},"attrs":{"0":null},"tokens":null},{"id":67,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"5","suffix":null},"kind":{"variant":"Int","fields":[5,"Unsuffixed"]},"span":{"lo":734,"hi":735}}]},"span":{"lo":734,"hi":735},"attrs":{"0":null},"tokens":null}]},"span":{"lo":721,"hi":735},"attrs":{"0":null},"tokens":null},"span":{"lo":713,"hi":736},"attrs":{"0":null},"tokens":null}]},"span":{"lo":713,"hi":736}},{"id":130,"kind":{"variant":"Semi","fields":[{"id":75,"kind":{"variant":"Block","fields":[{"stmts":[{"id":125,"kind":{"variant":"Semi","fields":[{"id":77,"kind":{"variant":"Call","fields":[{"id":78,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25769,"hi":25787},"segments":[{"ident":{"name":"$crate","span":{"lo":25769,"hi":25775}},"id":79,"args":null},{"ident":{"name":"io","span":{"lo":25777,"hi":25779}},"id":80,"args":null},{"ident":{"name":"_print","span":{"lo":25781,"hi":25787}},"id":81,"args":null}],"tokens":null}]},"span":{"lo":25769,"hi":25787},"attrs":{"0":null},"tokens":null},[{"id":82,"kind":{"variant":"Call","fields":[{"id":83,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25788,"hi":25821},"segments":[{"ident":{"name":"$crate","span":{"lo":25788,"hi":25821}},"id":84,"args":null},{"ident":{"name":"fmt","span":{"lo":25788,"hi":25821}},"id":85,"args":null},{"ident":{"name":"Arguments","span":{"lo":25788,"hi":25821}},"id":86,"args":null},{"ident":{"name":"new_v1","span":{"lo":25788,"hi":25821}},"id":87,"args":null}],"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"id":88,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":89,"kind":{"variant":"Array","fields":[[{"id":90,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Str","symbol":"","suffix":null},"kind":{"variant":"Str","fields":["","Cooked"]},"span":{"lo":493,"hi":497}}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null},{"id":91,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Str","symbol":"\\n","suffix":null},"kind":{"variant":"Str","fields":["\n","Cooked"]},"span":{"lo":493,"hi":497}}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null},{"id":92,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":93,"kind":{"variant":"Match","fields":[{"id":94,"kind":{"variant":"Tup","fields":[[{"id":95,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":96,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"y","span":{"lo":756,"hi":757}},"id":97,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"attrs":{"0":null},"pat":{"id":99,"kind":{"variant":"Tuple","fields":[[{"id":100,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"arg0","span":{"lo":499,"hi":501}},null]},"span":{"lo":499,"hi":501},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"tokens":null},"guard":null,"body":{"id":101,"kind":{"variant":"Array","fields":[[{"id":102,"kind":{"variant":"Call","fields":[{"id":103,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25788,"hi":25821},"segments":[{"ident":{"name":"$crate","span":{"lo":25788,"hi":25821}},"id":104,"args":null},{"ident":{"name":"fmt","span":{"lo":25788,"hi":25821}},"id":105,"args":null},{"ident":{"name":"ArgumentV1","span":{"lo":25788,"hi":25821}},"id":106,"args":null},{"ident":{"name":"new","span":{"lo":25788,"hi":25821}},"id":107,"args":null}],"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"id":108,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"arg0","span":{"lo":499,"hi":501}},"id":109,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null},{"id":110,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"$crate","span":{"lo":499,"hi":501}},"id":111,"args":null},{"ident":{"name":"fmt","span":{"lo":499,"hi":501}},"id":112,"args":null},{"ident":{"name":"Display","span":{"lo":499,"hi":501}},"id":113,"args":null},{"ident":{"name":"fmt","span":{"lo":499,"hi":501}},"id":114,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},"span":{"lo":25788,"hi":25821},"id":98,"is_placeholder":false}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25769,"hi":25822},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25769,"hi":25823}}],"id":76,"rules":"Default","span":{"lo":25759,"hi":25829},"tokens":null},null]},"span":{"lo":25759,"hi":25829},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25759,"hi":25829}},{"id":131,"kind":{"variant":"Item","fields":[{"attrs":[],"id":68,"span":{"lo":765,"hi":774},"vis":{"kind":"Inherited","span":{"lo":765,"hi":765},"tokens":null},"ident":{"name":"A","span":{"lo":772,"hi":773}},"kind":{"variant":"Struct","fields":[{"variant":"Unit","fields":[69]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":773,"hi":773}},"span":{"lo":773,"hi":773}}]},"tokens":null}]},"span":{"lo":765,"hi":774}}],"id":49,"rules":"Default","span":{"lo":518,"hi":776},"tokens":null}}]},"tokens":null},{"attrs":[],"id":70,"span":{"lo":879,"hi":888},"vis":{"kind":"Inherited","span":{"lo":879,"hi":879},"tokens":null},"ident":{"name":"S","span":{"lo":886,"hi":887}},"kind":{"variant":"Struct","fields":[{"variant":"Unit","fields":[71]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":887,"hi":887}},"span":{"lo":887,"hi":887}}]},"tokens":null},{"attrs":[],"id":72,"span":{"lo":890,"hi":968},"vis":{"kind":"Inherited","span":{"lo":890,"hi":890},"tokens":null},"ident":{"name":"mac_extern","span":{"lo":903,"hi":913}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":914,"hi":915},"close":{"lo":967,"hi":968}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":920,"hi":921},"close":{"lo":928,"hi":929}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":921,"hi":922}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":922,"hi":923}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":923,"hi":924}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["item",false]},"span":{"lo":924,"hi":928}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":930,"hi":932}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":933,"hi":934},"close":{"lo":965,"hi":966}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":943,"hi":949}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"C","suffix":null}]},"span":{"lo":950,"hi":953}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":954,"hi":955},"close":{"lo":959,"hi":960}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":956,"hi":957}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":957,"hi":958}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":73,"span":{"lo":969,"hi":1067},"vis":{"kind":"Inherited","span":{"lo":969,"hi":969},"tokens":null},"ident":{"name":"mac_assoc","span":{"lo":982,"hi":991}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":992,"hi":993},"close":{"lo":1066,"hi":1067}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":998,"hi":999},"close":{"lo":1006,"hi":1007}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":999,"hi":1000}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1000,"hi":1001}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":1001,"hi":1002}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["item",false]},"span":{"lo":1002,"hi":1006}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":1008,"hi":1010}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1011,"hi":1012},"close":{"lo":1064,"hi":1065}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["impl",false]},"span":{"lo":1021,"hi":1025}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["S",false]},"span":{"lo":1026,"hi":1027}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1028,"hi":1029},"close":{"lo":1033,"hi":1034}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":1030,"hi":1031}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1031,"hi":1032}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["trait",false]},"span":{"lo":1043,"hi":1048}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["Bar",false]},"span":{"lo":1049,"hi":1052}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1053,"hi":1054},"close":{"lo":1058,"hi":1059}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":1055,"hi":1056}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1056,"hi":1057}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":115,"span":{"lo":943,"hi":960},"vis":{"kind":"Inherited","span":{"lo":943,"hi":943},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"ForeignMod","fields":[{"unsafety":"No","abi":{"style":"Cooked","symbol":"C","suffix":null,"span":{"lo":950,"hi":953},"symbol_unescaped":"C"},"items":[{"attrs":[],"id":116,"span":{"lo":1087,"hi":1096},"vis":{"kind":"Inherited","span":{"lo":1087,"hi":1087},"tokens":null},"ident":{"name":"foo","span":{"lo":1090,"hi":1093}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1095,"hi":1095}]}},"span":{"lo":1087,"hi":1096}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1095,"hi":1095}},"span":{"lo":1093,"hi":1093}},"3":null}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1087,"hi":1089}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1090,"hi":1093}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1093,"hi":1094},"close":{"lo":1094,"hi":1095}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":1095,"hi":1096}}]},"Alone"]]}}]}]},"tokens":null},{"attrs":[],"id":117,"span":{"lo":1021,"hi":1034},"vis":{"kind":"Inherited","span":{"lo":1021,"hi":1021},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Impl","fields":[{"unsafety":"No","polarity":"Positive","defaultness":"Final","constness":"No","generics":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1027,"hi":1027}},"span":{"lo":1025,"hi":1025}},"of_trait":null,"self_ty":{"id":118,"kind":{"variant":"Path","fields":[null,{"span":{"lo":1026,"hi":1027},"segments":[{"ident":{"name":"S","span":{"lo":1026,"hi":1027}},"id":119,"args":null}],"tokens":null}]},"span":{"lo":1026,"hi":1027},"tokens":null},"items":[{"attrs":[],"id":120,"span":{"lo":1116,"hi":1127},"vis":{"kind":"Inherited","span":{"lo":1116,"hi":1116},"tokens":null},"ident":{"name":"foo","span":{"lo":1119,"hi":1122}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1125,"hi":1125}]}},"span":{"lo":1116,"hi":1124}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1124,"hi":1124}},"span":{"lo":1122,"hi":1122}},"3":{"stmts":[],"id":121,"rules":"Default","span":{"lo":1125,"hi":1127},"tokens":null}}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1116,"hi":1118}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1119,"hi":1122}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1122,"hi":1123},"close":{"lo":1123,"hi":1124}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1125,"hi":1126},"close":{"lo":1126,"hi":1127}},"Brace",{"0":[]}]},"Alone"]]}}]}]},"tokens":null},{"attrs":[],"id":122,"span":{"lo":1043,"hi":1059},"vis":{"kind":"Inherited","span":{"lo":1043,"hi":1043},"tokens":null},"ident":{"name":"Bar","span":{"lo":1049,"hi":1052}},"kind":{"variant":"Trait","fields":[{"0":"No","1":"No","2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1052,"hi":1052}},"span":{"lo":1052,"hi":1052}},"3":[],"4":[{"attrs":[],"id":123,"span":{"lo":1116,"hi":1127},"vis":{"kind":"Inherited","span":{"lo":1116,"hi":1116},"tokens":null},"ident":{"name":"foo","span":{"lo":1119,"hi":1122}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1125,"hi":1125}]}},"span":{"lo":1116,"hi":1124}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1124,"hi":1124}},"span":{"lo":1122,"hi":1122}},"3":{"stmts":[],"id":124,"rules":"Default","span":{"lo":1125,"hi":1127},"tokens":null}}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1116,"hi":1118}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1119,"hi":1122}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1122,"hi":1123},"close":{"lo":1123,"hi":1124}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1125,"hi":1126},"close":{"lo":1126,"hi":1127}},"Brace",{"0":[]}]},"Alone"]]}}]}]},"tokens":null}],"span":{"lo":255,"hi":1129},"proc_macros":[]}
output: {"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":258,"hi":265},"segments":[{"ident":{"name":"feature","span":{"lo":258,"hi":265}},"id":2,"args":null}],"tokens":null},"args":{"variant":"Delimited","fields":[{"open":{"lo":265,"hi":266},"close":{"lo":274,"hi":275}},"Parenthesis",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["llvm_asm",false]},"span":{"lo":266,"hi":274}}]},"Alone"]]}]},"tokens":null},{"0":[[{"variant":"Token","fields":[{"kind":"Pound","span":{"lo":255,"hi":256}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Not","span":{"lo":256,"hi":257}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":257,"hi":258},"close":{"lo":275,"hi":276}},"Bracket",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["feature",false]},"span":{"lo":258,"hi":265}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":265,"hi":266},"close":{"lo":274,"hi":275}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["llvm_asm",false]},"span":{"lo":266,"hi":274}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"id":null,"style":"Inner","span":{"lo":255,"hi":276}}],"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":5,"args":null}],"tokens":null},"args":"Empty","tokens":null},null]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":4,"span":{"lo":0,"hi":0},"vis":{"kind":"Inherited","span":{"lo":0,"hi":0},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":6,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":7,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":8,"args":null},{"ident":{"name":"rust_2015","span":{"lo":0,"hi":0}},"id":9,"args":null}],"tokens":null},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":11,"args":null}],"tokens":null},"args":"Empty","tokens":null},null]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":10,"span":{"lo":0,"hi":0},"vis":{"kind":"Inherited","span":{"lo":0,"hi":0},"tokens":null},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":12,"span":{"lo":278,"hi":326},"vis":{"kind":"Inherited","span":{"lo":278,"hi":278},"tokens":null},"ident":{"name":"V","span":{"lo":283,"hi":284}},"kind":{"variant":"Enum","fields":[{"variants":[{"attrs":{"0":null},"id":13,"span":{"lo":291,"hi":297},"vis":{"kind":"Inherited","span":{"lo":291,"hi":291},"tokens":null},"ident":{"name":"A","span":{"lo":291,"hi":292}},"data":{"variant":"Tuple","fields":[[{"attrs":{"0":null},"id":14,"span":{"lo":293,"hi":296},"vis":{"kind":"Inherited","span":{"lo":293,"hi":293},"tokens":null},"ident":null,"ty":{"id":15,"kind":{"variant":"Path","fields":[null,{"span":{"lo":293,"hi":296},"segments":[{"ident":{"name":"i32","span":{"lo":293,"hi":296}},"id":16,"args":null}],"tokens":null}]},"span":{"lo":293,"hi":296},"tokens":null},"is_placeholder":false}],17]},"disr_expr":null,"is_placeholder":false},{"attrs":{"0":null},"id":18,"span":{"lo":303,"hi":324},"vis":{"kind":"Inherited","span":{"lo":303,"hi":303},"tokens":null},"ident":{"name":"B","span":{"lo":303,"hi":304}},"data":{"variant":"Struct","fields":[[{"attrs":{"0":null},"id":19,"span":{"lo":307,"hi":322},"vis":{"kind":"Inherited","span":{"lo":307,"hi":307},"tokens":null},"ident":{"name":"f","span":{"lo":307,"hi":308}},"ty":{"id":20,"kind":{"variant":"Array","fields":[{"id":21,"kind":{"variant":"Path","fields":[null,{"span":{"lo":311,"hi":314},"segments":[{"ident":{"name":"i64","span":{"lo":311,"hi":314}},"id":22,"args":null}],"tokens":null}]},"span":{"lo":311,"hi":314},"tokens":null},{"id":23,"value":{"id":24,"kind":{"variant":"Binary","fields":[{"node":"Add","span":{"lo":318,"hi":319}},{"id":25,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"3","suffix":null},"kind":{"variant":"Int","fields":[3,"Unsuffixed"]},"span":{"lo":316,"hi":317}}]},"span":{"lo":316,"hi":317},"attrs":{"0":null},"tokens":null},{"id":26,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"4","suffix":null},"kind":{"variant":"Int","fields":[4,"Unsuffixed"]},"span":{"lo":320,"hi":321}}]},"span":{"lo":320,"hi":321},"attrs":{"0":null},"tokens":null}]},"span":{"lo":316,"hi":321},"attrs":{"0":null},"tokens":null}}]},"span":{"lo":310,"hi":322},"tokens":null},"is_placeholder":false}],false]},"disr_expr":null,"is_placeholder":false}]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":284,"hi":284}},"span":{"lo":284,"hi":284}}]},"tokens":null},{"attrs":[],"id":27,"span":{"lo":328,"hi":434},"vis":{"kind":"Inherited","span":{"lo":328,"hi":328},"tokens":null},"ident":{"name":"X","span":{"lo":334,"hi":335}},"kind":{"variant":"Trait","fields":[{"0":"No","1":"No","2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":335,"hi":335}},"span":{"lo":335,"hi":335}},"3":[],"4":[{"attrs":[],"id":28,"span":{"lo":342,"hi":354},"vis":{"kind":"Inherited","span":{"lo":342,"hi":342},"tokens":null},"ident":{"name":"Output","span":{"lo":347,"hi":353}},"kind":{"variant":"TyAlias","fields":[{"0":"Final","1":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":353,"hi":353}},"span":{"lo":353,"hi":353}},"2":[],"3":null}]},"tokens":null},{"attrs":[],"id":29,"span":{"lo":359,"hi":390},"vis":{"kind":"Inherited","span":{"lo":359,"hi":359},"tokens":null},"ident":{"name":"read","span":{"lo":362,"hi":366}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[{"attrs":{"0":null},"ty":{"id":32,"kind":{"variant":"Rptr","fields":[null,{"ty":{"id":33,"kind":"ImplicitSelf","span":{"lo":367,"hi":372},"tokens":null},"mutbl":"Not"}]},"span":{"lo":367,"hi":372},"tokens":null},"pat":{"id":31,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"self","span":{"lo":368,"hi":372}},null]},"span":{"lo":367,"hi":372},"tokens":null},"id":30,"span":{"lo":367,"hi":372},"is_placeholder":false}],"output":{"variant":"Ty","fields":[{"id":34,"kind":{"variant":"Path","fields":[null,{"span":{"lo":377,"hi":389},"segments":[{"ident":{"name":"Self","span":{"lo":377,"hi":381}},"id":35,"args":null},{"ident":{"name":"Output","span":{"lo":383,"hi":389}},"id":36,"args":null}],"tokens":null}]},"span":{"lo":377,"hi":389},"tokens":null}]}},"span":{"lo":359,"hi":390}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":389,"hi":389}},"span":{"lo":366,"hi":366}},"3":null}]},"tokens":null},{"attrs":[],"id":37,"span":{"lo":395,"hi":432},"vis":{"kind":"Inherited","span":{"lo":395,"hi":395},"tokens":null},"ident":{"name":"write","span":{"lo":398,"hi":403}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[{"attrs":{"0":null},"ty":{"id":40,"kind":{"variant":"Rptr","fields":[null,{"ty":{"id":41,"kind":"ImplicitSelf","span":{"lo":404,"hi":413},"tokens":null},"mutbl":"Mut"}]},"span":{"lo":404,"hi":413},"tokens":null},"pat":{"id":39,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"self","span":{"lo":409,"hi":413}},null]},"span":{"lo":404,"hi":413},"tokens":null},"id":38,"span":{"lo":404,"hi":413},"is_placeholder":false},{"attrs":{"0":null},"ty":{"id":44,"kind":{"variant":"Path","fields":[null,{"span":{"lo":418,"hi":430},"segments":[{"ident":{"name":"Self","span":{"lo":418,"hi":422}},"id":45,"args":null},{"ident":{"name":"Output","span":{"lo":424,"hi":430}},"id":46,"args":null}],"tokens":null}]},"span":{"lo":418,"hi":430},"tokens":null},"pat":{"id":43,"kind":"Wild","span":{"lo":415,"hi":416},"tokens":null},"id":42,"span":{"lo":415,"hi":430},"is_placeholder":false}],"output":{"variant":"Default","fields":[{"lo":431,"hi":431}]}},"span":{"lo":395,"hi":432}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":431,"hi":431}},"span":{"lo":403,"hi":403}},"3":null}]},"tokens":null}]}]},"tokens":null},{"attrs":[],"id":47,"span":{"lo":436,"hi":506},"vis":{"kind":"Inherited","span":{"lo":436,"hi":436},"tokens":null},"ident":{"name":"call_println","span":{"lo":449,"hi":461}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":462,"hi":463},"close":{"lo":505,"hi":506}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":468,"hi":469},"close":{"lo":477,"hi":478}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":469,"hi":470}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["y",false]},"span":{"lo":470,"hi":471}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":471,"hi":472}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["ident",false]},"span":{"lo":472,"hi":477}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":479,"hi":481}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":482,"hi":483},"close":{"lo":503,"hi":504}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["println",false]},"span":{"lo":484,"hi":491}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Not","span":{"lo":491,"hi":492}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":492,"hi":493},"close":{"lo":501,"hi":502}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"{}","suffix":null}]},"span":{"lo":493,"hi":497}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":497,"hi":498}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":499,"hi":500}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["y",false]},"span":{"lo":500,"hi":501}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":48,"span":{"lo":508,"hi":776},"vis":{"kind":"Inherited","span":{"lo":508,"hi":508},"tokens":null},"ident":{"name":"main","span":{"lo":511,"hi":515}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":518,"hi":518}]}},"span":{"lo":508,"hi":517}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":517,"hi":517}},"span":{"lo":515,"hi":515}},"3":{"stmts":[{"id":127,"kind":{"variant":"Expr","fields":[{"id":50,"kind":{"variant":"Block","fields":[{"stmts":[{"id":126,"kind":{"variant":"Semi","fields":[{"id":74,"kind":{"variant":"LlvmInlineAsm","fields":[{"asm":"","asm_str_style":"Cooked","outputs":[],"inputs":[],"clobbers":[],"volatile":true,"alignstack":false,"dialect":"Att"}]},"span":{"lo":664,"hi":682},"attrs":{"0":null},"tokens":null}]},"span":{"lo":664,"hi":682}}],"id":51,"rules":{"variant":"Unsafe","fields":["UserProvided"]},"span":{"lo":655,"hi":684},"tokens":null},null]},"span":{"lo":655,"hi":684},"attrs":{"0":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":526,"hi":529},"segments":[{"ident":{"name":"cfg","span":{"lo":526,"hi":529}},"id":52,"args":null}],"tokens":null},"args":{"variant":"Delimited","fields":[{"open":{"lo":529,"hi":530},"close":{"lo":648,"hi":649}},"Parenthesis",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["any",false]},"span":{"lo":530,"hi":533}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":533,"hi":534},"close":{"lo":647,"hi":648}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":534,"hi":545}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":546,"hi":547}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86","suffix":null}]},"span":{"lo":548,"hi":553}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":553,"hi":554}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":563,"hi":574}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":575,"hi":576}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86_64","suffix":null}]},"span":{"lo":577,"hi":585}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":585,"hi":586}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":595,"hi":606}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":607,"hi":608}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"arm","suffix":null}]},"span":{"lo":609,"hi":614}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":614,"hi":615}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":624,"hi":635}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":636,"hi":637}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"aarch64","suffix":null}]},"span":{"lo":638,"hi":647}}]},"Alone"]]}]},"Alone"]]}]},"tokens":null},{"0":[[{"variant":"Token","fields":[{"kind":"Pound","span":{"lo":524,"hi":525}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":525,"hi":526},"close":{"lo":649,"hi":650}},"Bracket",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["cfg",false]},"span":{"lo":526,"hi":529}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":529,"hi":530},"close":{"lo":648,"hi":649}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["any",false]},"span":{"lo":530,"hi":533}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":533,"hi":534},"close":{"lo":647,"hi":648}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":534,"hi":545}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":546,"hi":547}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86","suffix":null}]},"span":{"lo":548,"hi":553}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":553,"hi":554}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":563,"hi":574}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":575,"hi":576}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"x86_64","suffix":null}]},"span":{"lo":577,"hi":585}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":585,"hi":586}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":595,"hi":606}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":607,"hi":608}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"arm","suffix":null}]},"span":{"lo":609,"hi":614}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Comma","span":{"lo":614,"hi":615}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["target_arch",false]},"span":{"lo":624,"hi":635}}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Eq","span":{"lo":636,"hi":637}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"aarch64","suffix":null}]},"span":{"lo":638,"hi":647}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"id":null,"style":"Outer","span":{"lo":524,"hi":650}}]},"tokens":null}]},"span":{"lo":655,"hi":684}},{"id":128,"kind":{"variant":"Local","fields":[{"id":53,"pat":{"id":54,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"x","span":{"lo":694,"hi":695}},null]},"span":{"lo":694,"hi":695},"tokens":null},"ty":{"id":55,"kind":{"variant":"Paren","fields":[{"id":56,"kind":{"variant":"Path","fields":[null,{"span":{"lo":698,"hi":701},"segments":[{"ident":{"name":"i32","span":{"lo":698,"hi":701}},"id":57,"args":null}],"tokens":null}]},"span":{"lo":698,"hi":701},"tokens":null}]},"span":{"lo":697,"hi":702},"tokens":null},"init":{"id":58,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"35","suffix":null},"kind":{"variant":"Int","fields":[35,"Unsuffixed"]},"span":{"lo":705,"hi":707}}]},"span":{"lo":705,"hi":707},"attrs":{"0":null},"tokens":null},"span":{"lo":690,"hi":708},"attrs":{"0":null},"tokens":null}]},"span":{"lo":690,"hi":708}},{"id":129,"kind":{"variant":"Local","fields":[{"id":59,"pat":{"id":60,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"y","span":{"lo":717,"hi":718}},null]},"span":{"lo":717,"hi":718},"tokens":null},"ty":null,"init":{"id":61,"kind":{"variant":"Binary","fields":[{"node":"Add","span":{"lo":732,"hi":733}},{"id":62,"kind":{"variant":"Cast","fields":[{"id":63,"kind":{"variant":"Path","fields":[null,{"span":{"lo":721,"hi":722},"segments":[{"ident":{"name":"x","span":{"lo":721,"hi":722}},"id":64,"args":null}],"tokens":null}]},"span":{"lo":721,"hi":722},"attrs":{"0":null},"tokens":null},{"id":65,"kind":{"variant":"Path","fields":[null,{"span":{"lo":726,"hi":731},"segments":[{"ident":{"name":"i64","span":{"lo":726,"hi":729}},"id":66,"args":{"variant":"AngleBracketed","fields":[{"span":{"lo":729,"hi":731},"args":[]}]}}],"tokens":null}]},"span":{"lo":726,"hi":731},"tokens":null}]},"span":{"lo":721,"hi":731},"attrs":{"0":null},"tokens":null},{"id":67,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Integer","symbol":"5","suffix":null},"kind":{"variant":"Int","fields":[5,"Unsuffixed"]},"span":{"lo":734,"hi":735}}]},"span":{"lo":734,"hi":735},"attrs":{"0":null},"tokens":null}]},"span":{"lo":721,"hi":735},"attrs":{"0":null},"tokens":null},"span":{"lo":713,"hi":736},"attrs":{"0":null},"tokens":null}]},"span":{"lo":713,"hi":736}},{"id":130,"kind":{"variant":"Semi","fields":[{"id":75,"kind":{"variant":"Block","fields":[{"stmts":[{"id":125,"kind":{"variant":"Semi","fields":[{"id":77,"kind":{"variant":"Call","fields":[{"id":78,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25769,"hi":25787},"segments":[{"ident":{"name":"$crate","span":{"lo":25769,"hi":25775}},"id":79,"args":null},{"ident":{"name":"io","span":{"lo":25777,"hi":25779}},"id":80,"args":null},{"ident":{"name":"_print","span":{"lo":25781,"hi":25787}},"id":81,"args":null}],"tokens":null}]},"span":{"lo":25769,"hi":25787},"attrs":{"0":null},"tokens":null},[{"id":82,"kind":{"variant":"Call","fields":[{"id":83,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25788,"hi":25821},"segments":[{"ident":{"name":"$crate","span":{"lo":25788,"hi":25821}},"id":84,"args":null},{"ident":{"name":"fmt","span":{"lo":25788,"hi":25821}},"id":85,"args":null},{"ident":{"name":"Arguments","span":{"lo":25788,"hi":25821}},"id":86,"args":null},{"ident":{"name":"new_v1","span":{"lo":25788,"hi":25821}},"id":87,"args":null}],"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"id":88,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":89,"kind":{"variant":"Array","fields":[[{"id":90,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Str","symbol":"","suffix":null},"kind":{"variant":"Str","fields":["","Cooked"]},"span":{"lo":493,"hi":497}}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null},{"id":91,"kind":{"variant":"Lit","fields":[{"token":{"kind":"Str","symbol":"\\n","suffix":null},"kind":{"variant":"Str","fields":["\n","Cooked"]},"span":{"lo":493,"hi":497}}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null}]},"span":{"lo":493,"hi":497},"attrs":{"0":null},"tokens":null},{"id":92,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":93,"kind":{"variant":"Match","fields":[{"id":94,"kind":{"variant":"Tup","fields":[[{"id":95,"kind":{"variant":"AddrOf","fields":["Ref","Not",{"id":96,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"y","span":{"lo":756,"hi":757}},"id":97,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"attrs":{"0":null},"pat":{"id":99,"kind":{"variant":"Tuple","fields":[[{"id":100,"kind":{"variant":"Ident","fields":[{"variant":"ByValue","fields":["Not"]},{"name":"arg0","span":{"lo":499,"hi":501}},null]},"span":{"lo":499,"hi":501},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"tokens":null},"guard":null,"body":{"id":101,"kind":{"variant":"Array","fields":[[{"id":102,"kind":{"variant":"Call","fields":[{"id":103,"kind":{"variant":"Path","fields":[null,{"span":{"lo":25788,"hi":25821},"segments":[{"ident":{"name":"$crate","span":{"lo":25788,"hi":25821}},"id":104,"args":null},{"ident":{"name":"fmt","span":{"lo":25788,"hi":25821}},"id":105,"args":null},{"ident":{"name":"ArgumentV1","span":{"lo":25788,"hi":25821}},"id":106,"args":null},{"ident":{"name":"new","span":{"lo":25788,"hi":25821}},"id":107,"args":null}],"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},[{"id":108,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"arg0","span":{"lo":499,"hi":501}},"id":109,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null},{"id":110,"kind":{"variant":"Path","fields":[null,{"span":{"lo":499,"hi":501},"segments":[{"ident":{"name":"$crate","span":{"lo":499,"hi":501}},"id":111,"args":null},{"ident":{"name":"fmt","span":{"lo":499,"hi":501}},"id":112,"args":null},{"ident":{"name":"Display","span":{"lo":499,"hi":501}},"id":113,"args":null},{"ident":{"name":"fmt","span":{"lo":499,"hi":501}},"id":114,"args":null}],"tokens":null}]},"span":{"lo":499,"hi":501},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null},"span":{"lo":25788,"hi":25821},"id":98,"is_placeholder":false}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25788,"hi":25821},"attrs":{"0":null},"tokens":null}]]},"span":{"lo":25769,"hi":25822},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25769,"hi":25823}}],"id":76,"rules":"Default","span":{"lo":25759,"hi":25829},"tokens":null},null]},"span":{"lo":25759,"hi":25829},"attrs":{"0":null},"tokens":null}]},"span":{"lo":25759,"hi":25829}},{"id":131,"kind":{"variant":"Item","fields":[{"attrs":[],"id":68,"span":{"lo":765,"hi":774},"vis":{"kind":"Inherited","span":{"lo":765,"hi":765},"tokens":null},"ident":{"name":"A","span":{"lo":772,"hi":773}},"kind":{"variant":"Struct","fields":[{"variant":"Unit","fields":[69]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":773,"hi":773}},"span":{"lo":773,"hi":773}}]},"tokens":null}]},"span":{"lo":765,"hi":774}}],"id":49,"rules":"Default","span":{"lo":518,"hi":776},"tokens":null}}]},"tokens":null},{"attrs":[],"id":70,"span":{"lo":879,"hi":888},"vis":{"kind":"Inherited","span":{"lo":879,"hi":879},"tokens":null},"ident":{"name":"S","span":{"lo":886,"hi":887}},"kind":{"variant":"Struct","fields":[{"variant":"Unit","fields":[71]},{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":887,"hi":887}},"span":{"lo":887,"hi":887}}]},"tokens":null},{"attrs":[],"id":72,"span":{"lo":890,"hi":968},"vis":{"kind":"Inherited","span":{"lo":890,"hi":890},"tokens":null},"ident":{"name":"mac_extern","span":{"lo":903,"hi":913}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":914,"hi":915},"close":{"lo":967,"hi":968}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":920,"hi":921},"close":{"lo":928,"hi":929}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":921,"hi":922}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":922,"hi":923}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":923,"hi":924}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["item",false]},"span":{"lo":924,"hi":928}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":930,"hi":932}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":933,"hi":934},"close":{"lo":965,"hi":966}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":943,"hi":949}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"C","suffix":null}]},"span":{"lo":950,"hi":953}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":954,"hi":955},"close":{"lo":959,"hi":960}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":956,"hi":957}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":957,"hi":958}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":73,"span":{"lo":969,"hi":1067},"vis":{"kind":"Inherited","span":{"lo":969,"hi":969},"tokens":null},"ident":{"name":"mac_assoc","span":{"lo":982,"hi":991}},"kind":{"variant":"MacroDef","fields":[{"body":{"variant":"Delimited","fields":[{"open":{"lo":992,"hi":993},"close":{"lo":1066,"hi":1067}},"Brace",{"0":[[{"variant":"Delimited","fields":[{"open":{"lo":998,"hi":999},"close":{"lo":1006,"hi":1007}},"Paren",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":999,"hi":1000}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1000,"hi":1001}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Colon","span":{"lo":1001,"hi":1002}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["item",false]},"span":{"lo":1002,"hi":1006}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"FatArrow","span":{"lo":1008,"hi":1010}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1011,"hi":1012},"close":{"lo":1064,"hi":1065}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["impl",false]},"span":{"lo":1021,"hi":1025}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["S",false]},"span":{"lo":1026,"hi":1027}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1028,"hi":1029},"close":{"lo":1033,"hi":1034}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":1030,"hi":1031}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1031,"hi":1032}}]},"Alone"]]}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["trait",false]},"span":{"lo":1043,"hi":1048}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["Bar",false]},"span":{"lo":1049,"hi":1052}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1053,"hi":1054},"close":{"lo":1058,"hi":1059}},"Brace",{"0":[[{"variant":"Token","fields":[{"kind":"Dollar","span":{"lo":1055,"hi":1056}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["i",false]},"span":{"lo":1056,"hi":1057}}]},"Alone"]]}]},"Alone"]]}]},"Alone"]]}]},"macro_rules":true}]},"tokens":null},{"attrs":[],"id":115,"span":{"lo":943,"hi":960},"vis":{"kind":"Inherited","span":{"lo":943,"hi":943},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"ForeignMod","fields":[{"unsafety":"No","abi":{"style":"Cooked","symbol":"C","suffix":null,"span":{"lo":950,"hi":953},"symbol_unescaped":"C"},"items":[{"attrs":[],"id":116,"span":{"lo":1087,"hi":1096},"vis":{"kind":"Inherited","span":{"lo":1087,"hi":1087},"tokens":null},"ident":{"name":"foo","span":{"lo":1090,"hi":1093}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1095,"hi":1095}]}},"span":{"lo":1087,"hi":1096}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1095,"hi":1095}},"span":{"lo":1093,"hi":1093}},"3":null}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1087,"hi":1089}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1090,"hi":1093}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1093,"hi":1094},"close":{"lo":1094,"hi":1095}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":1095,"hi":1096}}]},"Alone"]]}}]}]},"tokens":null},{"attrs":[],"id":117,"span":{"lo":1021,"hi":1034},"vis":{"kind":"Inherited","span":{"lo":1021,"hi":1021},"tokens":null},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Impl","fields":[{"unsafety":"No","polarity":"Positive","defaultness":"Final","constness":"No","generics":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1027,"hi":1027}},"span":{"lo":1025,"hi":1025}},"of_trait":null,"self_ty":{"id":118,"kind":{"variant":"Path","fields":[null,{"span":{"lo":1026,"hi":1027},"segments":[{"ident":{"name":"S","span":{"lo":1026,"hi":1027}},"id":119,"args":null}],"tokens":null}]},"span":{"lo":1026,"hi":1027},"tokens":null},"items":[{"attrs":[],"id":120,"span":{"lo":1116,"hi":1127},"vis":{"kind":"Inherited","span":{"lo":1116,"hi":1116},"tokens":null},"ident":{"name":"foo","span":{"lo":1119,"hi":1122}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1125,"hi":1125}]}},"span":{"lo":1116,"hi":1124}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1124,"hi":1124}},"span":{"lo":1122,"hi":1122}},"3":{"stmts":[],"id":121,"rules":"Default","span":{"lo":1125,"hi":1127},"tokens":null}}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1116,"hi":1118}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1119,"hi":1122}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1122,"hi":1123},"close":{"lo":1123,"hi":1124}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1125,"hi":1126},"close":{"lo":1126,"hi":1127}},"Brace",{"0":[]}]},"Alone"]]}}]}]},"tokens":null},{"attrs":[],"id":122,"span":{"lo":1043,"hi":1059},"vis":{"kind":"Inherited","span":{"lo":1043,"hi":1043},"tokens":null},"ident":{"name":"Bar","span":{"lo":1049,"hi":1052}},"kind":{"variant":"Trait","fields":[{"0":"No","1":"No","2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1052,"hi":1052}},"span":{"lo":1052,"hi":1052}},"3":[],"4":[{"attrs":[],"id":123,"span":{"lo":1116,"hi":1127},"vis":{"kind":"Inherited","span":{"lo":1116,"hi":1116},"tokens":null},"ident":{"name":"foo","span":{"lo":1119,"hi":1122}},"kind":{"variant":"Fn","fields":[{"0":"Final","1":{"header":{"unsafety":"No","asyncness":"No","constness":"No","ext":"None"},"decl":{"inputs":[],"output":{"variant":"Default","fields":[{"lo":1125,"hi":1125}]}},"span":{"lo":1116,"hi":1124}},"2":{"params":[],"where_clause":{"has_where_token":false,"predicates":[],"span":{"lo":1124,"hi":1124}},"span":{"lo":1122,"hi":1122}},"3":{"stmts":[],"id":124,"rules":"Default","span":{"lo":1125,"hi":1127},"tokens":null}}]},"tokens":{"0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["fn",false]},"span":{"lo":1116,"hi":1118}}]},"Alone"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["foo",false]},"span":{"lo":1119,"hi":1122}}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1122,"hi":1123},"close":{"lo":1123,"hi":1124}},"Paren",{"0":[]}]},"Alone"],[{"variant":"Delimited","fields":[{"open":{"lo":1125,"hi":1126},"close":{"lo":1126,"hi":1127}},"Brace",{"0":[]}]},"Alone"]]}}]}]},"tokens":null}],"span":{"lo":255,"hi":1129},"proc_macros":[]}
thread '[ui] ui/ast-json/ast-json-ice.rs#expand' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:123:21

---- [ui] ui/borrowck/borrowck-asm.rs stdout ----
diff of stderr:


+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : : "r"(x));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : : "r"(x));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : "=r"(x));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : "=r"(a));  // OK, Shallow write to `a`
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : "+r"(x));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : "=*r"(x));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : "+r"(x));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |             llvm_asm!("nop" : : "r"(x), "r"(x) );
+ 
+ 
1 error[E0382]: use of moved value: `x`
3    |

75    |                                     |
76    |                                     value moved here
---
To only update this specific test, also pass `--test-args borrowck/borrowck-asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : : "r"(x));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : : "r"(x)); //~ ERROR cannot use


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : "=r"(x));  //~ ERROR cannot assign twice


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : "=r"(a));  // OK, Shallow write to `a`


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : "+r"(x));  //~ ERROR cannot assign twice


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : "=*r"(x)); //~ ERROR use of possibly-uninitialized variable


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : "+r"(x));  //~ ERROR cannot assign to `x` because it is borrowed


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("nop" : : "r"(x), "r"(x) );    //~ ERROR use of moved value


error[E0382]: use of moved value: `x`
   |
LL |         let x = &mut 0isize;
LL |         let x = &mut 0isize;
   |             - move occurs because `x` has type `&mut isize`, which does not implement the `Copy` trait
LL |         unsafe {
LL |             llvm_asm!("nop" : : "r"(x));
   |                                     - value moved here
LL |         }
LL |         let z = x;  //~ ERROR use of moved value: `x`
   |                 ^ value used here after move

error[E0503]: cannot use `x` because it was mutably borrowed
   |
   |
LL |         let y = &mut x;
   |                 ------ borrow of `x` occurs here
LL |         unsafe {
LL |             llvm_asm!("nop" : : "r"(x)); //~ ERROR cannot use
   |                                     ^ use of borrowed `x`
LL |         }
LL |         let z = y;
   |                 - borrow later used here
error[E0384]: cannot assign twice to immutable variable `x`
  --> /checkout/src/test/ui/borrowck/borrowck-asm.rs:40:36
   |
LL |         let x = 3;
LL |         let x = 3;
   |             -
   |             |
   |             first assignment to `x`
   |             help: consider making this binding mutable: `mut x`
LL |         unsafe {
LL |             llvm_asm!("nop" : "=r"(x));  //~ ERROR cannot assign twice
   |                                    ^ cannot assign twice to immutable variable
error[E0384]: cannot assign twice to immutable variable `x`
  --> /checkout/src/test/ui/borrowck/borrowck-asm.rs:54:36
   |
LL |         let x = 3;
LL |         let x = 3;
   |             -
   |             |
   |             first assignment to `x`
   |             help: consider making this binding mutable: `mut x`
LL |         unsafe {
LL |             llvm_asm!("nop" : "+r"(x));  //~ ERROR cannot assign twice
   |                                    ^ cannot assign twice to immutable variable

error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |             llvm_asm!("nop" : "=*r"(x)); //~ ERROR use of possibly-uninitialized variable
   |                                     ^ use of possibly-uninitialized `x`

error[E0506]: cannot assign to `x` because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of `x` occurs here
LL |         unsafe {
LL |             llvm_asm!("nop" : "+r"(x));  //~ ERROR cannot assign to `x` because it is borrowed
   |                                    ^ assignment to borrowed `x` occurs here
LL |         }
LL |         let z = y;
   |                 - borrow later used here

error[E0382]: use of moved value: `x`
   |
LL |         let x = &mut 2;
LL |         let x = &mut 2;
   |             - move occurs because `x` has type `&mut i32`, which does not implement the `Copy` trait
LL |         unsafe {
LL |             llvm_asm!("nop" : : "r"(x), "r"(x) );    //~ ERROR use of moved value
   |                                     -       ^ value used here after move
   |                                     value moved here

error: aborting due to 7 previous errors; 8 warnings emitted

---

---- [ui] ui/consts/inline_asm.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL | const _: () = unsafe { llvm_asm!("nop") };
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0015]: inline assembly is not allowed in constants
1 error[E0015]: inline assembly is not allowed in constants
2   --> $DIR/inline_asm.rs:3:24
3    |

6    |
7    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
10 
11 For more information about this error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args consts/inline_asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/inline_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/inline_asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL | const _: () = unsafe { llvm_asm!("nop") };
   |
   = note: `#[warn(deprecated)]` on by default

error[E0015]: inline assembly is not allowed in constants
error[E0015]: inline assembly is not allowed in constants
  --> /checkout/src/test/ui/consts/inline_asm.rs:3:24
   |
LL | const _: () = unsafe { llvm_asm!("nop") };
   |
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0015`.


------------------------------------------


---- [ui] ui/consts/miri_unleashed/inline_asm.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0080]: could not evaluate static initializer
1 error[E0080]: could not evaluate static initializer
2   --> $DIR/inline_asm.rs:10:14
3    |

26    |              ^^^^^^^^^^^^
27    = note: this warning originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 2 previous errors; 1 warning emitted
+ error: aborting due to 2 previous errors; 2 warnings emitted
30 
31 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/miri_unleashed/inline_asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
   |
   = note: `#[warn(deprecated)]` on by default

error[E0080]: could not evaluate static initializer
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
   |
LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ inline assembly is not supported
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:19:14
   |
   |
LL |     unsafe { asm!("nop"); }
   |              ^^^^^^^^^^^^ inline assembly is not supported
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
  --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
   |
LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:19:14
   |
   |
LL |     unsafe { asm!("nop"); }
   |              ^^^^^^^^^^^^
   = note: this warning originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/error-codes/E0661.rs stdout ----
diff of stderr:

4 LL |     llvm_asm!("nop" : "r"(a));
6 
6 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0661.rs:7:5
+    |
+ LL |     llvm_asm!("nop" : "r"(a));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
7 error[E0282]: type annotations needed
7 error[E0282]: type annotations needed
8   --> $DIR/E0661.rs:6:9
9    |

10 LL |     let a;
11    |         ^ consider giving `a` a type
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
14 
15 Some errors have detailed explanations: E0282, E0661.
---
To only update this specific test, also pass `--test-args error-codes/E0661.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0661.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0661" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0661/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0661]: output operand constraint lacks '=' or '+'
   |
   |
LL |     llvm_asm!("nop" : "r"(a));


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop" : "r"(a));
   |
   = note: `#[warn(deprecated)]` on by default

error[E0282]: type annotations needed
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0661.rs:6:9
   |
LL |     let a; //~ ERROR type annotations needed
   |         ^ consider giving `a` a type
error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0282, E0661.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.

------------------------------------------


---- [ui] ui/error-codes/E0660.rs stdout ----
diff of stderr:

10 LL |     llvm_asm!("nop" "nop" : "=r"(a));
12 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0660.rs:5:5
+    |
+ LL |     llvm_asm!("nop" "nop");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0660.rs:7:5
+    |
+ LL |     llvm_asm!("nop" "nop" : "=r"(a));
+ 
+ error: aborting due to 2 previous errors; 2 warnings emitted
14 
15 For more information about this error, try `rustc --explain E0660`.
---
To only update this specific test, also pass `--test-args error-codes/E0660.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0660.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0660" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0660/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0660]: malformed inline assembly
  --> /checkout/src/test/ui/error-codes/E0660.rs:5:5
   |
LL |     llvm_asm!("nop" "nop");

error[E0660]: malformed inline assembly
  --> /checkout/src/test/ui/error-codes/E0660.rs:7:5
   |
   |
LL |     llvm_asm!("nop" "nop" : "=r"(a));


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop" "nop");
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop" "nop" : "=r"(a));

error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0660`.
For more information about this error, try `rustc --explain E0660`.

------------------------------------------


---- [ui] ui/error-codes/E0662.rs stdout ----
diff of stderr:

4 LL |               : "=test"("a")
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0662.rs:6:5
+    |
+ LL |     llvm_asm!("xor %eax, %eax"
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args error-codes/E0662.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0662" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0662/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0662]: input operand constraint contains '='
   |
   |
LL |               : "=test"("a") //~ ERROR E0662


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("xor %eax, %eax"
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
---

---- [ui] ui/error-codes/E0663.rs stdout ----
diff of stderr:

4 LL |               : "+test"("a")
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0663.rs:6:5
+    |
+ LL |     llvm_asm!("xor %eax, %eax"
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args error-codes/E0663.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0663.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0663" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0663/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0663]: input operand constraint contains '+'
   |
   |
LL |               : "+test"("a") //~ ERROR E0663


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("xor %eax, %eax"
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
---

---- [ui] ui/error-codes/E0664.rs stdout ----
diff of stderr:

4 LL |               : "{eax}"
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+   --> $DIR/E0664.rs:6:5
+    |
+ LL |     llvm_asm!("mov $$0x200, %eax"
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args error-codes/E0664.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0664.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0664" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0664/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0664]: clobber should not be surrounded by braces
  --> /checkout/src/test/ui/error-codes/E0664.rs:9:17
   |
LL |               : "{eax}" //~ ERROR E0664


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov $$0x200, %eax"
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
---
---- [ui] ui/feature-gates/feature-gate-asm.rs stdout ----
diff of stderr:

16    = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
17    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to 2 previous errors; 1 warning emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
20 
21 For more information about this error, try `rustc --explain E0658`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/feature-gate-asm.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
   |
LL |         asm!("");
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'llvm_asm': prefer using the new asm! syntax instead
   |
   |
LL |         llvm_asm!("");
   |
   = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
   = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
   = help: add `#![feature(llvm_asm)]` to the crate attributes to enable

warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("");
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted
---
---- [ui] ui/feature-gates/feature-gate-asm2.rs stdout ----
diff of stderr:

16    = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
17    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         println!("{:?}", llvm_asm!(""));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to 2 previous errors; 1 warning emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
20 
21 For more information about this error, try `rustc --explain E0658`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/feature-gate-asm2.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
   |
LL |         println!("{:?}", asm!(""));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'llvm_asm': prefer using the new asm! syntax instead
   |
   |
LL |         println!("{:?}", llvm_asm!(""));
   |
   = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
   = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
   = help: add `#![feature(llvm_asm)]` to the crate attributes to enable

warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         println!("{:?}", llvm_asm!(""));
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted
---

---- [ui] ui/issues/issue-23458.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("int $3");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error: invalid operand in inline asm: 'int $3'
---
To only update this specific test, also pass `--test-args issues/issue-23458.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ccodegen-units=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("int $3"); //~ ERROR too few operands for instruction
   |
   = note: `#[warn(deprecated)]` on by default

error: invalid operand in inline asm: 'int $3'
error: invalid operand in inline asm: 'int $3'
  --> /checkout/src/test/ui/issues/issue-23458.rs:8:9
   |
LL |         llvm_asm!("int $3"); //~ ERROR too few operands for instruction


error: too few operands for instruction
   |
   |
LL |         llvm_asm!("int $3"); //~ ERROR too few operands for instruction
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     int 
   |     ^

error: aborting due to 2 previous errors; 1 warning emitted
error: aborting due to 2 previous errors; 1 warning emitted


------------------------------------------


---- [ui] ui/issues/issue-37366.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("pop  eax" :::: "intel");
...
...
LL | interrupt_handler!{}
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `interrupt_handler` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args issues/issue-37366.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37366.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37366" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37366/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("pop  eax" :::: "intel");
...
...
LL | interrupt_handler!{}
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `interrupt_handler` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-37433.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("" :: "r"(""));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0669]: invalid value for constraint in inline assembly
1 error[E0669]: invalid value for constraint in inline assembly
2   --> $DIR/issue-37433.rs:8:29
3    |

4 LL |         llvm_asm!("" :: "r"(""));
6 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
8 
---
To only update this specific test, also pass `--test-args issues/issue-37433.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("" :: "r"(""));
   |
   = note: `#[warn(deprecated)]` on by default

error[E0669]: invalid value for constraint in inline assembly
error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/issues/issue-37433.rs:8:29
   |
LL |         llvm_asm!("" :: "r"(""));

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0669`.
For more information about this error, try `rustc --explain E0669`.

------------------------------------------


---- [ui] ui/issues/issue-53787-inline-assembler-macro.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+ LL |             llvm_asm!(
+    |             ^^^^^^^^
+ ...
+ ...
+ LL |     fake_jump!("FirstFunc");
+    |
+    = note: `#[warn(deprecated)]` on by default
+    = note: `#[warn(deprecated)]` on by default
+    = note: this warning originates in the macro `fake_jump` (in Nightly builds, run with -Z macro-backtrace for more info)
1 error[E0669]: invalid value for constraint in inline assembly
2   --> $DIR/issue-53787-inline-assembler-macro.rs:24:16
3    |


4 LL |     fake_jump!("FirstFunc");
6 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
8 
---
To only update this specific test, also pass `--test-args issues/issue-53787-inline-assembler-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53787-inline-assembler-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53787-inline-assembler-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53787-inline-assembler-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |             llvm_asm!(
   |             ^^^^^^^^
...
...
LL |     fake_jump!("FirstFunc"); //~ ERROR invalid value for constraint in inline assembly
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `fake_jump` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/issues/issue-53787-inline-assembler-macro.rs:24:16
   |
   |
LL |     fake_jump!("FirstFunc"); //~ ERROR invalid value for constraint in inline assembly

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0669`.
For more information about this error, try `rustc --explain E0669`.

------------------------------------------


---- [ui] ui/llvm-asm/issue-54067.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r" (addr));
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/issue-54067.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-54067.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-54067" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-54067/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r" (addr));
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted


------------------------------------------


---- [ui] ui/llvm-asm/asm-src-loc-codegen-units.rs stdout ----

error: /checkout/src/test/ui/llvm-asm/asm-src-loc-codegen-units.rs:10: unexpected warning: '10:9: 10:17: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead [deprecated]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/asm-src-loc-codegen-units.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc-codegen-units" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "codegen-units=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc-codegen-units/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "10:9: 10:17: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead [deprecated]",
]

thread '[ui] ui/llvm-asm/asm-src-loc-codegen-units.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1532:13


---- [ui] ui/llvm-asm/issue-51431.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm! {"mov $0,$1"::"0"("bx"),"1"(0x00)}
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0669]: invalid value for constraint in inline assembly
1 error[E0669]: invalid value for constraint in inline assembly
2   --> $DIR/issue-51431.rs:8:37
3    |

4 LL |         llvm_asm! {"mov $0,$1"::"0"("bx"),"1"(0x00)}
6 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
8 
---
To only update this specific test, also pass `--test-args llvm-asm/issue-51431.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-51431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-51431" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-51431/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm! {"mov $0,$1"::"0"("bx"),"1"(0x00)}
   |
   = note: `#[warn(deprecated)]` on by default

error[E0669]: invalid value for constraint in inline assembly
error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/issue-51431.rs:8:37
   |
LL |         llvm_asm! {"mov $0,$1"::"0"("bx"),"1"(0x00)}

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0669`.
For more information about this error, try `rustc --explain E0669`.

------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-in-bad-modifier.rs stdout ----
diff of stderr:

10 LL |         llvm_asm!("mov $1, $0" : "=r"(y) : "+r"(5));
12 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "=r"(5));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "=r"(y) : "+r"(5));
+ 
+ error: aborting due to 2 previous errors; 2 warnings emitted
14 
15 Some errors have detailed explanations: E0662, E0663.
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-in-bad-modifier.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-in-bad-modifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-bad-modifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-bad-modifier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0662]: input operand constraint contains '='
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "=r"(5)); //~ ERROR operand constraint contains '='


error[E0663]: input operand constraint contains '+'
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(y) : "+r"(5)); //~ ERROR operand constraint contains '+'


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "=r"(5)); //~ ERROR operand constraint contains '='
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(y) : "+r"(5)); //~ ERROR operand constraint contains '+'

error: aborting due to 2 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0662, E0663.
---

---- [ui] ui/llvm-asm/inline-asm-bad-constraint.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("" :"={rax"(rax))
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("callq $0" : : "0"(foo))
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("addb $1, $0" : "={rax}"((0i32, rax)));
+ 
1 error[E0668]: malformed inline assembly
2   --> $DIR/inline-asm-bad-constraint.rs:22:9
3    |
3    |

22    |
23    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 3 previous errors
+ error: aborting due to 3 previous errors; 3 warnings emitted
26 
27 For more information about this error, try `rustc --explain E0668`.
---
To only update this specific test, also pass `--test-args llvm-asm/inline-asm-bad-constraint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/inline-asm-bad-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/inline-asm-bad-constraint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/inline-asm-bad-constraint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("" :"={rax"(rax)) //~ ERROR E0668
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("callq $0" : : "0"(foo)) //~ ERROR E0668


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("addb $1, $0" : "={rax}"((0i32, rax))); //~ ERROR E0668

error[E0668]: malformed inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-constraint.rs:22:9
   |
   |
LL |         llvm_asm!("" :"={rax"(rax)) //~ ERROR E0668
   |
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0668]: malformed inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-constraint.rs:30:9
   |
   |
LL |         llvm_asm!("callq $0" : : "0"(foo)) //~ ERROR E0668
   |
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0668]: malformed inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-constraint.rs:37:9
   |
   |
LL |         llvm_asm!("addb $1, $0" : "={rax}"((0i32, rax))); //~ ERROR E0668
   |
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0668`.


------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-bad-clobber.rs stdout ----
diff of stderr:

4 LL |         llvm_asm!("xor %eax, %eax" : : : "{eax}");
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("xor %eax, %eax" : : : "{eax}");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-bad-clobber.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-bad-clobber.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-bad-clobber" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-bad-clobber/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0664]: clobber should not be surrounded by braces
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-bad-clobber.rs:23:42
   |
LL |         llvm_asm!("xor %eax, %eax" : : : "{eax}");


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("xor %eax, %eax" : : : "{eax}");
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
---


---- [ui] ui/llvm-asm/asm-src-loc.rs stdout ----

error: /checkout/src/test/ui/llvm-asm/asm-src-loc.rs:9: unexpected warning: '9:9: 9:17: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead [deprecated]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/asm-src-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:9: 9:17: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead [deprecated]",
]

thread '[ui] ui/llvm-asm/asm-src-loc.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1532:13


---- [ui] ui/llvm-asm/inline-asm-bad-operand.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("" :: "r"(""));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("ret" : : "{rdi}"(target));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     unsafe { llvm_asm!("" :: "i"(hello)) };
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     unsafe { llvm_asm!("" :: "r"(hello.as_ptr())) };
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("movups $1, %xmm0"::"m"(arr));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov sp, $0"::"r"(addr));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov sp, $0"::"r"(addr),
+ 
1 error[E0669]: invalid value for constraint in inline assembly
2   --> $DIR/inline-asm-bad-operand.rs:22:29
3    |
3    |

40 LL | ...                   "r"("hello e0669"));
42 
- error: aborting due to 7 previous errors
+ error: aborting due to 7 previous errors; 7 warnings emitted
44 
---
To only update this specific test, also pass `--test-args llvm-asm/inline-asm-bad-operand.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/inline-asm-bad-operand" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/inline-asm-bad-operand/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("" :: "r"("")); //~ ERROR E0669
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("ret" : : "{rdi}"(target)); //~ ERROR E0669


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!("" :: "i"(hello)) }; //~ ERROR E0669


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!("" :: "r"(hello.as_ptr())) };


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("movups $1, %xmm0"::"m"(arr)); //~ ERROR E0669


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r"(addr)); //~ ERROR E0669


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r"(addr), //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:22:29
   |
   |
LL |         llvm_asm!("" :: "r"("")); //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:27:37
   |
   |
LL |         llvm_asm!("ret" : : "{rdi}"(target)); //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:34:34
   |
   |
LL |     unsafe { llvm_asm!("" :: "i"(hello)) }; //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:42:43
   |
   |
LL |         llvm_asm!("movups $1, %xmm0"::"m"(arr)); //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:49:37
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r"(addr)); //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:56:37
   |
   |
LL |         llvm_asm!("mov sp, $0"::"r"(addr), //~ ERROR E0669

error[E0669]: invalid value for constraint in inline assembly
  --> /checkout/src/test/ui/llvm-asm/inline-asm-bad-operand.rs:57:37
   |
   |
LL | ...                   "r"("hello e0669")); //~ ERROR E0669

error: aborting due to 7 previous errors; 7 warnings emitted

For more information about this error, try `rustc --explain E0669`.
For more information about this error, try `rustc --explain E0669`.

------------------------------------------


---- [ui] ui/llvm-asm/issue-62046.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("nop" : "+r"("r15"));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0668]: malformed inline assembly
1 error[E0668]: malformed inline assembly
2   --> $DIR/issue-62046.rs:8:9
3    |

6    |
7    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
10 
11 For more information about this error, try `rustc --explain E0668`.
---
To only update this specific test, also pass `--test-args llvm-asm/issue-62046.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-62046.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-62046" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-62046/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("nop" : "+r"("r15"));
   |
   = note: `#[warn(deprecated)]` on by default

error[E0668]: malformed inline assembly
error[E0668]: malformed inline assembly
  --> /checkout/src/test/ui/llvm-asm/issue-62046.rs:8:9
   |
LL |         llvm_asm!("nop" : "+r"("r15"));
   |
   |
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0668`.


------------------------------------------


---- [ui] ui/llvm-asm/issue-33264.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |             llvm_asm!("
   |             ^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args llvm-asm/issue-33264.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-33264.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-33264" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-33264/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |             llvm_asm!("
   |             ^^^^^^^^
   |
---

---- [ui] ui/llvm-asm/issue-69092.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     unsafe { llvm_asm!(".ascii \"Xen\0\""); }
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
1 error: expected string in '.ascii' directive
3    |


10 LL |     .ascii "Xen
12 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
14 
---
To only update this specific test, also pass `--test-args llvm-asm/issue-69092.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-69092.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-69092" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-69092/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!(".ascii \"Xen\0\""); }
   |
   = note: `#[warn(deprecated)]` on by default


error: expected string in '.ascii' directive
   |
   |
LL |     unsafe { llvm_asm!(".ascii \"Xen\0\""); }
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:9
   |
LL |     .ascii "Xen

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-misplaced-option.rs stdout ----
diff of stderr:

10 LL |         llvm_asm!("add $2, $1; mov $1, $0" : "=r"(x) : "r"(x), "r"(8_usize) : "cc", "volatile");
12 
- warning: 2 warnings emitted
- warning: 2 warnings emitted
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5_usize), "0"(x) : : "cc");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("add $2, $1; mov $1, $0" : "=r"(x) : "r"(x), "r"(8_usize) : "cc", "volatile");
+ 
+ warning: 4 warnings emitted
14 
15 
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-misplaced-option.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-misplaced-option.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-misplaced-option" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-misplaced-option/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unrecognized option
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-misplaced-option.rs:25:69
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5_usize), "0"(x) : : "cc");

warning: expected a clobber, found an option
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-misplaced-option.rs:32:85
   |
   |
LL |         llvm_asm!("add $2, $1; mov $1, $0" : "=r"(x) : "r"(x), "r"(8_usize) : "cc", "volatile");


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5_usize), "0"(x) : : "cc");
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("add $2, $1; mov $1, $0" : "=r"(x) : "r"(x), "r"(8_usize) : "cc", "volatile");

warning: 4 warnings emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-out-assign-imm.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
1 error[E0384]: cannot assign twice to immutable variable `x`
1 error[E0384]: cannot assign twice to immutable variable `x`
2   --> $DIR/llvm-asm-out-assign-imm.rs:25:39
3    |

9 LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5));
10    |                                       ^ cannot assign twice to immutable variable
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
13 
14 For more information about this error, try `rustc --explain E0384`.
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-out-assign-imm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-out-assign-imm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-assign-imm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-assign-imm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5));
   |
   = note: `#[warn(deprecated)]` on by default

error[E0384]: cannot assign twice to immutable variable `x`
error[E0384]: cannot assign twice to immutable variable `x`
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-out-assign-imm.rs:25:39
   |
LL |     let x: isize;
   |         - help: consider making this binding mutable: `mut x`
LL |     x = 1;
   |     ----- first assignment to `x`
...
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5));
   |                                       ^ cannot assign twice to immutable variable
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0384`.


------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-out-read-uninit.rs stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
1 error[E0381]: use of possibly-uninitialized variable: `x`
3    |


4 LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x));
5    |                                                ^ use of possibly-uninitialized `x`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
8 
9 For more information about this error, try `rustc --explain E0381`.
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-out-read-uninit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-out-read-uninit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-read-uninit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-read-uninit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x));
   |
   = note: `#[warn(deprecated)]` on by default


error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x));
   |                                                ^ use of possibly-uninitialized `x`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0381`.


------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-out-no-modifier.rs stdout ----
diff of stderr:

4 LL |         llvm_asm!("mov $1, $0" : "r"(x) : "r"(5));
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |         llvm_asm!("mov $1, $0" : "r"(x) : "r"(5));
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-out-no-modifier.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-out-no-modifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-no-modifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-no-modifier/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0661]: output operand constraint lacks '=' or '+'
   |
   |
LL |         llvm_asm!("mov $1, $0" : "r"(x) : "r"(5)); //~ ERROR output operand constraint lacks '='


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "r"(x) : "r"(5)); //~ ERROR output operand constraint lacks '='
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
---

---- [ui] ui/llvm-asm/llvm-asm-parse-errors.rs stdout ----
diff of stderr:

64 LL |     llvm_asm!(123);
66 
- error: aborting due to 11 previous errors
- error: aborting due to 11 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!();
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("nop" : struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("mov %eax, $$0x2" : struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}" struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}"(struct));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}" struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}"(struct));
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("mov $$0x200, %eax" : : : struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("mov eax, 2" : "={eax}"(foo) : : : struct);
+ 
+ 
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!(123);
+ 
+ error: aborting due to 11 previous errors; 11 warnings emitted
68 
69 
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-parse-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-parse-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-parse-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro requires a string literal as an argument
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:4:5
   |
LL |     llvm_asm!(); //~ ERROR requires a string literal as an argument
   |     ^^^^^^^^^^^^ string literal required
error: expected string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:5:23
   |
   |
LL |     llvm_asm!("nop" : struct); //~ ERROR expected string literal

error: expected string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:6:35
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : struct); //~ ERROR expected string literal


error: expected `(`, found keyword `struct`
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}" struct); //~ ERROR expected `(`
   |                                            ^^^^^^ expected `(`
error: expected expression, found keyword `struct`
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:8:44
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}"(struct)); //~ ERROR expected expression

error: expected string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:9:49
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : struct); //~ ERROR expected string literal


error: expected `(`, found keyword `struct`
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}" struct); //~ ERROR expected `(`
   |                                                        ^^^^^^ expected `(`
error: expected expression, found keyword `struct`
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:11:56
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}"(struct)); //~ ERROR expected expression

error: expected string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:12:41
   |
   |
LL |     llvm_asm!("mov $$0x200, %eax" : : : struct); //~ ERROR expected string literal

error: expected string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:13:50
   |
   |
LL |     llvm_asm!("mov eax, 2" : "={eax}"(foo) : : : struct); //~ ERROR expected string literal

error: inline assembly must be a string literal
  --> /checkout/src/test/ui/llvm-asm/llvm-asm-parse-errors.rs:14:15
   |
   |
LL |     llvm_asm!(123); //~ ERROR inline assembly must be a string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!(); //~ ERROR requires a string literal as an argument
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop" : struct); //~ ERROR expected string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : struct); //~ ERROR expected string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}" struct); //~ ERROR expected `(`


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov %eax, $$0x2" : "={eax}"(struct)); //~ ERROR expected expression


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : struct); //~ ERROR expected string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}" struct); //~ ERROR expected `(`


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("in %dx, %al" : "={al}"(result) : "{dx}"(struct)); //~ ERROR expected expression


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov $$0x200, %eax" : : : struct); //~ ERROR expected string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("mov eax, 2" : "={eax}"(foo) : : : struct); //~ ERROR expected string literal


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!(123); //~ ERROR inline assembly must be a string literal

error: aborting due to 11 previous errors; 11 warnings emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-concat-src.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!(concat!("", "")) };
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-concat-src.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-concat-src.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-concat-src/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-concat-src/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     unsafe { llvm_asm!(concat!("", "")) };
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted


------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-literal-escaping.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("\x6Eop" :: "\x72"(x) : "\x65ax" : "\x76olatile");
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-literal-escaping.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-literal-escaping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-literal-escaping" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-literal-escaping/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("\x6Eop" :: "\x72"(x) : "\x65ax" : "\x76olatile");
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted


------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-indirect-memory.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r" (out) : "*m" (ptr));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=*m" (ptr) : "r" (val));


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $0, $1; mov $2, $0" : "+*m" (ptr), "=&r" (out) : "r" (val));

warning: 3 warnings emitted


---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-indirect-memory.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-indirect-memory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-indirect-memory/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-indirect-memory/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r" (out) : "*m" (ptr));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=*m" (ptr) : "r" (val));


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $0, $1; mov $2, $0" : "+*m" (ptr), "=&r" (out) : "r" (val));

warning: 3 warnings emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-out-assign.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5_usize));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x + 7));

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-out-assign.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-out-assign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-assign/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-out-assign/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(5_usize));
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("mov $1, $0" : "=r"(x) : "r"(x + 7));

warning: 2 warnings emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-in-out-operand.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("dec $0" : "+rm"(tmp) :: "cc");
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("inc $0" : "+rm"(tmp) :: "cc");


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |         llvm_asm!(
   |         ^^^^^^^^


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |         llvm_asm!(
   |         ^^^^^^^^


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("shr $$2, $1; add $1, $0" : "+&r"(x) : "r"(x) : "cc");

warning: 5 warnings emitted


---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-in-out-operand.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-in-out-operand.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-out-operand/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-out-operand/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("dec $0" : "+rm"(tmp) :: "cc");
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("inc $0" : "+rm"(tmp) :: "cc");


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |         llvm_asm!(
   |         ^^^^^^^^


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
LL |         llvm_asm!(
   |         ^^^^^^^^


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |         llvm_asm!("shr $$2, $1; add $1, $0" : "+&r"(x) : "r"(x) : "cc");

warning: 5 warnings emitted



------------------------------------------


---- [ui] ui/llvm-asm/llvm-asm-in-moved.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("mov $1, $0" : "=r"(_y) : "r"(x));
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args llvm-asm/llvm-asm-in-moved.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/llvm-asm-in-moved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-moved/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/llvm-asm-in-moved/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |             llvm_asm!("mov $1, $0" : "=r"(_y) : "r"(x));
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted


------------------------------------------


---- [ui] ui/llvm-asm/issue-14936.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |                 llvm_asm!("mov ($1), $0"
...
...
LL |         demo!("=r")
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `demo` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |                 llvm_asm!("mov ($1), $0"
...
...
LL |         demo!("+r")
   |
   |
   = note: this warning originates in the macro `demo` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args llvm-asm/issue-14936.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/issue-14936.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-14936" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/issue-14936/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |                 llvm_asm!("mov ($1), $0"
...
...
LL |         demo!("=r")
   |
   = note: `#[warn(deprecated)]` on by default
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `demo` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |                 llvm_asm!("mov ($1), $0"
...
...
LL |         demo!("+r")
   |
   |
   = note: this warning originates in the macro `demo` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/macros/macros-nonfatal-errors.rs stdout ----
diff of stderr:

221 LL |     trace_macros!(invalid);
223 
- error: aborting due to 27 previous errors
- error: aborting due to 27 previous errors
+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!(invalid);
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to 27 previous errors; 1 warning emitted
---
To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | struct DefaultInnerAttrTupleStruct(#[default] ());


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo = #[default] 0, //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Bar([u8; #[default] 1]), //~ ERROR the `#[default]` attribute may only be used on unit enum variants

error: no default declared
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:42:10
   |
   |
LL | #[derive(Default)] //~ ERROR no default declared
   |
   |
   = help: make a unit variant default by placing `#[default]` above it
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
error: multiple declared defaults
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:48:10
   |
   |
LL | #[derive(Default)] //~ ERROR multiple declared defaults
...
LL |     Foo,
   |     --- first default
LL |     #[default]
LL |     #[default]
LL |     Bar,
   |     --- additional default
LL |     #[default]
LL |     Baz,
   |     --- additional default
   |
   = note: only one variant can be default
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `#[default]` attribute does not accept a value
   |
   |
LL |     #[default = 1] //~ ERROR `#[default]` attribute does not accept a value
   |
   |
   = help: try using `#[default]`

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:67:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
...
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:75:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^

error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo {}, //~ ERROR the `#[default]` attribute may only be used on unit enum variants
   |
   |
   = help: consider a manual implementation of `Default`

error: default variant must be exhaustive
   |
   |
LL |     #[non_exhaustive]
   |     ----------------- declared `#[non_exhaustive]` here
LL |     Foo, //~ ERROR default variant must be exhaustive
   |
   |
   = help: consider a manual implementation of `Default`
error: asm template must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:98:10
   |
   |
LL |     asm!(invalid); //~ ERROR

error: inline assembly must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:99:15
   |
   |
LL |     llvm_asm!(invalid); //~ ERROR


error: concat_idents! requires ident args.
   |
   |
LL |     concat_idents!("not", "idents"); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:103:17
   |
   |
LL |     option_env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:104:10
   |
   |
LL |     env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:105:10
   |
   |
LL |     env!(foo, abr, baz); //~ ERROR


error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
   |
   |
LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
   |
   = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)

error: format argument must be a string literal
error: format argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:108:13
   |
LL |     format!(invalid); //~ ERROR
   |
help: you might be missing a string literal to format with
   |
   |
LL |     format!("{}", invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:110:14
   |
   |
LL |     include!(invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:112:18
   |
   |
LL |     include_str!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   |
   = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)
error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:114:20
   |
   |
LL |     include_bytes!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)


error: trace_macros! accepts only `true` or `false`
   |
   |
LL |     trace_macros!(invalid); //~ ERROR


warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!(invalid); //~ ERROR
   |
   = note: `#[warn(deprecated)]` on by default

error: aborting due to 27 previous errors; 1 warning emitted
error: aborting due to 27 previous errors; 1 warning emitted


------------------------------------------


---- [ui] ui/out-of-stack.rs stdout ----
normalized stderr:
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL | pub fn black_box<T>(dummy: T) { unsafe { llvm_asm!("" : : "r"(&dummy)) } }
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/out-of-stack/out-of-stack.stderr
To only update this specific test, also pass `--test-args out-of-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/out-of-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/out-of-stack/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/out-of-stack/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
  --> /checkout/src/test/ui/out-of-stack.rs:23:42
   |
LL | pub fn black_box<T>(dummy: T) { unsafe { llvm_asm!("" : : "r"(&dummy)) } }
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---

---- [ui] ui/unsafe/inline_asm.rs#thir stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("nop");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
1 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
3    |


15    = note: inline assembly is entirely unchecked and can cause undefined behavior
16    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
19 
20 For more information about this error, try `rustc --explain E0133`.
20 For more information about this error, try `rustc --explain E0133`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/inline_asm.thir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |
   = note: `#[warn(deprecated)]` on by default


error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior

error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/unsafe/inline_asm.rs#mir stdout ----
diff of stderr:

+ warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
+    |
+    |
+ LL |     llvm_asm!("nop");
+    |
+    = note: `#[warn(deprecated)]` on by default
+ 
+ 
1 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
3    |


15    = note: inline assembly is entirely unchecked and can cause undefined behavior
16    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 2 previous errors
+ error: aborting due to 2 previous errors; 1 warning emitted
19 
20 For more information about this error, try `rustc --explain E0133`.
20 For more information about this error, try `rustc --explain E0133`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/inline_asm.mir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |
   = note: `#[warn(deprecated)]` on by default


error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior

error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0133`.

---
test result: FAILED. 11920 passed; 45 failed; 102 ignored; 0 measured; 0 filtered out; finished in 126.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:52
