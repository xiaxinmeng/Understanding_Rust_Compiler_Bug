plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 14831 tests
..........................................ii............................................    88/14831
...................................................................................iiiii   176/14831
iiiiiiiiii....F.....F.F.F......i..................i.....................................   264/14831
........................................................................................   440/14831
........................................................................................   528/14831
........................................................................................   616/14831
........................................................................................   704/14831
---
........................................................................................  2024/14831
................i.....................i...........ii....................................  2112/14831
........................................................................................  2200/14831
........................................................................................  2288/14831
......................F..FFi............................................................  2376/14831
........................................................................................  2552/14831
........................................................................................  2640/14831
........................................................................................  2728/14831
........................................................................................  2816/14831
---
........................................................................................ 11352/14831
........................................................................................ 11440/14831
........................................................................................ 11528/14831
........................................................................................ 11616/14831
.......................................................................iiiii...i...Fi.i. 11704/14831
......................................................................F................i 11880/14831
........................................................................................ 11968/14831
..........ii.ii.iiii.i..iiiiiiiiiii.i................................................... 12056/14831
........................................................................................ 12144/14831
---

---- [ui] tests/ui/asm/bad-template.rs#aarch64_mirunsafeck stdout ----
diff of stderr:

87 LL |         asm!("{}", in("x0") foo);
89 
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
92    |
92    |
93 LL |         asm!("{:foo}", in(reg) foo);
172    |
172    |
173    = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
177    |
177    |
178 LL | global_asm!("{:foo}", const FOO);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/bad-template.aarch64_mirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-template.rs`

error in revision `aarch64_mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/auxiliary" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:31:15
LL |         asm!("{}");
   |               ^^ from here
   |
   = note: no arguments were given
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> fake-test-src-base/asm/bad-template.rs:33:21
  --> fake-test-src-base/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> fake-test-src-base/asm/bad-template.rs:36:16
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> fake-test-src-base/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:41:21
  --> fake-test-src-base/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> fake-test-src-base/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);
help: use the register name directly in the assembly code
  --> fake-test-src-base/asm/bad-template.rs:48:20
   |
   |
LL |         asm!("{}", in("x0") foo);

error: asm template monitor must be at least a single character
  --> fake-test-src-base/asm/bad-template.rs:50:17
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
  --> fake-test-src-base/asm/bad-template.rs:53:18
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:59:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:61:14
  --> fake-test-src-base/asm/bad-template.rs:61:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> fake-test-src-base/asm/bad-template.rs:61:20
  --> fake-test-src-base/asm/bad-template.rs:61:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> fake-test-src-base/asm/bad-template.rs:64:15
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:66:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> fake-test-src-base/asm/bad-template.rs:66:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:66:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:69:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:69:20
  --> fake-test-src-base/asm/bad-template.rs:69:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: asm template monitor must be at least a single character
  --> fake-test-src-base/asm/bad-template.rs:72:16
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
  --> fake-test-src-base/asm/bad-template.rs:74:17
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
warning: formatting may not be suitable for sub-register argument
  --> fake-test-src-base/asm/bad-template.rs:50:15
   |
   |
LL |         asm!("{:foo}", in(reg) foo);
   |               ^^^^^^           --- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
   = note: `#[warn(asm_sub_register)]` on by default
error: aborting due to 21 previous errors; 1 warning emitted
------------------------------------------



---- [ui] tests/ui/asm/bad-template.rs#aarch64_thirunsafeck stdout ----
diff of stderr:

87 LL |         asm!("{}", in("x0") foo);
89 
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
92    |
92    |
93 LL |         asm!("{:foo}", in(reg) foo);
172    |
172    |
173    = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
177    |
177    |
178 LL | global_asm!("{:foo}", const FOO);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/bad-template.aarch64_thirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-template.rs`

error in revision `aarch64_thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/auxiliary" "-Z" "thir-unsafeck" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:31:15
LL |         asm!("{}");
   |               ^^ from here
   |
   = note: no arguments were given
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> fake-test-src-base/asm/bad-template.rs:33:21
  --> fake-test-src-base/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> fake-test-src-base/asm/bad-template.rs:36:16
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> fake-test-src-base/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:41:21
  --> fake-test-src-base/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> fake-test-src-base/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);
help: use the register name directly in the assembly code
  --> fake-test-src-base/asm/bad-template.rs:48:20
   |
   |
LL |         asm!("{}", in("x0") foo);

error: asm template monitor must be at least a single character
  --> fake-test-src-base/asm/bad-template.rs:50:17
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
  --> fake-test-src-base/asm/bad-template.rs:53:18
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:59:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:61:14
  --> fake-test-src-base/asm/bad-template.rs:61:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> fake-test-src-base/asm/bad-template.rs:61:20
  --> fake-test-src-base/asm/bad-template.rs:61:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> fake-test-src-base/asm/bad-template.rs:64:15
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> fake-test-src-base/asm/bad-template.rs:66:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> fake-test-src-base/asm/bad-template.rs:66:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:66:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> fake-test-src-base/asm/bad-template.rs:69:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> fake-test-src-base/asm/bad-template.rs:69:20
  --> fake-test-src-base/asm/bad-template.rs:69:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: asm template monitor must be at least a single character
  --> fake-test-src-base/asm/bad-template.rs:72:16
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
  --> fake-test-src-base/asm/bad-template.rs:74:17
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
warning: formatting may not be suitable for sub-register argument
  --> fake-test-src-base/asm/bad-template.rs:50:15
   |
   |
LL |         asm!("{:foo}", in(reg) foo);
   |               ^^^^^^           --- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
   = note: `#[warn(asm_sub_register)]` on by default
error: aborting due to 21 previous errors; 1 warning emitted
------------------------------------------



---- [ui] tests/ui/asm/bad-template.rs#x86_64_mirunsafeck stdout ----
diff of stderr:

87 LL |         asm!("{}", in("eax") foo);
89 
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
92    |
92    |
93 LL |         asm!("{:foo}", in(reg) foo);
172    |
172    |
173    = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
- error: asm template modifier must be a single character
- error: asm template modifier must be a single character
+ error: asm template monitor must be at least a single character
177    |
177    |
178 LL | global_asm!("{:foo}", const FOO);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_mirunsafeck/bad-template.x86_64_mirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


