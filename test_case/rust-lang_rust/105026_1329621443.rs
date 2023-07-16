plain

running 13914 tests
........................................F.ii............................................ 88/13914
...............................................................................iiiiiiiii 176/13914
iii.iii......F.F...........i..................i......................................... 264/13914
........................................................................................ 440/13914
........................................................................................ 528/13914
........................................................................................ 616/13914
........................................................................................ 704/13914
---
........................................................................................ 11968/13914
........................................................................................ 12056/13914
........................................................................................ 12144/13914
........................................................................................ 12232/13914
...................i.......i........iF....i..F...FFF.F......F...i....................... 12320/13914
........................................................................................ 12496/13914
........................................................................................ 12584/13914
........................................................................................ 12672/13914
........................................................................................ 12760/13914
---

---- [ui] src/test/ui/abi/unsupported.rs#aarch64 stdout ----
diff of stderr:

+ '+v8a' is not a recognized feature for this target (ignoring feature)
+ '+v8a' is not a recognized feature for this target (ignoring feature)
1 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/unsupported.aarch64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `aarch64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "aarch64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/auxiliary" "--target=aarch64-unknown-linux-gnu" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0570]: `"wasm"` is not a supported ABI for the current target
   |
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
   |
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "x86-interrupt" fn x86() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" fn thiscall() {}

warning: use of calling convention not supported on this target
  --> /checkout/src/test/ui/abi/unsupported.rs:47:1
   |
   |
LL | extern "stdcall" fn stdcall() {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
   = note: `#[warn(unsupported_calling_conventions)]` on by default
---

---- [ui] src/test/ui/asm/bad-template.rs#aarch64_mirunsafeck stdout ----
diff of stderr:

+ '+v8a' is not a recognized feature for this target (ignoring feature)
+ '+v8a' is not a recognized feature for this target (ignoring feature)
1 error: invalid reference to argument at index 0
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/bad-template.aarch64_mirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-template.rs`

error in revision `aarch64_mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/auxiliary" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
error: invalid reference to argument at index 0
   |
LL |         asm!("{}");
   |               ^^ from here
   |
   |
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:36:16
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);


error: asm template modifier must be a single character
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
   |
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:59:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:61:14
  --> /checkout/src/test/ui/asm/bad-template.rs:61:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:61:20
  --> /checkout/src/test/ui/asm/bad-template.rs:61:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:64:15
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:66:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:66:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:66:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:69:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:69:20
  --> /checkout/src/test/ui/asm/bad-template.rs:69:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`

error: asm template modifier must be a single character
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
   |
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/bad-template.rs:50:15
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



---- [ui] src/test/ui/asm/bad-template.rs#aarch64_thirunsafeck stdout ----
diff of stderr:

+ '+v8a' is not a recognized feature for this target (ignoring feature)
+ '+v8a' is not a recognized feature for this target (ignoring feature)
1 error: invalid reference to argument at index 0
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/bad-template.aarch64_thirunsafeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-template.rs`

error in revision `aarch64_thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/auxiliary" "-Z" "thir-unsafeck" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
error: invalid reference to argument at index 0
   |
LL |         asm!("{}");
   |               ^^ from here
   |
   |
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:36:16
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);


error: asm template modifier must be a single character
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
   |
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:59:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:61:14
  --> /checkout/src/test/ui/asm/bad-template.rs:61:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:61:20
  --> /checkout/src/test/ui/asm/bad-template.rs:61:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:64:15
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:66:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:66:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:66:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:69:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:69:20
  --> /checkout/src/test/ui/asm/bad-template.rs:69:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`

error: asm template modifier must be a single character
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
   |
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/bad-template.rs:50:15
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



---- [ui] src/test/ui/target-feature/feature-hierarchy.rs#aarch64-neon stdout ----
normalized stderr:
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-neon/feature-hierarchy.aarch64-neon.stderr
To only update this specific test, also pass `--test-args target-feature/feature-hierarchy.rs`


error in revision `aarch64-neon`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/feature-hierarchy.rs" "-Zthreads=1" "--cfg" "aarch64_neon" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-neon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-neon/auxiliary" "-Ctarget-feature=+neon" "--target=aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)


---- [ui] src/test/ui/target-feature/feature-hierarchy.rs#aarch64-sve2 stdout ----
normalized stderr:
normalized stderr:
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-sve2/feature-hierarchy.aarch64-sve2.stderr
To only update this specific test, also pass `--test-args target-feature/feature-hierarchy.rs`


error in revision `aarch64-sve2`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/feature-hierarchy.rs" "-Zthreads=1" "--cfg" "aarch64_sve2" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-sve2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/feature-hierarchy.aarch64-sve2/auxiliary" "-Ctarget-feature=-neon,+sve2" "--target=aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)
'+v8a' is not a recognized feature for this target (ignoring feature)


---- [ui] src/test/ui/target-feature/no-llvm-leaks.rs#aarch64 stdout ----
normalized stderr:
