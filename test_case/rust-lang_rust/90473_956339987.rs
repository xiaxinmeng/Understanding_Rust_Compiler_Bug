plain
.................................................................................................... 3400/12369
.................................................................................................... 3500/12369
............................i........i.........i.................................................... 3600/12369
...................................................................................................i 3700/12369
i...................................................................FF..............F............... 3800/12369
.................................................................................................... 4000/12369
.................................................................................................... 4100/12369
.................................................................................................... 4200/12369
.................................................................................................... 4300/12369
---
1 error: there is no argument named `foo`
-   --> $DIR/format-args-capture-macro-hygiene.rs:4:13
+   --> $DIR/format-args-capture-macro-hygiene.rs:2:13
3    |
4 LL |     format!(concat!("{foo}"));

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
9    = note: this error originates in the macro `concat` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
10 
11 error: there is no argument named `bar`
-   --> $DIR/format-args-capture-macro-hygiene.rs:5:13
13    |
13    |
14 LL |     format!(concat!("{ba", "r} {}"), 1);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene/format-args-capture-macro-hygiene.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene/format-args-capture-macro-hygiene.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/format-args-capture-macro-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-args-capture-macro-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-macro-hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: there is no argument named `foo`
  --> /checkout/src/test/ui/fmt/format-args-capture-macro-hygiene.rs:2:13
   |
LL |     format!(concat!("{foo}"));         //~ ERROR: there is no argument named `foo`
   |
   |
   = note: did you intend to capture a variable `foo` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
   = note: this error originates in the macro `concat` (in Nightly builds, run with -Z macro-backtrace for more info)

error: there is no argument named `bar`
   |
   |
LL |     format!(concat!("{ba", "r} {}"), 1);     //~ ERROR: there is no argument named `bar`
   |
   |
   = note: did you intend to capture a variable `bar` from the surrounding scope?
   = note: to avoid ambiguity, `format_args!` cannot capture variables when the format string is expanded from a macro
   = note: this error originates in the macro `concat` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors


------------------------------------------
---
1 error: named argument never used
-   --> $DIR/format-args-capture-missing-variables.rs:10:51
+   --> $DIR/format-args-capture-missing-variables.rs:8:51
3    |
4 LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
5    |             -------------------                   ^ named argument never used
7    |             formatting specifier missing
8 
9 error[E0425]: cannot find value `foo` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:4:17
-   --> $DIR/format-args-capture-missing-variables.rs:4:17
+   --> $DIR/format-args-capture-missing-variables.rs:2:17
11    |
12 LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

14 
15 error[E0425]: cannot find value `bar` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:4:26
-   --> $DIR/format-args-capture-missing-variables.rs:4:26
+   --> $DIR/format-args-capture-missing-variables.rs:2:26
17    |
18 LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

20 
21 error[E0425]: cannot find value `foo` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:8:14
-   --> $DIR/format-args-capture-missing-variables.rs:8:14
+   --> $DIR/format-args-capture-missing-variables.rs:6:14
23    |
24 LL |     format!("{foo}");

26 
26 
27 error[E0425]: cannot find value `valueb` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:10:23
29    |
29    |
30 LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);

32 
33 error[E0425]: cannot find value `foo` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:16:9
---
39 error[E0425]: cannot find value `foo` in this scope
-   --> $DIR/format-args-capture-missing-variables.rs:21:13
+   --> $DIR/format-args-capture-missing-variables.rs:19:13
41    |
42 LL |     panic!("{foo} {bar}", bar=1);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-missing-variables/format-args-capture-missing-variables.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-missing-variables/format-args-capture-missing-variables.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/format-args-capture-missing-variables.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-missing-variables" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-missing-variables/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: named argument never used
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:8:51
   |
LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
   |             -------------------                   ^ named argument never used
   |             formatting specifier missing

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:2:17
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:2:17
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `bar` in this scope
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:2:26
   |
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:6:14
   |
   |
LL |     format!("{foo}"); //~ ERROR: cannot find value `foo` in this scope


error[E0425]: cannot find value `valueb` in this scope
   |
   |
LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:14:9
   |
   |
LL |         {foo}
   |         ^^^^^ not found in this scope

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/format-args-capture-missing-variables.rs:19:13
   |
LL |     panic!("{foo} {bar}", bar=1); //~ ERROR: cannot find value `foo` in this scope

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0425`.
---
60 
- error: there is no argument named `foo`
-   --> $DIR/ifmt-bad-arg.rs:27:17
-    |
- LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
-    |
- 
- 
- error: there is no argument named `bar`
-   --> $DIR/ifmt-bad-arg.rs:27:26
-    |
- LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
-    |
- 
- error: there is no argument named `foo`
-   --> $DIR/ifmt-bad-arg.rs:31:14
-   --> $DIR/ifmt-bad-arg.rs:31:14
-    |
- LL |     format!("{foo}");
-    |
- 
82 error: multiple unused formatting arguments
83   --> $DIR/ifmt-bad-arg.rs:32:17
83   --> $DIR/ifmt-bad-arg.rs:32:17
84    |

153    |                                |
154    |                                named argument
155 
- error: there is no argument named `valueb`
-   --> $DIR/ifmt-bad-arg.rs:45:23
-    |
- LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
-    |
- 
163 error: named argument never used
164   --> $DIR/ifmt-bad-arg.rs:45:51
---
206 
- error: there is no argument named `foo`
-   --> $DIR/ifmt-bad-arg.rs:60:9
-    |
- LL |         {foo}
-    |
- 
- 
214 error: invalid format string: expected `'}'`, found `'t'`
215   --> $DIR/ifmt-bad-arg.rs:75:1

297    = note: positional arguments are zero-based
297    = note: positional arguments are zero-based
298    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
+ error[E0425]: cannot find value `foo` in this scope
+   --> $DIR/ifmt-bad-arg.rs:27:17
+    |
+    |
+ LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
+ 
+ 
+ error[E0425]: cannot find value `bar` in this scope
+   --> $DIR/ifmt-bad-arg.rs:27:26
+    |
+ LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);
+ 
+ error[E0425]: cannot find value `foo` in this scope
+   --> $DIR/ifmt-bad-arg.rs:31:14
+    |
+    |
+ LL |     format!("{foo}");
+ 
+ 
+ error[E0425]: cannot find value `valueb` in this scope
+   --> $DIR/ifmt-bad-arg.rs:45:23
+    |
+ LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
+ 
+ error[E0425]: cannot find value `foo` in this scope
+   --> $DIR/ifmt-bad-arg.rs:60:9
+    |
+    |
+ LL |         {foo}
+ 
300 error[E0308]: mismatched types
301   --> $DIR/ifmt-bad-arg.rs:78:32
302    |
---
To only update this specific test, also pass `--test-args fmt/ifmt-bad-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-bad-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: 1 positional argument in format string, but no arguments were given
   |
LL |     format!("{}");
   |              ^^

---
error: argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:9:20
   |
LL |     format!("{1}", 1);
   |             -----  ^ argument never used
   |             formatting specifier missing


error: 2 positional arguments in format string, but no arguments were given
   |
   |
LL |     format!("{} {}");
   |              ^^ ^^
error: invalid reference to positional argument 1 (there is 1 argument)
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:16:18
   |
LL |     format!("{0} {1}", 1);
LL |     format!("{0} {1}", 1);
   |                  ^^^
   |
   = note: positional arguments are zero-based

error: invalid reference to positional argument 2 (there are 2 arguments)
   |
   |
LL |     format!("{0} {1} {2}", 1, 2);
   |
   = note: positional arguments are zero-based


error: invalid reference to positional argument 2 (there are 2 arguments)
   |
   |
LL |     format!("{} {value} {} {}", 1, value=2);
   |
   = note: positional arguments are zero-based


error: invalid reference to positional arguments 3, 4 and 5 (there are 3 arguments)
   |
   |
LL |     format!("{name} {value} {} {} {} {} {} {}", 0, name=1, value=2);
   |                                      ^^ ^^ ^^
   = note: positional arguments are zero-based

error: multiple unused formatting arguments
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:32:17
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:32:17
   |
LL |     format!("", 1, 2);               //~ ERROR: multiple unused formatting arguments
   |             --  ^  ^ argument never used
   |             |   |
   |             |   argument never used
   |             multiple missing formatting specifiers
error: argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:33:22
   |
   |
LL |     format!("{}", 1, 2);             //~ ERROR: argument never used
   |             ----     ^ argument never used
   |             formatting specifier missing

error: argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:34:20
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:34:20
   |
LL |     format!("{1}", 1, 2);            //~ ERROR: argument never used
   |             -----  ^ argument never used
   |             formatting specifier missing

error: named argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:35:26
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:35:26
   |
LL |     format!("{}", 1, foo=2);         //~ ERROR: named argument never used
   |             ----         ^ named argument never used
   |             formatting specifier missing

error: argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:36:22
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:36:22
   |
LL |     format!("{foo}", 1, foo=2);      //~ ERROR: argument never used
   |             -------  ^ argument never used
   |             formatting specifier missing

error: named argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:37:21
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:37:21
   |
LL |     format!("", foo=2);              //~ ERROR: named argument never used
   |             --      ^ named argument never used
   |             formatting specifier missing

error: multiple unused formatting arguments
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:38:32
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:38:32
   |
LL |     format!("{} {}", 1, 2, foo=1, bar=2);  //~ ERROR: multiple unused formatting arguments
   |             -------            ^      ^ named argument never used
   |             |                  named argument never used
   |             multiple missing formatting specifiers

error: duplicate argument named `foo`
error: duplicate argument named `foo`
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:40:33
   |
LL |     format!("{foo}", foo=1, foo=2);  //~ ERROR: duplicate argument
   |                          -      ^ duplicate argument
   |                          previously here

error: positional arguments cannot follow named arguments
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:41:35
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:41:35
   |
LL |     format!("{foo} {} {}", foo=1, 2);   //~ ERROR: positional arguments cannot follow
   |                                -  ^ positional arguments must be before named arguments
   |                                named argument

error: named argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:45:51
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:45:51
   |
LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);
   |             -------------------                   ^ named argument never used
   |             formatting specifier missing


error: invalid format string: expected `'}'` but string was terminated
   |
   |
LL |     format!("{"); //~ ERROR: expected `'}'` but string was terminated
   |              -^ expected `'}'` in format string
   |              because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: unmatched `}` found
   |
   |
LL |     format!("foo } bar"); //~ ERROR: unmatched `}` found
   |                  ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`

error: invalid format string: unmatched `}` found
   |
   |
LL |     format!("foo }"); //~ ERROR: unmatched `}` found
   |                  ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
error: argument never used
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:56:27
   |
   |
LL |     format!("foo %s baz", "bar"); //~ ERROR: argument never used
   |                  --       ^^^^^ argument never used
   |                  |
   |                  help: format specifiers use curly braces: `{}`
   = note: printf formatting not supported; see the documentation for `std::fmt`


error: invalid format string: expected `'}'`, found `'t'`
   |
   |
LL | ninth number: {
   |               - because of this opening brace
LL | tenth number: {}",
   | ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: 4 positional arguments in format string, but there are 3 arguments
   |
   |
LL |     println!("{} {:.*} {}", 1, 3.2, 4);
   |               ^^ ^^--^ ^^   -  ---  -
   |                    |           this parameter corresponds to the precision flag
   |                    |           this parameter corresponds to the precision flag
   |                    this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: 4 positional arguments in format string, but there are 3 arguments
   |
   |
LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
   |               ^^ ^^^----^ ^^   -  ---  -
   |                     | |           this parameter corresponds to the precision flag
   |                     | |           this parameter corresponds to the precision flag
   |                     | this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
   |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: invalid reference to positional argument 7 (there are 3 arguments)
   |
   |
LL |     println!("{} {:07$} {}", 1, 3.2, 4);
   |                  ^^^--^
   |                     |
   |                     this width flag expects an `usize` argument at position 7, but there are 3 arguments
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
error: unknown format trait `foo`
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:86:17
   |
   |
LL |     println!("{:foo}", 1); //~ ERROR unknown format trait `foo`
   |
   |
   = note: the only appropriate formatting traits are:
           - ``, which uses the `Display` trait
           - `?`, which uses the `Debug` trait
           - `e`, which uses the `LowerExp` trait
           - `E`, which uses the `UpperExp` trait
           - `o`, which uses the `Octal` trait
           - `p`, which uses the `Pointer` trait
           - `b`, which uses the `Binary` trait
           - `x`, which uses the `LowerHex` trait
           - `X`, which uses the `UpperHex` trait

error: invalid reference to positional arguments 4, 5, 6 and 7 (there is 1 argument)
   |
   |
LL |     println!("{5} {:4$} {6:7$}", 1);
   |               ^^^ ^^--^ ^^^--^
   |                     |      |
   |                     |      this width flag expects an `usize` argument at position 7, but there is 1 argument
   |                     this width flag expects an `usize` argument at position 4, but there is 1 argument
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: 2 positional arguments in format string, but no arguments were given
   |
   |
LL |     println!("{:.*}");
   |               ^^--^
   |                 |
   |                 this precision flag adds an extra required argument at position 0, which is why there are 2 arguments expected
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:27:17
   |
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `bar` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:27:26
   |
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:31:14
   |
   |
LL |     format!("{foo}");                //~ ERROR: no argument named `foo`


error[E0425]: cannot find value `valueb` in this scope
   |
   |
LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:60:9
   |
   |
LL |         {foo}
   |         ^^^^^ not found in this scope

error[E0308]: mismatched types
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:78:32
   |
LL |     println!("{} {:.*} {}", 1, 3.2, 4);
   |                                ^^^ expected `usize`, found floating-point number
   = note: expected reference `&usize`
   = note: expected reference `&usize`
              found reference `&{float}`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:81:35
   |
   |
LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
   |                                   ^^^ expected `usize`, found floating-point number
   = note: expected reference `&usize`
   = note: expected reference `&usize`
              found reference `&{float}`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 36 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
---
test result: FAILED. 12256 passed; 3 failed; 110 ignored; 0 measured; 0 filtered out; finished in 106.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:02
