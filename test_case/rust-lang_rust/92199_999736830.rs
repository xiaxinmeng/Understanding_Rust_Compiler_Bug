plain
........................................................................................i........... 5500/12514
.................................................................................................... 5600/12514
.................................................................................................... 5700/12514
.................................................................................................... 5800/12514
........................................................................F.F......................... 5900/12514
...................................i...........F.................................................... 6100/12514
.................................................................................................... 6200/12514
................i................................................................................... 6300/12514
.................................................................................i.................. 6400/12514
---

---- [ui] ui/async-await/issue-69446-fnmut-capture.rs stdout ----
diff of stderr:

1 error: captured variable cannot escape `FnMut` closure body
3    |
- LL |       let mut x = Foo;
-    |           ----- variable defined here
-    |           ----- variable defined here
6 LL |       bar(move || async {
7    |  _______________-_^

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
9    | |               inferred to be a `FnMut` closure
10 LL | |         x.foo();
-    | |         - variable captured here
+    | |         |
+    | |         variable defined here
+    | |         variable captured here
12 LL | |     });
12 LL | |     });
13    | |_____^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/issue-69446-fnmut-capture.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/issue-69446-fnmut-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-69446-fnmut-capture.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-69446-fnmut-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
   |
LL |       bar(move || async { //~ ERROR captured
   |  _______________-_^
   | |               |
   | |               inferred to be a `FnMut` closure
LL | |         x.foo();
   | |         |
   | |         variable defined here
   | |         variable captured here
LL | |     });
LL | |     });
   | |_____^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/borrowck/borrowck-describe-lvalue.rs stdout ----
diff of stderr:

21 error: captured variable cannot escape `FnMut` closure body
23    |
- LL |           let mut x = 0;
-    |               ----- variable defined here
26 LL |              || {
26 LL |              || {
27    |               - inferred to be a `FnMut` closure
28 LL | /                || {

29 LL | |                    let y = &mut x;
-    | |                                 - variable captured here
+    | |                                 |
+    | |                                 variable defined here
+    | |                                 variable captured here
31 LL | |                    &mut x;
31 LL | |                    &mut x;
32 LL | |                    *y = 1;
33 LL | |                    drop(y);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |             let y = &mut x;
   |                     ------ first mutable borrow occurs here
LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |             ^^^^^^ second mutable borrow occurs here
LL |             *y = 1;


error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |                    let y = &mut x;
   |                            ------ first mutable borrow occurs here
LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |                    ^^^^^^ second mutable borrow occurs here
LL |                    *y = 1;


error: captured variable cannot escape `FnMut` closure body
   |
LL |              || {
LL |              || {
   |               - inferred to be a `FnMut` closure
LL | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
LL | |                    let y = &mut x;
   | |                                 |
   | |                                 variable defined here
   | |                                 variable captured here
   | |                                 variable captured here
LL | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
LL | |                    *y = 1;
LL | |                    drop(y);
LL | |                 }
   | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape

error[E0503]: cannot use `f.x` because it was mutably borrowed
   |
   |
LL |         let x = f.x();
   |                 ----- borrow of `f` occurs here
LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
   |         ^^^ use of borrowed `f`
LL |         drop(x);


error[E0503]: cannot use `g.0` because it was mutably borrowed
   |
LL |         let x = g.x();
LL |         let x = g.x();
   |                 ----- borrow of `g` occurs here
LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
   |         ^^^ use of borrowed `g`
LL |         drop(x);


error[E0503]: cannot use `h.0` because it was mutably borrowed
   |
LL |         let x = &mut h.0;
LL |         let x = &mut h.0;
   |                 -------- borrow of `h.0` occurs here
LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
   |         ^^^ use of borrowed `h.0`
LL |         drop(x);


error[E0503]: cannot use `e.0` because it was mutably borrowed
   |
   |
LL |         let x = e.x();
   |                 ----- borrow of `e` occurs here
LL |         match e {
LL |             Baz::X(value) => value //~ ERROR cannot use `e.0` because it was mutably borrowed
   |                    ^^^^^ use of borrowed `e`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `u.a` because it was mutably borrowed
   |
LL |         let x = &mut u.a;
LL |         let x = &mut u.a;
   |                 -------- borrow of `u.a` occurs here
LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
   |         ^^^ use of borrowed `u.a`
LL |         drop(x);


error[E0503]: cannot use `f.x` because it was mutably borrowed
   |
   |
LL |         let x = f.x();
   |                 ----- borrow of `*f` occurs here
LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
   |         ^^^ use of borrowed `*f`
LL |         drop(x);


error[E0503]: cannot use `g.0` because it was mutably borrowed
   |
LL |         let x = g.x();
LL |         let x = g.x();
   |                 ----- borrow of `*g` occurs here
LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
   |         ^^^ use of borrowed `*g`
LL |         drop(x);


error[E0503]: cannot use `h.0` because it was mutably borrowed
   |
LL |         let x = &mut h.0;
LL |         let x = &mut h.0;
   |                 -------- borrow of `h.0` occurs here
LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
   |         ^^^ use of borrowed `h.0`
LL |         drop(x);


error[E0503]: cannot use `e.0` because it was mutably borrowed
   |
   |
LL |         let x = e.x();
   |                 ----- borrow of `*e` occurs here
LL |         match *e {
LL |             Baz::X(value) => value
   |                    ^^^^^ use of borrowed `*e`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `u.a` because it was mutably borrowed
   |
LL |         let x = &mut u.a;
LL |         let x = &mut u.a;
   |                 -------- borrow of `u.a` occurs here
LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
   |         ^^^ use of borrowed `u.a`
LL |         drop(x);


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         match v {
LL |             &[x, _, .., _, _] => println!("{}", x),
   |               ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x, .., _, _] => println!("{}", x),
   |                  ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, _, .., x, _] => println!("{}", x),
   |                         ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, _, .., _, x] => println!("{}", x),
   |                            ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         match v {
LL |             &[x @ ..] => println!("{:?}", x),
   |               ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x @ ..] => println!("{:?}", x),
   |                  ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[x @ .., _] => println!("{:?}", x),
   |               ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x @ .., _] => println!("{:?}", x),
   |                  ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `e` because it was mutably borrowed
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ borrow of `e` occurs here
LL |         match e {
   |               ^ use of borrowed `e`
LL |         drop(x);
   |              - borrow later used here


error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ mutable borrow occurs here
...
LL |             E::A(ref ax) =>
   |                  ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ mutable borrow occurs here
...
LL |             E::B { x: ref bx } =>
   |                       ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `s.y.0` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut s;
LL |         let x = &mut s;
   |                 ------ mutable borrow occurs here
LL |         match s {
LL |             S  { y: (ref y0, _), .. } =>
   |                      ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `s.x.y` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut s;
LL |         let x = &mut s;
   |                 ------ mutable borrow occurs here
...
LL |             S  { x: F { y: ref x0, .. }, .. } =>
   |                            ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0503]: cannot use `*v` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         v[0].y;
   |         ^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[_].y` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         v[0].y;
   |         ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0502]: cannot borrow `v[..].x` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ mutable borrow occurs here
LL |         match v {
LL |             &[_, F {x: ref xf, ..}] => println!("{}", xf),
   |                        ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
   |
LL |             let x = &mut block;
LL |             let x = &mut block;
   |                     ---------- mutable borrow occurs here
LL |             let p: &'a u8 = &*block.current;
   |                             ^^^^^^^^^^^^^^^ immutable borrow occurs here
LL |             drop(x);
   |                  - mutable borrow later used here


error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
   |
LL |             let x = &mut block;
LL |             let x = &mut block;
   |                     ---------- mutable borrow occurs here
LL |             let p : *const u8 = &*(*block).current;
   |                                 ^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
LL |             drop(x);
   |                  - mutable borrow later used here

error[E0382]: use of moved value: `x`
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:274:22
   |
LL |                 drop(x);
   |                      - value moved here
LL |                 drop(x); //~ ERROR use of moved value: `x`
   |                      ^ value used here after move
   |
   = note: move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
error: aborting due to 32 previous errors

Some errors have detailed explanations: E0382, E0499, E0502, E0503.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.

------------------------------------------


---- [ui] ui/issues/issue-40510-1.rs stdout ----
diff of stderr:

1 error: captured variable cannot escape `FnMut` closure body
3    |
3    |
- LL |     let mut x: Box<()> = Box::new(());
-    |         ----- variable defined here
- LL | 
7 LL |     || {
8    |      - inferred to be a `FnMut` closure

10    |         ^^^^^-
11    |         |    |
+    |         |    variable defined here
+    |         |    variable defined here
12    |         |    variable captured here
13    |         returns a reference to a captured variable which escapes the closure body


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/issue-40510-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/issue-40510-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-40510-1.rs`

---
test result: FAILED. 12388 passed; 7 failed; 119 ignored; 0 measured; 0 filtered out; finished in 157.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:22
