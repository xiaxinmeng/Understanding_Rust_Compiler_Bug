plain

---- [ui] src/test/ui/fmt/ifmt-bad-arg.rs stdout ----
diff of stderr:

257    = note: positional arguments are zero-based
258    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
259 
- error: 1 positional argument in format string, but no arguments were given
-   --> $DIR/ifmt-bad-arg.rs:97:15
+ error: invalid reference to positional argument 0 (no arguments were given)
+   --> $DIR/ifmt-bad-arg.rs:97:16
262    |
263 LL |     println!("{:.0$}");
-    |               ^^---^
-    |                 |
-    |                 this precision flag expects an `usize` argument at position 0, but no arguments were given
267    |
268    = note: positional arguments are zero-based
268    = note: positional arguments are zero-based
269    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-bad-arg/ifmt-bad-arg.stderr
To update references, rerun the tests and pass the `--bless` flag
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

