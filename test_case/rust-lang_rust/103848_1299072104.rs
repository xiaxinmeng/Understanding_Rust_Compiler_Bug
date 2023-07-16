plain
........................................................................................ 3960/13761
......i..........i..........i........................................................... 4048/13761
........................................................................................ 4136/13761
.iii.................................................................................... 4224/13761
...F.F................................................i................................. 4312/13761
........................................................................................ 4488/13761
........................................................................................ 4576/13761
........................................................................................ 4664/13761
................i....................................................................... 4752/13761
---
........................................................................................ 7392/13761
...........i............................................................................ 7480/13761
........................................................................................ 7568/13761
........................................................................................ 7656/13761
.........Fii.F.....................................ii........................F.......... 7744/13761
........................................................................................ 7920/13761
..........................................ii............................................ 8008/13761
........................................................................................ 8096/13761
........................................................................................ 8184/13761
---
---- [ui] src/test/ui/fmt/ifmt-bad-arg.rs stdout ----
diff of stderr:

170    |                  |
171    |                  help: format specifiers use curly braces: `{}`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
174 
175 error: invalid format string: expected `'}'`, found `'t'`
176   --> $DIR/ifmt-bad-arg.rs:75:1

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg/ifmt-bad-arg.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg/ifmt-bad-arg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/ifmt-bad-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-bad-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg/auxiliary"
stdout: none
--- stderr -------------------------------
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
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:16:19
   |
LL |     format!("{0} {1}", 1);
LL |     format!("{0} {1}", 1);
   |                   ^
   |
   = note: positional arguments are zero-based

error: invalid reference to positional argument 2 (there are 2 arguments)
   |
   |
LL |     format!("{0} {1} {2}", 1, 2);
   |
   = note: positional arguments are zero-based


error: 3 positional arguments in format string, but there are 2 arguments
   |
   |
LL |     format!("{} {value} {} {}", 1, value=2);
   |              ^^         ^^ ^^   -        -

error: 6 positional arguments in format string, but there are 3 arguments
   |
   |
LL |     format!("{name} {value} {} {} {} {} {} {}", 0, name=1, value=2);
   |                             ^^ ^^ ^^ ^^ ^^ ^^   -       -        -
error: multiple unused formatting arguments
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:32:17
   |
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
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:40:29
   |
LL |     format!("{foo}", foo=1, foo=2);  //~ ERROR: duplicate argument
   |                      ---    ^^^ duplicate argument
   |                      previously here

error: positional arguments cannot follow named arguments
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:41:35
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:41:35
   |
LL |     format!("{foo} {} {}", foo=1, 2);   //~ ERROR: positional arguments cannot follow
   |                            -----  ^ positional arguments must be before named arguments
   |                            named argument

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
   |
   = help: you might have meant to use the `print!` macro

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
   |                    |
   |                    this precision flag adds an extra required argument at position 1, which is why there are 4 arguments expected
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: invalid reference to positional arguments 3 and 7 (there are 3 arguments)
   |
   |
LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
   |                     ^^     ^
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: invalid reference to positional argument 7 (there are 3 arguments)
   |
   |
LL |     println!("{} {:07$} {}", 1, 3.2, 4);
   |
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
error: unknown format trait `foo`
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:86:17
   |
   |
LL |     println!("{:foo}", 1); //~ ERROR unknown format trait `foo`
   |
   = note: the only appropriate formatting traits are:
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
   |                ^    ^^   ^ ^^
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

error: invalid reference to positional argument 0 (no arguments were given)
   |
   |
LL |     println!("{foo:0$}");
   |
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

error: invalid reference to positional argument 0 (no arguments were given)
   |
   |
LL |     println!("{:.0$}");
   |
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:27:18
   |
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `bar` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:27:27
   |
   |
LL |     format!("{} {foo} {} {bar} {}", 1, 2, 3);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:31:15
   |
   |
LL |     format!("{foo}");                //~ ERROR: cannot find value `foo` in this scope


error[E0425]: cannot find value `valueb` in this scope
   |
   |
LL |     format!("{valuea} {valueb}", valuea=5, valuec=7);

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:60:10
   |
   |
LL |         {foo}
   |          ^^^ not found in this scope

error[E0308]: mismatched types
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:78:32
   |
LL |     println!("{} {:.*} {}", 1, 3.2, 4);
   |                                |
   |                                expected `usize`, found floating-point number
   |                                arguments to this function are incorrect
   |
   |
   = note: expected reference `&usize`
              found reference `&{float}`
note: associated function defined here
  --> /checkout/library/core/src/fmt/mod.rs:357:12
   |
LL |     pub fn from_usize(x: &usize) -> ArgumentV1<'_> {
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:81:35
  --> /checkout/src/test/ui/fmt/ifmt-bad-arg.rs:81:35
   |
LL |     println!("{} {:07$.*} {}", 1, 3.2, 4);
   |                                   |
   |                                   expected `usize`, found floating-point number
   |                                   arguments to this function are incorrect
   |
   |
   = note: expected reference `&usize`
              found reference `&{float}`
note: associated function defined here
  --> /checkout/library/core/src/fmt/mod.rs:357:12
   |
LL |     pub fn from_usize(x: &usize) -> ArgumentV1<'_> {
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 38 previous errors

---
---- [ui] src/test/ui/fmt/issue-89173.rs stdout ----
diff of stderr:

12    |
13 LL |     print!("%0*x", width, num);
-    = note: printf formatting not supported; see the documentation for `std::fmt`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-89173/issue-89173.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/issue-89173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/issue-89173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-89173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-89173/auxiliary"
stdout: none
--- stderr -------------------------------
error: multiple unused formatting arguments
   |
   |
LL |     print!("%0*x", width, num);
   |            ------  ^^^^^  ^^^ argument never used
   |            |       argument never used
   |            multiple missing formatting specifiers
   |
   |
note: format specifiers use curly braces, and you have to use a positional or named parameter for the width
   |
   |
LL |     print!("%0*x", width, num);
   |             ^^^^
   = help: you might have meant to use the `print!` macro
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/macros/format-foreign.rs stdout ----
diff of stderr:

8    |              |               argument never used
9    |              multiple missing formatting specifiers
10    |
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
12 help: format specifiers use curly braces
13    |
14 LL |     println!("{:.2$} {}!\n", "Hello,", "World", 4);
22    |               |
22    |               |
23    |               help: format specifiers use curly braces: `{0:1$.2$}`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
27 error: multiple unused formatting arguments
28   --> $DIR/format-foreign.rs:6:7


37    | |____|  argument never used
38    |      multiple missing formatting specifiers
-    = note: printf formatting not supported; see the documentation for `std::fmt`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
41 help: format specifiers use curly braces
42    |
43 LL ~     println!(r###"{:.2$}
60    |                         |
60    |                         |
61    |                         help: format specifiers use curly braces: `{NAME}`
-    = note: shell formatting not supported; see the documentation for `std::fmt`
-    = note: shell formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
65 error: multiple unused formatting arguments
66   --> $DIR/format-foreign.rs:15:32

72    |              |                 argument never used
72    |              |                 argument never used
73    |              multiple missing formatting specifiers
74    |
-    = note: shell formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
76 help: format specifiers use curly braces
77    |
78 LL |     println!("{1} {0} $$ {NAME}", 1, 2, NAME=3);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign/format-foreign.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/format-foreign.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/format-foreign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-foreign/auxiliary"
stdout: none
--- stderr -------------------------------
error: multiple unused formatting arguments
   |
   |
LL |     println!("%.*3$s %s!\n", "Hello,", "World", 4); //~ ERROR multiple unused formatting arguments
   |              --------------  ^^^^^^^^  ^^^^^^^  ^ argument never used
   |              |               |         argument never used
   |              |               argument never used
   |              multiple missing formatting specifiers
   |
   |
   = help: you might have meant to use the `print!` macro
help: format specifiers use curly braces
   |
LL |     println!("{:.2$} {}!\n", "Hello,", "World", 4); //~ ERROR multiple unused formatting arguments

error: argument never used
  --> /checkout/src/test/ui/macros/format-foreign.rs:3:29
   |
   |
LL |     println!("%1$*2$.*3$f", 123.456); //~ ERROR never used
   |               -----------   ^^^^^^^ argument never used
   |               |
   |               help: format specifiers use curly braces: `{0:1$.2$}`
   |
   = help: you might have meant to use the `print!` macro
error: multiple unused formatting arguments
  --> /checkout/src/test/ui/macros/format-foreign.rs:6:7
   |
   |
LL |       println!(r###"%.*3$s
   |  ______________-
LL | |         %s!\n
LL | | "###, "Hello,", "World", 4);
   | |    -  ^^^^^^^^  ^^^^^^^  ^ argument never used
   | |    |  |         argument never used
   | |    |  |         argument never used
   | |____|  argument never used
   |      multiple missing formatting specifiers
   |
   = help: you might have meant to use the `print!` macro
help: format specifiers use curly braces
   |
LL ~     println!(r###"{:.2$}
LL ~         {}!\n

error: argument never used
  --> /checkout/src/test/ui/macros/format-foreign.rs:12:30
   |
   |
LL |     println!("{} %f", "one", 2.0); //~ ERROR never used
   |              -------         ^^^ argument never used
   |              formatting specifier missing

error: named argument never used
  --> /checkout/src/test/ui/macros/format-foreign.rs:14:39
  --> /checkout/src/test/ui/macros/format-foreign.rs:14:39
   |
LL |     println!("Hi there, $NAME.", NAME="Tim"); //~ ERROR never used
   |                         -----         ^^^^^ named argument never used
   |                         |
   |                         help: format specifiers use curly braces: `{NAME}`
   |
   = help: you might have meant to use the `print!` macro
error: multiple unused formatting arguments
  --> /checkout/src/test/ui/macros/format-foreign.rs:15:32
   |
   |
LL |     println!("$1 $0 $$ $NAME", 1, 2, NAME=3);
   |              ----------------  ^  ^       ^ named argument never used
   |              |                 |  argument never used
   |              |                 argument never used
   |              multiple missing formatting specifiers
   |
   |
   = help: you might have meant to use the `print!` macro
help: format specifiers use curly braces
   |
LL |     println!("{1} {0} $$ {NAME}", 1, 2, NAME=3);

error: aborting due to 6 previous errors
------------------------------------------



---- [ui] src/test/ui/macros/format-unused-lables.rs stdout ----
diff of stderr:

44 LL |              , UNUSED="args");
45    |                       ^^^^^^ named argument never used
-    = note: shell formatting not supported; see the documentation for `std::fmt`
-    = note: shell formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
49 error: aborting due to 4 previous errors
50 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-unused-lables/format-unused-lables.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/format-unused-lables.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/format-unused-lables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-unused-lables" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-unused-lables/auxiliary"
stdout: none
--- stderr -------------------------------
error: multiple unused formatting arguments
   |
LL |     println!("Test", 123, 456, 789);
LL |     println!("Test", 123, 456, 789);
   |              ------  ^^^  ^^^  ^^^ argument never used
   |              |       |    argument never used
   |              |       argument never used
   |              multiple missing formatting specifiers


error: multiple unused formatting arguments
  --> /checkout/src/test/ui/macros/format-unused-lables.rs:6:9
   |
LL |     println!("Test2",
   |              ------- multiple missing formatting specifiers
LL |         123,  //~ ERROR multiple unused formatting arguments
   |         ^^^ argument never used
   |         ^^^ argument never used
LL |         789
   |         ^^^ argument never used


error: named argument never used
  --> /checkout/src/test/ui/macros/format-unused-lables.rs:11:35
   |
LL |     println!("Some stuff", UNUSED="args"); //~ ERROR named argument never used
   |              ------------         ^^^^^^ named argument never used
   |              formatting specifier missing

error: multiple unused formatting arguments
  --> /checkout/src/test/ui/macros/format-unused-lables.rs:14:9
  --> /checkout/src/test/ui/macros/format-unused-lables.rs:14:9
   |
LL |     println!("Some more $STUFF",
   |              |          |
   |              |          |
   |              |          help: format specifiers use curly braces: `{STUFF}`
   |              multiple missing formatting specifiers
LL |         "woo!",  //~ ERROR multiple unused formatting arguments
   |         ^^^^^^ argument never used
LL |             STUFF=
LL |        "things"
   |        ^^^^^^^^ named argument never used
LL |              , UNUSED="args");
   |                       ^^^^^^ named argument never used
   |
   = help: you might have meant to use the `print!` macro
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/macros/issue-92267.rs stdout ----
diff of stderr:

10    |
11 LL | pub fn main() { println!("🦀%%%", 0) }
-    = note: printf formatting not supported; see the documentation for `std::fmt`
-    = note: printf formatting not supported; see the documentation for `std::fmt`
+    = help: you might have meant to use the `print!` macro
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-92267/issue-92267.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/issue-92267.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-92267.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-92267" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-92267/auxiliary"
stdout: none
--- stderr -------------------------------
error: argument never used
   |
   |
LL | pub fn main() { println!("🦀%%%", 0) } //~ ERROR argument never used
   |                                   ^ argument never used
note: format specifiers use curly braces, and the conversion specifier `
note: format specifiers use curly braces, and the conversion specifier `
      ` is unknown or unsupported
   |
   |
LL | pub fn main() { println!("🦀%%%", 0) } //~ ERROR argument never used
   |                               ^^
   = help: you might have meant to use the `print!` macro
error: aborting due to previous error
------------------------------------------


