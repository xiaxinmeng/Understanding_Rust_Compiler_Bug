plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12354 tests
.................................................................................................... 100/12354
........................................iiiiiiiiiii........FFF.i...F.......F.F....iF...FF...FF..F... 200/12354
...........................................................................F........................ 300/12354
.................................................................................................... 500/12354
.................................................................................................... 600/12354
...........................................................................i........................ 700/12354
...................................................ii............................................... 800/12354
---
---- [ui] ui/asm/bad-template.rs#aarch64_mirunsafeck stdout ----
diff of stderr:

183    |
184    = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
- error: aborting due to 21 previous errors
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:60:20
+    |
+    |
+ LL | global_asm!("{1}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
187 
187 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:65:19
+    |
+ LL | global_asm!("{}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:68:20
+    |
+ LL | global_asm!("{1}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:71:23
+    |
+ LL | global_asm!("{:foo}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:17
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:28
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error: aborting due to 27 previous errors
+ 
+ For more information about this error, try `rustc --explain E0658`.
188 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/bad-template.aarch64_mirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `aarch64_mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:31:15
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
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
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
   |
LL | global_asm!("{1}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:71:23
   |
LL | global_asm!("{:foo}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:17
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:28
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---
---- [ui] ui/asm/bad-template.rs#aarch64_thirunsafeck stdout ----
diff of stderr:

183    |
184    = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
- error: aborting due to 21 previous errors
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:60:20
+    |
+    |
+ LL | global_asm!("{1}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
187 
187 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:65:19
+    |
+ LL | global_asm!("{}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:68:20
+    |
+ LL | global_asm!("{1}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:71:23
+    |
+ LL | global_asm!("{:foo}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:17
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:28
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error: aborting due to 27 previous errors
+ 
+ For more information about this error, try `rustc --explain E0658`.
188 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/bad-template.aarch64_thirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `aarch64_thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:31:15
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
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
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
   |
LL | global_asm!("{1}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:71:23
   |
LL | global_asm!("{:foo}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:17
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:28
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---
---- [ui] ui/asm/bad-template.rs#x86_64_thirunsafeck stdout ----
diff of stderr:

183    |
184    = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
- error: aborting due to 21 previous errors
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:60:20
+    |
+    |
+ LL | global_asm!("{1}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
187 
187 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:65:19
+    |
+ LL | global_asm!("{}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:68:20
+    |
+ LL | global_asm!("{1}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:71:23
+    |
+ LL | global_asm!("{:foo}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:17
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:28
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error: aborting due to 27 previous errors
+ 
+ For more information about this error, try `rustc --explain E0658`.
188 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_thirunsafeck/bad-template.x86_64_thirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `x86_64_thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "x86_64_thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:31:15
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:45:15
   |
   |
LL |         asm!("{}", in("eax") foo);
   |               ^^   ------------- explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:45:20
   |
LL |         asm!("{}", in("eax") foo);


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
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
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
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
   |
LL | global_asm!("{1}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:71:23
   |
LL | global_asm!("{:foo}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:17
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:28
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---
---- [ui] ui/asm/bad-template.rs#x86_64_mirunsafeck stdout ----
diff of stderr:

183    |
184    = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
- error: aborting due to 21 previous errors
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:60:20
+    |
+    |
+ LL | global_asm!("{1}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
187 
187 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:65:19
+    |
+ LL | global_asm!("{}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:68:20
+    |
+ LL | global_asm!("{1}", a = const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:71:23
+    |
+ LL | global_asm!("{:foo}", const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-template.rs:73:17
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/bad-template.rs:73:28
+    |
+ LL | global_asm!("", const FOO, const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error: aborting due to 27 previous errors
+ 
+ For more information about this error, try `rustc --explain E0658`.
188 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_mirunsafeck/bad-template.x86_64_mirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `x86_64_mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "x86_64_mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.x86_64_mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:31:15
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
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
  --> /checkout/src/test/ui/asm/bad-template.rs:45:15
   |
   |
LL |         asm!("{}", in("eax") foo);
   |               ^^   ------------- explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:45:20
   |
LL |         asm!("{}", in("eax") foo);


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
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
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
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
   |
LL | global_asm!("{1}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:71:23
   |
LL | global_asm!("{:foo}", const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:17
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/bad-template.rs:73:28
   |
LL | global_asm!("", const FOO, const FOO);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---

---- [ui] ui/asm/named-asm-labels.rs stdout ----
diff of stderr:

- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:22:15
+ error[E0658]: const operands for inline assembly are unstable
3    |
3    |
- LL |         asm!("bar: nop");
-    |
-    |
-    = note: `#[deny(named_asm_labels)]` on by default
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:25:15
-    |
- LL |         asm!("abcd:");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:28:15
-    |
- LL |         asm!("foo: bar1: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:32:15
-    |
- LL |         asm!("foo1: nop", "nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:33:15
-    |
- LL |         asm!("foo2: foo3: nop", "nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:35:22
-    |
- LL |         asm!("nop", "foo4: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:36:15
-    |
- LL |         asm!("foo5: nop", "foo6: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:36:28
-    |
- LL |         asm!("foo5: nop", "foo6: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:41:15
-    |
- LL |         asm!("foo7: nop; foo8: nop");
-    |               ^^^^       ^^^^
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:43:15
-    |
- LL |         asm!("foo9: nop; nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:44:20
-    |
- LL |         asm!("nop; foo10: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:47:15
-    |
- LL |         asm!("bar2: nop\n bar3: nop");
-    |               ^^^^        ^^^^
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:49:15
-    |
- LL |         asm!("bar4: nop\n nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:50:21
-    |
- LL |         asm!("nop\n bar5: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:51:21
-    |
- LL |         asm!("nop\n bar6: bar7: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:57:13
-    |
- LL |             blah2: nop
-    |             ^^^^^
- LL |             blah3: nop
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:66:19
-    |
- LL |             nop ; blah4: nop
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:80:15
-    |
- LL |         asm!("blah1: 2bar: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:83:15
-    |
- LL |         asm!("def: def: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:84:15
-    |
- LL |         asm!("def: nop\ndef: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:85:15
-    |
- LL |         asm!("def: nop; def: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:93:15
-    |
- LL |         asm!("fooo\u{003A} nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:94:15
-    |
- LL |         asm!("foooo\x3A nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:97:15
-    |
- LL |         asm!("fooooo:\u{000A} nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:98:15
-    |
- LL |         asm!("foooooo:\x0A nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:102:14
-    |
- LL |         asm!("\x41\x42\x43\x3A\x20\x6E\x6F\x70");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:110:13
-    |
- LL |             ab: nop // ab: does foo
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:122:14
-    |
- LL |         asm!(include_str!("named-asm-labels.s"));
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- warning: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:132:19
-    |
- LL |             asm!("warned: nop");
-    |
- note: the lint level is defined here
-   --> $DIR/named-asm-labels.rs:130:16
-    |
-    |
- LL |         #[warn(named_asm_labels)]
-    |                ^^^^^^^^^^^^^^^^
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:141:20
-    |
273 LL |     unsafe { asm!(".Lfoo: mov rax, {}; ret;", "nop", const 1, options(noreturn)) }
+    |                                                      ^^^^^^^
275    |
275    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
278 
278 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:147:20
+ error[E0658]: const operands for inline assembly are unstable
281    |
281    |
282 LL |     unsafe { asm!(".Lbar: mov rax, {}; ret;", "nop", const 1, options(noreturn)) }
+    |                                                      ^^^^^^^
284    |
284    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
287 
287 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:155:20
-    |
- LL |     unsafe { asm!(".Laaa: nop; ret;", options(noreturn)) }
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
+ error: aborting due to 2 previous errors
296 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:165:24
-    |
- LL |         unsafe { asm!(".Lbbb: nop; ret;", options(noreturn)) }
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:174:15
-    |
- LL |         asm!("closure1: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:178:15
-    |
- LL |         asm!("closure2: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- 
- error: avoid using named labels in inline assembly
-   --> $DIR/named-asm-labels.rs:188:19
-    |
- LL |             asm!("closure3: nop");
-    |
-    |
-    = help: only local labels of the form `<number>:` should be used in inline asm
-    = note: see the asm section of the unstable book <https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html#labels> for more information
- error: aborting due to 35 previous errors; 1 warning emitted
- 
+ For more information about this error, try `rustc --explain E0658`.
335 
---
To only update this specific test, also pass `--test-args asm/named-asm-labels.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/named-asm-labels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/named-asm-labels" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/named-asm-labels/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/named-asm-labels.rs:141:54
   |
LL |     unsafe { asm!(".Lfoo: mov rax, {}; ret;", "nop", const 1, options(noreturn)) } //~ ERROR avoid using named labels
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/named-asm-labels.rs:147:54
   |
LL |     unsafe { asm!(".Lbar: mov rax, {}; ret;", "nop", const 1, options(noreturn)) }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---

---- [ui] ui/asm/naked-functions.rs stdout ----
diff of stderr:

4 LL |     asm!("", options(readonly, nostack), options(pure));
6 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/naked-functions.rs:74:10
+    |
+    |
+ LL |          const F,
+    |          ^^^^^^^
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ error[E0658]: sym operands for inline assembly are unstable
+   --> $DIR/naked-functions.rs:75:10
+ LL |          sym G,
+    |          ^^^^^
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
7 error: patterns not allowed in naked function parameters
8   --> $DIR/naked-functions.rs:18:5
9    |

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
error: asm with the `pure` option must have at least one output
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));

error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/naked-functions.rs:74:10
   |
   |
LL |          const F,
   |          ^^^^^^^
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
   |
LL |          sym G,
   |          ^^^^^
   |
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:18:5
   |
LL |     mut a: u32,
LL |     mut a: u32,
   |     ^^^^^

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:20:5
   |
LL |     &b: &i32,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:22:6
   |
   |
LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:24:5
   |
   |
LL |     P { x, y }: P,

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:34:5
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
  --> /checkout/src/test/ui/asm/naked-functions.rs:40:31
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
   | |_____^
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
   | |     -------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("");
   | |     -------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("", options(noreturn));
   | |     --------------------------- multiple asm blocks are unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:106:11
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
  --> /checkout/src/test/ui/asm/naked-functions.rs:140:15
   |
LL | pub unsafe fn default_abi() {
   |
   |
   = note: `#[warn(undefined_naked_function_abi)]` on by default
warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:146:29
   |
   |
LL | pub unsafe extern "Rust" fn rust_abi() {

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:180:1
   |
---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:188:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:196:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:207:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:210:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---
---- [ui] ui/asm/x86_64/bad-reg.rs stdout ----
diff of stderr:

30    |
31    = note: the `xmm_reg` register class supports the following template modifiers: `x`, `y`, `z`
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/bad-reg.rs:20:22
+    |
+    |
+ LL |         asm!("{:a}", const 0);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
33 error: asm template modifiers are not allowed for `const` arguments
35    |

38    |               |
39    |               template modifier
39    |               template modifier
40 
+ error[E0658]: sym operands for inline assembly are unstable
+    |
+    |
+ LL |         asm!("{:a}", sym main);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
+ 
41 error: asm template modifiers are not allowed for `sym` arguments
43    |


156 LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
158 
- error: aborting due to 21 previous errors
+ error: aborting due to 23 previous errors
160 
160 
+ For more information about this error, try `rustc --explain E0658`.
161 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg/bad-reg.stderr
To only update this specific test, also pass `--test-args asm/x86_64/bad-reg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/bad-reg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid register class `foo`: unknown register class
   |
   |
LL |         asm!("{}", in(foo) foo);


error: invalid register `foo`: unknown register
   |
   |
LL |         asm!("", in("foo") foo);

error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/x86_64/bad-reg.rs:16:15
   |
   |
LL |         asm!("{:z}", in(reg) foo);
   |               ^^^^   ----------- argument
   |               template modifier
   |
   |
   = note: the `reg` register class supports the following template modifiers: `l`, `x`, `e`, `r`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/x86_64/bad-reg.rs:18:15
   |
   |
LL |         asm!("{:r}", in(xmm_reg) foo);
   |               ^^^^   --------------- argument
   |               template modifier
   |
   |
   = note: the `xmm_reg` register class supports the following template modifiers: `x`, `y`, `z`
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/bad-reg.rs:20:22
   |
   |
LL |         asm!("{:a}", const 0);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error: asm template modifiers are not allowed for `const` arguments
   |
   |
LL |         asm!("{:a}", const 0);
   |               ^^^^   ------- argument
   |               template modifier


error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |         asm!("{:a}", sym main);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable

error: asm template modifiers are not allowed for `sym` arguments
   |
   |
LL |         asm!("{:a}", sym main);
   |               ^^^^   -------- argument
   |               template modifier


error: invalid register `ebp`: the frame pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("ebp") foo);


error: invalid register `rsp`: the stack pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("rsp") foo);


error: invalid register `ip`: the instruction pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("ip") foo);


error: invalid register `k0`: the k0 AVX mask register cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("k0") foo);


error: invalid register `ah`: high byte registers cannot be used as an operand on x86_64
   |
   |
LL |         asm!("", in("ah") foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("st(2)") foo);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("mm0") foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(x87_reg) foo);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(mmx_reg) foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(x87_reg) _);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(mmx_reg) _);


error: register `al` conflicts with register `ax`
   |
   |
LL |         asm!("", in("eax") foo, in("al") bar);
   |                  -------------  ^^^^^^^^^^^^ register `al`
   |                  register `ax`


error: register `ax` conflicts with register `ax`
   |
   |
LL |         asm!("", in("rax") foo, out("rax") bar);
   |                  -------------  ^^^^^^^^^^^^^^ register `ax`
   |                  register `ax`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("rax") foo, out("rax") bar);


error: register `ymm0` conflicts with register `xmm0`
   |
   |
LL |         asm!("", in("xmm0") foo, in("ymm0") bar);
   |                  --------------  ^^^^^^^^^^^^^^ register `ymm0`
   |                  register `xmm0`


error: register `ymm0` conflicts with register `xmm0`
   |
   |
LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
   |                  --------------  ^^^^^^^^^^^^^^^ register `ymm0`
   |                  register `xmm0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("xmm0") foo, out("ymm0") bar);

error: aborting due to 23 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui/asm/x86_64/const.rs#mirunsafeck stdout ----

error in revision `mirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/const.mirunsafeck/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/const.mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:11:40
   |
LL |         asm!("mov {}, {}", out(reg) a, const X);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:23:40
   |
LL |         asm!("mov {}, {}", out(reg) a, const 5);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:27:40
   |
LL |         asm!("mov {}, {}", out(reg) b, const constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:31:40
   |
LL |         asm!("mov {}, {}", out(reg) c, const constfn(5) + constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:39:28
   |
LL | global_asm!("mov eax, {}", const 5);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:40:28
   |
LL | global_asm!("mov eax, {}", const constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:41:28
   |
LL | global_asm!("mov eax, {}", const constfn(5) + constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---


---- [ui] ui/asm/x86_64/const.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/const.thirunsafeck/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/const.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:11:40
   |
LL |         asm!("mov {}, {}", out(reg) a, const X);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:23:40
   |
LL |         asm!("mov {}, {}", out(reg) a, const 5);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:27:40
   |
LL |         asm!("mov {}, {}", out(reg) b, const constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:31:40
   |
LL |         asm!("mov {}, {}", out(reg) c, const constfn(5) + constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:39:28
   |
LL | global_asm!("mov eax, {}", const 5);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:40:28
   |
LL | global_asm!("mov eax, {}", const constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/const.rs:41:28
   |
LL | global_asm!("mov eax, {}", const constfn(5) + constfn(5));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---

---- [ui] ui/asm/x86_64/parse-error.rs stdout ----
diff of stderr:

457 LL |         asm!("{1}", in("eax") foo, const bar);
458    |                                          ^^^ non-constant value
- error: aborting due to 66 previous errors
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:37:31
+    |
+    |
+ LL |         asm!("{}", options(), const foo);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
461 
461 
- For more information about this error, try `rustc --explain E0435`.
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:46:38
+    |
+ LL |         asm!("{}", clobber_abi("C"), const foo);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:55:21
+    |
+ LL |         asm!("{a}", a = const foo, a = const bar);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:55:36
+    |
+ LL |         asm!("{a}", a = const foo, a = const bar);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:62:36
+    |
+ LL |         asm!("{a}", in("eax") foo, a = const bar);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:65:36
+    |
+ LL |         asm!("{a}", in("eax") foo, a = const bar);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:68:36
+    |
+ LL |         asm!("{1}", in("eax") foo, const bar);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:108:30
+    |
+ LL | global_asm!("{}", options(), const FOO);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:125:20
+    |
+ LL | global_asm!("{a}", a = const FOO, a = const BAR);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/parse-error.rs:125:35
+    |
+ LL | global_asm!("{a}", a = const FOO, a = const BAR);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
---
To only update this specific test, also pass `--test-args asm/x86_64/parse-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:11:14
   |
LL |         asm!(foo);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:13:19
   |
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`

error: expected operand, clobber_abi, options, or additional template string
   |
   |
LL |         asm!("{}", foo);
   |                    ^^^ expected operand, clobber_abi, options, or additional template string

error: expected `(`, found `foo`
   |
   |
LL |         asm!("{}", in foo);
   |                       ^^^ expected `(`

error: expected `)`, found `foo`
   |
   |
LL |         asm!("{}", in(reg foo));
   |                           ^^^ expected `)`
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:21:27
   |
   |
LL |         asm!("{}", in(reg));
   |                           ^ expected expression
error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:23:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:25:37
   |
   |
LL |         asm!("{}", inout(reg) foo =>);
   |                                     ^ expected expression

error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
   |
   |
LL |         asm!("{}", in(reg) foo => bar);
   |                                ^^ expected one of 7 possible tokens

error: argument to `sym` must be a path expression
   |
   |
LL |         asm!("{}", sym foo + bar);


error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(foo));
   |                          ^^^ expected one of 9 possible tokens

error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", options(nomem foo));
   |                                ^^^ expected one of `)` or `,`

error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(nomem, foo));
   |                                 ^^^ expected one of 9 possible tokens
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:37:31
   |
   |
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:40:30
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:40:30
   |
LL |         asm!("", clobber_abi(foo));


error: expected `)`, found `foo`
   |
   |
LL |         asm!("", clobber_abi("C" foo));
   |                                  ^^^ expected `)`

error: expected `)`, found `,`
   |
   |
LL |         asm!("", clobber_abi("C", foo));
   |                                 ^ expected `)`
error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:46:38
   |
   |
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                    ----------------  ^^^^^^^^^ argument
   |                    clobber_abi

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:49:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:49:29
   |
LL |         asm!("", options(), clobber_abi("C"));
   |                  ---------  ^^^^^^^^^^^^^^^^
   |                  options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:51:31
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:51:31
   |
LL |         asm!("{}", options(), clobber_abi("C"), const foo);
   |                    ---------  ^^^^^^^^^^^^^^^^
   |                    options

error: clobber_abi specified multiple times
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:36
   |
LL |         asm!("", clobber_abi("C"), clobber_abi("C"));
   |                  ----------------  ^^^^^^^^^^^^^^^^
   |                  |
   |                  clobber_abi previously specified here
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:36
   |
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:60:18
   |
   |
LL |         asm!("", a = in("eax") foo);


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:68:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:68:36
   |
LL |         asm!("{1}", in("eax") foo, const bar);
   |                     -------------  ^^^^^^^^^ positional argument
   |                     explicit register argument


error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `""`
   |
   |
LL |         asm!("", options(), "");
   |                             ^^ expected one of 9 possible tokens

error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `"{}"`
   |
   |
LL |         asm!("{}", in(reg) foo, "{}", out(reg) foo);
   |                                 ^^^^ expected one of 9 possible tokens
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:75:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:77:21
   |
LL |         asm!("{1}", format!("{{{}}}", 0), in(reg) foo, out(reg) bar);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", in(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inout(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inlateout(reg) _);

error: requires at least a template string argument
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:90:1
   |
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:92:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:94:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:96:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:98:24
   |
   |
LL | global_asm!("{}", const);
   |                        ^ expected expression

error: expected one of `,`, `.`, `?`, or an operator, found `FOO`
   |
   |
LL | global_asm!("{}", const(reg) FOO);
   |                              ^^^ expected one of `,`, `.`, `?`, or an operator

error: expected one of `)`, `att_syntax`, or `raw`, found `FOO`
   |
   |
LL | global_asm!("", options(FOO));
   |                         ^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem, FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:108:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:110:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:110:29
   |
LL | global_asm!("", clobber_abi(FOO));


error: expected `)`, found `FOO`
   |
   |
LL | global_asm!("", clobber_abi("C" FOO));
   |                                 ^^^ expected `)`

error: expected `)`, found `,`
   |
   |
LL | global_asm!("", clobber_abi("C", FOO));
   |                                ^ expected `)`
error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:116:37
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);
   |                   ----------------  ^^^^^^^^^ argument
   |                   clobber_abi


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:119:28
   |
   |
LL | global_asm!("", options(), clobber_abi("C"));
   |                 ---------  ^^^^^^^^^^^^^^^^
   |                 options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:121:30
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:121:30
   |
LL | global_asm!("{}", options(), clobber_abi("C"), const FOO);
   |                   ---------  ^^^^^^^^^^^^^^^^
   |                   options

error: clobber_abi specified multiple times
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:35
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:35
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));
   |                 ----------------  ^^^^^^^^^^^^^^^^
   |                 |
   |                 clobber_abi previously specified here
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:125:35
   |
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:125:35
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:125:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                                   ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`

error: expected one of `clobber_abi`, `const`, or `options`, found `""`
   |
   |
LL | global_asm!("", options(), "");
   |                            ^^ expected one of `clobber_abi`, `const`, or `options`

error: expected one of `clobber_abi`, `const`, or `options`, found `"{}"`
   |
   |
LL | global_asm!("{}", const FOO, "{}", const FOO);
   |                              ^^^^ expected one of `clobber_abi`, `const`, or `options`
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:132:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:134:20
   |
LL | global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0435]: attempt to use a non-constant value in a constant
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:37:37
   |
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", options(), const foo);
   |                                     ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:46:44
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                                            ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:31
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                               ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:62:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:65:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:68:42
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{1}", in("eax") foo, const bar);
   |                                          ^^^ non-constant value
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:37:31
   |
   |
LL |         asm!("{}", options(), const foo);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:46:38
   |
LL |         asm!("{}", clobber_abi("C"), const foo);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:21
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:62:36
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
---

---- [ui] ui/asm/x86_64/type-check-2.rs stdout ----
diff of stderr:

- error: arguments for inline assembly must be copyable
-   --> $DIR/type-check-2.rs:42:32
+ error[E0658]: sym operands for inline assembly are unstable
3    |
3    |
- LL |         asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
-    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |         asm!("{}", sym S);
6    |
6    |
-    = note: `SimdNonCopy` does not implement the Copy trait
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
8 
- error: cannot use value of type `[closure@$DIR/type-check-2.rs:54:28: 54:38]` for inline assembly
-   --> $DIR/type-check-2.rs:54:28
+ error[E0658]: sym operands for inline assembly are unstable
11    |
11    |
- LL |         asm!("{}", in(reg) |x: i32| x);
-    |                            ^^^^^^^^^^
+ LL |         asm!("{}", sym main);
14    |
14    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
16 
- error: cannot use value of type `Vec<i32>` for inline assembly
-   --> $DIR/type-check-2.rs:56:28
+ error[E0658]: sym operands for inline assembly are unstable
19    |
19    |
- LL |         asm!("{}", in(reg) vec![0]);
-    |                            ^^^^^^^
+ LL |         asm!("{}", sym C);
22    |
22    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
-    = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
25 
- error: cannot use value of type `(i32, i32, i32)` for inline assembly
-   --> $DIR/type-check-2.rs:58:28
+ error[E0658]: sym operands for inline assembly are unstable
28    |
28    |
- LL |         asm!("{}", in(reg) (1, 2, 3));
-    |
-    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
- 
- error: cannot use value of type `[i32; 3]` for inline assembly
-   --> $DIR/type-check-2.rs:60:28
-    |
- LL |         asm!("{}", in(reg) [1, 2, 3]);
-    |
-    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
- 
- error: cannot use value of type `fn() {main}` for inline assembly
-   --> $DIR/type-check-2.rs:68:31
-    |
- LL |         asm!("{}", inout(reg) f);
-    |
-    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
- error: cannot use value of type `&mut i32` for inline assembly
-   --> $DIR/type-check-2.rs:71:31
-    |
-    |
- LL |         asm!("{}", inout(reg) r);
-    |
-    |
-    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
- 
- error: asm `sym` operand must point to a fn or static
-   --> $DIR/type-check-2.rs:35:24
-    |
- LL |         asm!("{}", sym C);
- 
- 
- error: asm `sym` operand must point to a fn or static
-   --> $DIR/type-check-2.rs:37:24
-    |
67 LL |         asm!("{}", sym x);
- 
- 
- error[E0381]: use of possibly-uninitialized variable: `x`
-   --> $DIR/type-check-2.rs:13:28
72    |
72    |
- LL |         asm!("{}", in(reg) x);
-    |                            ^ use of possibly-uninitialized `x`
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_sym)]` to the crate attributes to enable
75 
- error[E0381]: use of possibly-uninitialized variable: `y`
-   --> $DIR/type-check-2.rs:16:9
-    |
- LL |         asm!("{}", inout(reg) y);
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
+ error: aborting due to 4 previous errors
81 
- error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:24:29
-    |
- LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
- LL |         asm!("{}", in(reg) v[0]);
- LL |         asm!("{}", out(reg) v[0]);
-    |                             ^ cannot borrow as mutable
- 
- error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:26:31
-    |
- LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
- ...
- LL |         asm!("{}", inout(reg) v[0]);
-    |                               ^ cannot borrow as mutable
- error: aborting due to 13 previous errors
- 
- Some errors have detailed explanations: E0381, E0596.
- For more information about an error, try `rustc --explain E0381`.
- For more information about an error, try `rustc --explain E0381`.
+ For more information about this error, try `rustc --explain E0658`.
104 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/x86_64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |         asm!("{}", sym S);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |         asm!("{}", sym main);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |         asm!("{}", sym C);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |         asm!("{}", sym x);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.

---
-   --> $DIR/type-check-3.rs:12:28
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-3.rs:82:19
3    |
- LL |         asm!("{}", in(reg) 0i128);
-    |                            ^^^^^
+ LL | global_asm!("{}", const S);
6    |
6    |
-    = note: register class `reg` supports these types: i16, i32, i64, f32, f64
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
8 
- error: type `__m128` cannot be used with this register class
-   --> $DIR/type-check-3.rs:14:28
-   --> $DIR/type-check-3.rs:14:28
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-3.rs:84:19
11    |
- LL |         asm!("{}", in(reg) _mm_setzero_ps());
-    |                            ^^^^^^^^^^^^^^^^
+ LL | global_asm!("{}", const const_foo(0));
14    |
14    |
-    = note: register class `reg` supports these types: i16, i32, i64, f32, f64
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
16 
- error: type `__m256` cannot be used with this register class
-   --> $DIR/type-check-3.rs:16:28
-   --> $DIR/type-check-3.rs:16:28
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-3.rs:85:19
19    |
- LL |         asm!("{}", in(reg) _mm256_setzero_ps());
-    |                            ^^^^^^^^^^^^^^^^^^^
+ LL | global_asm!("{}", const const_foo(S));
22    |
22    |
-    = note: register class `reg` supports these types: i16, i32, i64, f32, f64
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
24 
- error: type `u8` cannot be used with this register class
-   --> $DIR/type-check-3.rs:18:32
-   --> $DIR/type-check-3.rs:18:32
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-3.rs:87:19
27    |
- LL |         asm!("{}", in(xmm_reg) 0u8);
-    |                                ^^^
+ LL | global_asm!("{}", const const_bar(0));
30    |
30    |
-    = note: register class `xmm_reg` supports these types: i32, i64, f32, f64, i8x16, i16x8, i32x4, i64x2, f32x4, f64x2
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
32 
32 
- error: `avx512bw` target feature is not enabled
-   --> $DIR/type-check-3.rs:27:29
+ error[E0658]: const operands for inline assembly are unstable
35    |
35    |
- LL |         asm!("{}", in(kreg) 0u64);
-    |
-    |
-    = note: this is required to use type `u64` with register class `kreg`
- warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:32:15
-    |
-    |
- LL |         asm!("{0} {0}", in(reg) 0i16);
-    |               ^^^ ^^^           ---- for this argument
-    |
-    = note: `#[warn(asm_sub_register)]` on by default
-    = help: use the `x` modifier to have the register formatted as `ax`
-    = help: or use the `r` modifier to keep the default formatting of `rax`
- warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:34:15
-    |
-    |
- LL |         asm!("{0} {0:x}", in(reg) 0i16);
-    |               ^^^                 ---- for this argument
-    |
-    = help: use the `x` modifier to have the register formatted as `ax`
-    = help: or use the `r` modifier to keep the default formatting of `rax`
- warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:36:15
-    |
-    |
- LL |         asm!("{}", in(reg) 0i32);
-    |               ^^           ---- for this argument
-    |
-    = help: use the `e` modifier to have the register formatted as `eax`
-    = help: or use the `r` modifier to keep the default formatting of `rax`
- warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:39:15
-    |
-    |
- LL |         asm!("{}", in(ymm_reg) 0i64);
-    |               ^^               ---- for this argument
-    |
-    = help: use the `x` modifier to have the register formatted as `xmm0`
-    = help: or use the `y` modifier to keep the default formatting of `ymm0`
- error: type `i8` cannot be used with this register class
-   --> $DIR/type-check-3.rs:50:28
-    |
-    |
- LL |         asm!("{}", in(reg) 0i8);
-    |
-    |
-    = note: register class `reg` supports these types: i16, i32, i64, f32, f64
-    = help: consider using the `reg_byte` register class instead
- 
- error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:62:33
-    |
- LL |         asm!("{:r}", inout(reg) 0u32 => val_f32);
-    |                                 ^^^^    ^^^^^^^ type `f32`
-    |                                 type `u32`
-    |
-    |
-    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
- 
- error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:64:33
-    |
- LL |         asm!("{:r}", inout(reg) 0u32 => val_ptr);
-    |                                 ^^^^    ^^^^^^^ type `*mut u8`
-    |                                 type `u32`
-    |
-    |
-    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
- 
- error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:66:33
-    |
- LL |         asm!("{:r}", inout(reg) main => val_u32);
-    |                                 ^^^^    ^^^^^^^ type `u32`
-    |                                 type `fn()`
-    |
-    |
-    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:82:25
-    |
-    |
- LL | global_asm!("{}", const S);
-    |
-    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:85:35
-    |
-    |
- LL | global_asm!("{}", const const_foo(S));
-    |
-    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:88:35
-    |
-    |
136 LL | global_asm!("{}", const const_bar(S));
+    |                   ^^^^^^^^^^^^^^^^^^
138    |
138    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
140 
- error: aborting due to 12 previous errors; 4 warnings emitted
+ error: aborting due to 5 previous errors
+ error: aborting due to 5 previous errors
142 
- For more information about this error, try `rustc --explain E0013`.
+ For more information about this error, try `rustc --explain E0658`.
144 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-3/type-check-3.stderr
To only update this specific test, also pass `--test-args asm/x86_64/type-check-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx512f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/type-check-3.rs:82:19
   |
LL | global_asm!("{}", const S);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/type-check-3.rs:84:19
   |
LL | global_asm!("{}", const const_foo(0));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/type-check-3.rs:85:19
   |
LL | global_asm!("{}", const const_foo(S));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/type-check-3.rs:87:19
   |
LL | global_asm!("{}", const const_bar(0));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/type-check-3.rs:88:19
   |
LL | global_asm!("{}", const const_bar(S));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable

---

---- [ui] ui/asm/type-check-1.rs stdout ----
diff of stderr:

25 LL |         asm!("{}", const const_bar(x));
26    |                                    ^ non-constant value
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:37:20
+    |
+    |
+ LL |         asm!("{}", const x);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:39:20
+    |
+ LL |         asm!("{}", const const_foo(0));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:40:20
+    |
+ LL |         asm!("{}", const const_foo(x));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:42:20
+    |
+ LL |         asm!("{}", const const_bar(0));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:43:20
+    |
+ LL |         asm!("{}", const const_bar(x));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:48:20
+    |
+ LL |         asm!("{}", const 0);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:49:20
+    |
+ LL |         asm!("{}", const 0i32);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:50:20
+    |
+ LL |         asm!("{}", const 0i128);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:51:20
+    |
+ LL |         asm!("{}", const 0f32);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:53:20
+    |
+ LL |         asm!("{}", const 0 as *mut u8);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:55:20
+    |
+ LL |         asm!("{}", const &0);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:62:19
+    |
+ LL | global_asm!("{}", const 0);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:63:19
+    |
+ LL | global_asm!("{}", const 0i32);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:64:19
+    |
+ LL | global_asm!("{}", const 0i128);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:65:19
+    |
+ LL | global_asm!("{}", const 0f32);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: const operands for inline assembly are unstable
+   --> $DIR/type-check-1.rs:67:19
+    |
+ LL | global_asm!("{}", const 0 as *mut u8);
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_const)]` to the crate attributes to enable
+ 
+ 
28 error[E0308]: mismatched types
29   --> $DIR/type-check-1.rs:51:26
30    |

106    = note:     expected type `{integer}`
107            found raw pointer `*mut u8`
- error: aborting due to 13 previous errors
+ error: aborting due to 29 previous errors
110 
- Some errors have detailed explanations: E0277, E0308, E0435.
- Some errors have detailed explanations: E0277, E0308, E0435.
+ Some errors have detailed explanations: E0277, E0308, E0435, E0658.
112 For more information about an error, try `rustc --explain E0277`.
113 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/type-check-1.stderr
To only update this specific test, also pass `--test-args asm/type-check-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:37:26
   |
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const x);
   |                          ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:40:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_foo(x));
   |                                    ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:43:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_bar(x));
   |                                    ^ non-constant value
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:37:20
   |
   |
LL |         asm!("{}", const x);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:39:20
   |
LL |         asm!("{}", const const_foo(0));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:40:20
   |
LL |         asm!("{}", const const_foo(x));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:42:20
   |
LL |         asm!("{}", const const_bar(0));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:43:20
   |
LL |         asm!("{}", const const_bar(x));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:48:20
   |
LL |         asm!("{}", const 0);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:49:20
   |
LL |         asm!("{}", const 0i32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:50:20
   |
LL |         asm!("{}", const 0i128);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:51:20
   |
LL |         asm!("{}", const 0f32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:53:20
   |
LL |         asm!("{}", const 0 as *mut u8);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:55:20
   |
LL |         asm!("{}", const &0);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:62:19
   |
LL | global_asm!("{}", const 0);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:63:19
   |
LL | global_asm!("{}", const 0i32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:64:19
   |
LL | global_asm!("{}", const 0i128);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:65:19
   |
LL | global_asm!("{}", const 0f32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/type-check-1.rs:67:19
   |
LL | global_asm!("{}", const 0 as *mut u8);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:51:26
   |
LL |         asm!("{}", const 0f32);
   |                          ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:53:26
   |
   |
LL |         asm!("{}", const 0 as *mut u8);
   |                          ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:55:26
   |
   |
LL |         asm!("{}", const &0);
   |                          ^^ expected integer, found `&{integer}`
help: consider removing the borrow
   |
   |
LL -         asm!("{}", const &0);
LL +         asm!("{}", const 0);

error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:13:29
   |
   |
LL |         asm!("{}", out(reg) 1 + 2);
   |                             ^^^^^ cannot assign to this expression
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:15:31
   |
   |
LL |         asm!("{}", inout(reg) 1 + 2);
   |                               ^^^^^ cannot assign to this expression
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:21:28
   |
   |
LL |         asm!("{}", in(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:23:29
   |
   |
LL |         asm!("{}", out(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:25:31
   |
   |
LL |         asm!("{}", inout(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:65:25
   |
   |
LL | global_asm!("{}", const 0f32);
   |                         ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:67:25
   |
   |
LL | global_asm!("{}", const 0 as *mut u8);
   |                         ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error: aborting due to 29 previous errors

Some errors have detailed explanations: E0277, E0308, E0435, E0658.
For more information about an error, try `rustc --explain E0277`.
---
test result: FAILED. 12231 passed; 13 failed; 110 ignored; 0 measured; 0 filtered out; finished in 128.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:26
