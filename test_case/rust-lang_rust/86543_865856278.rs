plain
.......................F............................................................................ 1300/12009
.................................................................................................... 1400/12009
.........................................................iiii.ii.i..............i................... 1500/12009
.................................................................................................... 1600/12009
..........................................................................i.F....................... 1700/12009
....................................................................F......F........................ 1800/12009
.................i.................................................................................. 2000/12009
.................................................................................................... 2100/12009
.................................................................................................... 2200/12009
.................................................................................................... 2300/12009
---
....................................i..i............................................................ 2800/12009
.................................................................................................... 2900/12009
.................................................iiiii.............................................. 3000/12009
.................................................................................................... 3100/12009
..................F...............F.................F............................................... 3200/12009
.................................................................................................... 3400/12009
.................................................................................................... 3500/12009
.................................................................................................... 3600/12009
.............................................................ii..................................... 3700/12009
.............................................................ii..................................... 3700/12009
..............................................................................i..................... 3800/12009
........................i........................................................................... 3900/12009
....F..F.....F................F...........F.............FF.F.F...................................... 4000/12009
...........................................................F........................................ 4100/12009
.................................................................................................... 4300/12009
.................................................................................................... 4400/12009
.................................................................................................... 4500/12009
.......................ii........................................................................... 4600/12009
.......................ii........................................................................... 4600/12009
...............i.................................................................................... 4700/12009
...........................................................................................F........ 4800/12009
......................................F..........F.................................................. 4900/12009
.................................................................................................... 5100/12009
.................................................................................................... 5200/12009
.................................................................................................... 5300/12009
............................i..i.................................................................... 5400/12009
---
..................................................i................................F................ 6200/12009
.......................i............................................................................ 6300/12009
...............F.....................................................................ii.ii.......i.. 6400/12009
.i.................................................................................................. 6500/12009
..............................i....i..................................iFF.F......F................i. 6600/12009
....F.....FF........FF......F.F.....F...................F.FF.FFFFFF.F...F.F..F.FF.......F....F.FF.FF 6700/12009
F.F.FF.FF..F...F...F...F.FFFFF.........i.......FFF..FFFF.F.F........F.........F.....FF...F.F.FF..... 6800/12009
............................................ii.................................................i.... 7000/12009
.........................................................................................F.......... 7100/12009
.................................................................................................... 7200/12009
.................................................................................................... 7300/12009
---
.................................................................................................... 8500/12009
i................................................................................................... 8600/12009
.................................................................................................... 8700/12009
.................................................................................................... 8800/12009
............................................................FF...F.................................. 8900/12009
.......F............................................................................................ 9000/12009
.................................................................................................... 9200/12009
.................................................................................................... 9300/12009
.................................................................................................... 9400/12009
.................................................................................................... 9500/12009
.................................................................................................... 9500/12009
.................................................................................................... 9600/12009
....................................F............................................................... 9700/12009
.........................................ii.......i................................................. 9800/12009
.........................................F.F........................................................ 9900/12009
.................................................................................................... 10100/12009
..................................................F................................................. 10200/12009
.................................................................................................... 10300/12009
.................................................................................................... 10400/12009
.................................................................................................... 10400/12009
.................................................................................................... 10500/12009
.................................................................................................... 10600/12009
.................................................................................................... 10700/12009
......ii.......................................i.................................................... 10800/12009
.................................................................................................... 10900/12009
.................................................................................................... 11000/12009
.................................................................................................... 11100/12009
...F.............FF....FF.F.F....................................................................... 11200/12009
..................................................................................ii................ 11300/12009
............................................................................F.F..................... 11400/12009
.................................................................................................... 11600/12009
............................F....................................................................... 11700/12009
.................................................................................................... 11800/12009
.................................................................................................... 11800/12009
..........F........F................................................................................ 11900/12009
i.iF..F............................................................................................. 12000/12009
failures:

---- [ui] ui/associated-type-bounds/type-alias.rs stdout ----
diff of stderr:
diff of stderr:

- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:5:25
-    |
- LL | type _TaWhere1<T> where T: Iterator<Item: Copy> = T;
-    |
-    |
-    = note: `#[warn(type_alias_bounds)]` on by default
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere1<T>  = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:6:25
-    |
- LL | type _TaWhere2<T> where T: Iterator<Item: 'static> = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere2<T>  = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:7:25
-    |
- LL | type _TaWhere3<T> where T: Iterator<Item: 'static> = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere3<T>  = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:8:25
-    |
- LL | type _TaWhere4<T> where T: Iterator<Item: 'static + Copy + Send> = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere4<T>  = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:9:25
-    |
- LL | type _TaWhere5<T> where T: Iterator<Item: for<'a> Into<&'a u8>> = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere5<T>  = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias.rs:10:25
-    |
- LL | type _TaWhere6<T> where T: Iterator<Item: Iterator<Item: Copy>> = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaWhere6<T>  = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:12:20
-    |
- LL | type _TaInline1<T: Iterator<Item: Copy>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline1<T> = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:13:20
-    |
- LL | type _TaInline2<T: Iterator<Item: 'static>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline2<T> = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:14:20
-    |
- LL | type _TaInline3<T: Iterator<Item: 'static>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline3<T> = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:15:20
-    |
- LL | type _TaInline4<T: Iterator<Item: 'static + Copy + Send>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline4<T> = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:16:20
-    |
- LL | type _TaInline5<T: Iterator<Item: for<'a> Into<&'a u8>>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline5<T> = T;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias.rs:17:20
-    |
- LL | type _TaInline6<T: Iterator<Item: Iterator<Item: Copy>>> = T;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type _TaInline6<T> = T;
- 
- warning: 12 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args associated-type-bounds/type-alias.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/type-alias" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/type-alias/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/cast/cast-char.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/cast-char.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-char" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-char/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/conditional-compilation/cfg-attr-multi-true.rs stdout ----
diff of stderr:

24 LL |         MustUseDeprecated {}
26 
26 
- warning: unused `MustUseDeprecated` that must be used
-   --> $DIR/cfg-attr-multi-true.rs:19:5
-    |
- LL |     MustUseDeprecated::new();
-    |
- note: the lint level is defined here
-   --> $DIR/cfg-attr-multi-true.rs:7:9
-    |
---
To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-multi-true.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated struct `MustUseDeprecated`
   |
   |
LL | impl MustUseDeprecated { //~ warning: use of deprecated
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated struct `MustUseDeprecated`
   |
   |
LL |     MustUseDeprecated::new(); //~ warning: use of deprecated


warning: use of deprecated struct `MustUseDeprecated`
   |
   |
LL |     fn new() -> MustUseDeprecated { //~ warning: use of deprecated


warning: use of deprecated struct `MustUseDeprecated`
   |
   |
LL |         MustUseDeprecated {} //~ warning: use of deprecated

warning: 4 warnings emitted



------------------------------------------


---- [ui] ui/const-generics/const-parameter-uppercase-lint.rs#full stdout ----

error in revision `full`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-parameter-uppercase-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-parameter-uppercase-lint.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-parameter-uppercase-lint.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/const-generics/const-parameter-uppercase-lint.rs#min stdout ----

error in revision `min`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-parameter-uppercase-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-parameter-uppercase-lint.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-parameter-uppercase-lint.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
diff of 64bit.stderr:

19    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
20    = note: the raw bytes of the constant (size: 0, align: 1) {}
21 
- warning: the type `!` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:5:14
-    |
- LL |     unsafe { std::mem::transmute(()) }
-    |              |
-    |              |
-    |              this code causes undefined behavior when executed
-    |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
-    |
-    = note: `#[warn(invalid_value)]` on by default
-    = note: the `!` type has no valid value
- 
- warning: the type `Empty` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:17:35
-    |
- LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
-    |                                   |
-    |                                   |
-    |                                   this code causes undefined behavior when executed
-    |                                   help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
-    |
-    = note: enums with no variants have no valid value
- error: aborting due to 2 previous errors; 2 warnings emitted
+ error: aborting due to 2 previous errors
46 
47 For more information about this error, try `rustc --explain E0080`.
47 For more information about this error, try `rustc --explain E0080`.
48 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
...
...
LL | const FOO: [Empty; 3] = [foo(); 3];
   |                          ----- inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:26
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:1
   |
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0]: encountered a value of uninhabited type Empty
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/enable-unstable-lib-feature.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enable-unstable-lib-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enable-unstable-lib-feature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enable-unstable-lib-feature/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/enum/enum-discrim-too-small2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum/enum-discrim-too-small2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-too-small2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-too-small2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/enum/enum-size-variance.rs stdout ----
diff of stderr:

- warning: enum variant is more than three times larger (32 bytes) than the next largest
-   --> $DIR/enum-size-variance.rs:18:5
-    |
- LL |     L(i64, i64, i64, i64),
-    |
- note: the lint level is defined here
-   --> $DIR/enum-size-variance.rs:3:9
-    |
-    |
- LL | #![warn(variant_size_differences)]
- 
- warning: 1 warning emitted
- 
- 
---
To only update this specific test, also pass `--test-args enum/enum-size-variance.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum/enum-size-variance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-size-variance/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-size-variance/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/issue-52398.rs:17:5
-    |
- LL | /     move || {
- LL | |         A.test(yield);
- LL | |     };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: unused generator that must be used
-   --> $DIR/issue-52398.rs:24:5
-    |
-    |
- LL | /     static move || {
- LL | |         yield *y.borrow();
- LL | |         return "Done";
- LL | |     };
-    |
-    |
-    = note: generators are lazy and do nothing unless resumed
- warning: 2 warnings emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/issue-52398.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-52398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-52398/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-52398/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/issue-57084.rs:22:5
-    |
- LL | /     || {
- LL | |         let _to_pin = with(move || println!("{:p}", data));
- LL | |         loop {
- LL | |             yield
- LL | |         }
- LL | |     };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/issue-57084.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-57084.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57084/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-57084/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/match-bindings.rs:12:5
-    |
- LL | /     || {
- LL | |         loop {
- LL | |             if let true = true {
- LL | |                 match Enum::A(String::new()) {
- LL | |         }
- LL | |     };
-    | |______^
-    |
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/match-bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/match-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/match-bindings/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/match-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/reborrow-mut-upvar.rs:6:5
-    |
- LL | /     || {
- LL | |         {
- LL | |             let _baz = &*bar;
- LL | |             yield;
- ...  |
- LL | |         *bar = 2;
- LL | |     };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/reborrow-mut-upvar.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/reborrow-mut-upvar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/reborrow-mut-upvar/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/reborrow-mut-upvar/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/too-live-local-in-immovable-gen.rs:8:9
-    |
- LL | /         static move || {
- LL | |             // Tests that the generator transformation finds out that `a` is not live
- LL | |             // during the yield expression. Type checking will also compute liveness
- LL | |             // and it should also find out that `a` is not live.
- ...  |
- LL | |             let _ = &a;
- LL | |         };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/too-live-local-in-immovable-gen.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/too-live-local-in-immovable-gen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/too-live-local-in-immovable-gen/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/too-live-local-in-immovable-gen/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/yield-in-args-rev.rs:13:5
-    |
- LL | /     || {
- LL | |         let b = true;
- LL | |         foo(yield, &b);
- LL | |     };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-args-rev/yield-in-args-rev.stderr
To only update this specific test, also pass `--test-args generator/yield-in-args-rev.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-in-args-rev.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-args-rev/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-args-rev/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/yield-in-box.rs:9:5
-    |
- LL | /     || {
- LL | |         let y = 2u32;
- LL | |         {
- LL | |             let _t = box (&x, yield 0, &y);
- LL | |         }
- LL | |     };
-    | |______^
-    |
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/yield-in-box.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-in-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/yield-in-initializer.rs:6:5
-    |
- LL | /     static || {
- LL | |         loop {
- LL | |             // Test that `opt` is not live across the yield, even when borrowed in a loop
- LL | |             // See https://github.com/rust-lang/rust/issues/52792
- LL | |         }
- LL | |     };
-    | |______^
-    |
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/yield-in-initializer.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-in-initializer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-initializer/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-initializer/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/yield-subtype.rs:11:5
-    |
- LL | /     || {
- LL | |         yield a;
- LL | |         yield b;
- LL | |     };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args generator/yield-subtype.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-subtype/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-subtype/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/generics/generic-no-mangle.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/generic-no-mangle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-no-mangle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-no-mangle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused closure that must be used
-   --> $DIR/issue-1460.rs:6:5
-    |
- LL |     {|i: u32| if 1 == i { }};
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: closures are lazy and do nothing unless called
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args issues/issue-1460.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-1460.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1460/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1460/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-16250.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16250.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16250" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16250/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused closure that must be used
-   --> $DIR/issue-16256.rs:6:5
-    |
- LL |     |c: u8| buf.push(c);
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: closures are lazy and do nothing unless called
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args issues/issue-16256.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16256.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16256/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16256/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-45562.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45562.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45562" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45562/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-63364.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-63364.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-63364" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-63364/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-79744.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-79744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-79744" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-79744/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/command-line-lint-group-deny.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-deny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-deny" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-deny/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/command-line-lint-group-warn.rs stdout ----
diff of stderr:

- warning: variable `_InappropriateCamelCasing` should have a snake case name
-   --> $DIR/command-line-lint-group-warn.rs:5:9
-    |
- LL |     let _InappropriateCamelCasing = true;
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_inappropriate_camel_casing`
-    |
-    = note: `-W non-snake-case` implied by `-W bad-style`
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args lint/command-line-lint-group-warn.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-warn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-W" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-warn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/command-line-lint-group-forbid.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-forbid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-forbid" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-forbid/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/clashing-extern-fn.rs stdout ----
diff of stderr:

213    = note: expected `unsafe extern "C" fn() -> usize`
214               found `unsafe extern "C" fn() -> Option<UnsafeCell<NonZeroUsize>>`
215 
- warning: `extern` block uses type `Option<TransparentNoNiche>`, which is not FFI-safe
-   --> $DIR/clashing-extern-fn.rs:410:55
-    |
- LL |             fn hidden_niche_transparent_no_niche() -> Option<TransparentNoNiche>;
-    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
-    |
-    = note: `#[warn(improper_ctypes)]` on by default
-    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
-    = note: enum has no representation hint
- 
- warning: `extern` block uses type `Option<UnsafeCell<NonZeroUsize>>`, which is not FFI-safe
-   --> $DIR/clashing-extern-fn.rs:414:46
-    |
- LL |             fn hidden_niche_unsafe_cell() -> Option<UnsafeCell<NonZeroUsize>>;
-    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
-    |
-    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
-    = note: enum has no representation hint
- warning: 19 warnings emitted
+ warning: 17 warnings emitted
236 
237 
---
To only update this specific test, also pass `--test-args lint/clashing-extern-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/clashing-extern-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `clash` redeclared with a different signature
   |
   |
LL |             fn clash(x: u8);
   |             ---------------- `clash` previously declared here
...
LL |             fn clash(x: u64); //~ WARN `clash` redeclared with a different signature
   |             ^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:5:9
   |
   |
LL | #![warn(clashing_extern_declarations)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`

warning: `extern_link_name` redeclared with a different signature
   |
   |
LL | /     #[link_name = "extern_link_name"]
LL | |     fn some_new_name(x: i16);
   | |_____________________________- `extern_link_name` previously declared here
...
LL |           fn extern_link_name(x: u32);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `some_other_extern_link_name` redeclares `some_other_new_name` with a different signature
   |
   |
LL |       fn some_other_new_name(x: i16);
   |       ------------------------------- `some_other_new_name` previously declared here
...
LL | /         #[link_name = "some_other_new_name"]
LL | |         //~^ WARN `some_other_extern_link_name` redeclares `some_other_new_name` with a different
LL | |         fn some_other_extern_link_name(x: u32);
   | |_______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `other_both_names_different` redeclares `link_name_same` with a different signature
   |
   |
LL | /     #[link_name = "link_name_same"]
LL | |     fn both_names_different(x: i16);
   | |____________________________________- `link_name_same` previously declared here
...
LL | /         #[link_name = "link_name_same"]
LL | |         //~^ WARN `other_both_names_different` redeclares `link_name_same` with a different
LL | |         fn other_both_names_different(x: u32);
   | |______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `different_mod` redeclared with a different signature
   |
   |
LL |         fn different_mod(x: u8);
   |         ------------------------ `different_mod` previously declared here
...
LL |         fn different_mod(x: u64); //~ WARN `different_mod` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`

warning: `variadic_decl` redeclared with a different signature
   |
   |
LL |     fn variadic_decl(x: u8, ...);
   |     ----------------------------- `variadic_decl` previously declared here
...
LL |         fn variadic_decl(x: u8); //~ WARN `variadic_decl` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(u8, ...)`
              found `unsafe extern "C" fn(u8)`

warning: `weigh_banana` redeclared with a different signature
   |
   |
LL |             fn weigh_banana(count: *const Banana) -> u64;
   |             --------------------------------------------- `weigh_banana` previously declared here
...
LL |             fn weigh_banana(count: *const Banana) -> u64;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(*const one::Banana) -> u64`
              found `unsafe extern "C" fn(*const three::Banana) -> u64`

warning: `draw_point` redeclared with a different signature
   |
   |
LL |             fn draw_point(p: Point);
   |             ------------------------ `draw_point` previously declared here
...
LL |             fn draw_point(p: Point);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(sameish_members::a::Point)`
              found `unsafe extern "C" fn(sameish_members::b::Point)`

warning: `origin` redeclared with a different signature
   |
LL |             fn origin() -> Point3;
LL |             fn origin() -> Point3;
   |             ---------------------- `origin` previously declared here
...
LL |             fn origin() -> Point3; //~ WARN `origin` redeclared with a different signature
   |             ^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> same_sized_members_clash::a::Point3`
              found `unsafe extern "C" fn() -> same_sized_members_clash::b::Point3`

warning: `transparent_incorrect` redeclared with a different signature
   |
LL |             fn transparent_incorrect() -> T;
LL |             fn transparent_incorrect() -> T;
   |             -------------------------------- `transparent_incorrect` previously declared here
LL |             fn transparent_incorrect() -> isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> T`
   = note: expected `unsafe extern "C" fn() -> T`
              found `unsafe extern "C" fn() -> isize`

warning: `missing_return_type` redeclared with a different signature
   |
LL |             fn missing_return_type() -> usize;
LL |             fn missing_return_type() -> usize;
   |             ---------------------------------- `missing_return_type` previously declared here
LL |             fn missing_return_type();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn()`

warning: `non_zero_usize` redeclared with a different signature
   |
LL |             fn non_zero_usize() -> core::num::NonZeroUsize;
LL |             fn non_zero_usize() -> core::num::NonZeroUsize;
   |             ----------------------------------------------- `non_zero_usize` previously declared here
LL |             fn non_zero_usize() -> usize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> NonZeroUsize`
   = note: expected `unsafe extern "C" fn() -> NonZeroUsize`
              found `unsafe extern "C" fn() -> usize`

warning: `non_null_ptr` redeclared with a different signature
   |
   |
LL |             fn non_null_ptr() -> core::ptr::NonNull<usize>;
   |             ----------------------------------------------- `non_null_ptr` previously declared here
LL |             fn non_null_ptr() -> *const usize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   |
   = note: expected `unsafe extern "C" fn() -> NonNull<usize>`
              found `unsafe extern "C" fn() -> *const usize`

warning: `option_non_zero_usize_incorrect` redeclared with a different signature
   |
LL |             fn option_non_zero_usize_incorrect() -> usize;
LL |             fn option_non_zero_usize_incorrect() -> usize;
   |             ---------------------------------------------- `option_non_zero_usize_incorrect` previously declared here
LL |             fn option_non_zero_usize_incorrect() -> isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> isize`

warning: `option_non_null_ptr_incorrect` redeclared with a different signature
   |
   |
LL |             fn option_non_null_ptr_incorrect() -> *const usize;
   |             --------------------------------------------------- `option_non_null_ptr_incorrect` previously declared here
...
LL |             fn option_non_null_ptr_incorrect() -> *const isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn() -> *const usize`
   = note: expected `unsafe extern "C" fn() -> *const usize`
              found `unsafe extern "C" fn() -> *const isize`

warning: `hidden_niche_transparent_no_niche` redeclared with a different signature
   |
   |
LL |             fn hidden_niche_transparent_no_niche() -> usize;
   |             ------------------------------------------------ `hidden_niche_transparent_no_niche` previously declared here
...
LL |             fn hidden_niche_transparent_no_niche() -> Option<TransparentNoNiche>;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> Option<TransparentNoNiche>`

warning: `hidden_niche_unsafe_cell` redeclared with a different signature
   |
   |
LL |             fn hidden_niche_unsafe_cell() -> usize;
   |             --------------------------------------- `hidden_niche_unsafe_cell` previously declared here
...
LL |             fn hidden_niche_unsafe_cell() -> Option<UnsafeCell<NonZeroUsize>>;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> Option<UnsafeCell<NonZeroUsize>>`
warning: 17 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/deny-overflowing-literals.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/deny-overflowing-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/deny-overflowing-literals" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/deny-overflowing-literals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/expr_attr_paren_order.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/expr_attr_paren_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/expr_attr_paren_order" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/expr_attr_paren_order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/forbid-group-group-1.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/forbid-group-group-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/forbid-group-group-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/fn_must_use.rs stdout ----
diff of stderr:

- warning: unused return value of `need_to_use_this_value` that must be used
-   --> $DIR/fn_must_use.rs:55:5
- LL |     need_to_use_this_value();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
- note: the lint level is defined here
- note: the lint level is defined here
-   --> $DIR/fn_must_use.rs:3:9
-    |
- LL | #![warn(unused_must_use)]
-    |         ^^^^^^^^^^^^^^^
-    = note: it's important
- 
- warning: unused return value of `MyStruct::need_to_use_this_method_value` that must be used
-   --> $DIR/fn_must_use.rs:60:5
-    |
- LL |     m.need_to_use_this_method_value();
- 
- 
- warning: unused return value of `EvenNature::is_even` that must be used
-   --> $DIR/fn_must_use.rs:61:5
-    |
- LL |     m.is_even(); // trait method!
-    |
-    = note: no side effects
- 
- 
- warning: unused return value of `MyStruct::need_to_use_this_associated_function_value` that must be used
-   --> $DIR/fn_must_use.rs:64:5
-    |
- LL |     MyStruct::need_to_use_this_associated_function_value();
- 
- 
- warning: unused return value of `std::cmp::PartialEq::eq` that must be used
-   --> $DIR/fn_must_use.rs:70:5
-    |
- LL |     2.eq(&3);
- 
- 
- warning: unused return value of `std::cmp::PartialEq::eq` that must be used
-   --> $DIR/fn_must_use.rs:71:5
-    |
- LL |     m.eq(&n);
- 
- warning: unused comparison that must be used
-   --> $DIR/fn_must_use.rs:74:5
-    |
---
To only update this specific test, also pass `--test-args lint/fn_must_use.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/fn_must_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/fn_must_use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/fn_must_use/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/force-warn/force-lint-group-allow-all-warnings.rs stdout ----
diff of stderr:

- warning: function `FUNCTION` should have a snake case name
-   --> $DIR/force-lint-group-allow-all-warnings.rs:6:8
- LL | pub fn FUNCTION() {}
- LL | pub fn FUNCTION() {}
-    |        ^^^^^^^^ help: convert the identifier to snake case: `function`
-    |
-    = note: warning forced by `force-warns` commandline option
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args lint/force-warn/force-lint-group-allow-all-warnings.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-lint-group-allow-all-warnings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-group-allow-all-warnings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "nonstandard_style" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-group-allow-all-warnings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

11    |         ^^^^^^^^
12    = note: `#[deny(dead_code)]` implied by `#[deny(warnings)]`
13 
- error: constant `foo` should have an upper case name
-   --> $DIR/issue-17718-const-naming.rs:4:7
-    |
- LL | const foo: isize = 3;
-    |       ^^^ help: convert the identifier to upper case (notice the capitalization): `FOO`
- note: the lint level is defined here
-   --> $DIR/issue-17718-const-naming.rs:2:9
-    |
-    |
- LL | #![deny(warnings)]
-    |         ^^^^^^^^
-    = note: `#[deny(non_upper_case_globals)]` implied by `#[deny(warnings)]`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
28 
29 
---
To only update this specific test, also pass `--test-args lint/issue-17718-const-naming.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-17718-const-naming.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-17718-const-naming" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-17718-const-naming/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: constant is never used: `foo`
  --> /checkout/src/test/ui/lint/issue-17718-const-naming.rs:4:1
   |
LL | const foo: isize = 3;
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/issue-17718-const-naming.rs:2:9
   |
---
---- [ui] ui/lint/issue-14309.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-14309.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-14309" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-14309/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/issue-66362-no-snake-case-warning-for-field-puns.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-66362-no-snake-case-warning-for-field-puns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-66362-no-snake-case-warning-for-field-puns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-66362-no-snake-case-warning-for-field-puns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-2.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-2.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-2.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/lint-ctypes-73249-2.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-2.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-2.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-3.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/lint-ctypes-73249-3.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-5.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/lint-ctypes-73251-1.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73251-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-1.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-1.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/lint-ctypes-73251-1.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73251-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-1.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-1.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-ctypes-73251-2.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73251-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-2.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-2.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/lint-ctypes-73251-2.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73251-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-2.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73251-2.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-5.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-ctypes-enum.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-deref-nullptr.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-deref-nullptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-deref-nullptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-deref-nullptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-ctypes.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-ctypes-fn.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-group-nonstandard-style.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-group-nonstandard-style" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-group-nonstandard-style/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: type `snake_case` should have an upper camel case name
   |
   |
LL |         struct snake_case; //~ WARN should have an upper camel
   |                ^^^^^^^^^^ help: convert the identifier to upper camel case: `SnakeCase`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:18:17
   |
LL |         #![warn(nonstandard_style)]
---
---- [ui] ui/lint/lint-lowercase-static-const-pattern.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-lowercase-static-const-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-lowercase-static-const-pattern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-lowercase-static-const-pattern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-missing-copy-implementations.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-missing-copy-implementations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-missing-copy-implementations" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-missing-copy-implementations/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-identifiers-suggestion-reserved.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-identifiers-suggestion-reserved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-identifiers-suggestion-reserved" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-identifiers-suggestion-reserved/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-functions.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-crate-2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-crate-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name" "NonSnakeCase" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-lifetimes.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-modules.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-modules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-modules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-snake-case-crate.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-snake-case-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-uppercase-associated-const.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-uppercase-associated-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-uppercase-associated-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-uppercase-associated-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-non-uppercase-statics.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-non-uppercase-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-uppercase-statics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-uppercase-statics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-nonstandard-style-unicode-2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-nonstandard-style-unicode-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-nonstandard-style-unicode-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-nonstandard-style-unicode-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-nonstandard-style-unicode-3.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-nonstandard-style-unicode-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-nonstandard-style-unicode-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-nonstandard-style-unicode-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-owned-heap-memory.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-owned-heap-memory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-owned-heap-memory" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-owned-heap-memory/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-range-endpoint-overflow.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-range-endpoint-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-range-endpoint-overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-range-endpoint-overflow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-shorthand-field.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-shorthand-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-shorthand-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-shorthand-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-type-limits2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-limits2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "unused-comparisons" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-type-limits.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-limits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "unused-comparisons" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-type-overflow2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-type-overflow.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-type-limits3.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-limits3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "unused-comparisons" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-limits3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/lint-unexported-no-mangle.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unexported-no-mangle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "private_no_mangle_fns" "-F" "no_mangle_const_items" "-F" "private_no_mangle_statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`
warning: 6 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-uppercase-variables.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-uppercase-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-uppercase-variables" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-uppercase-variables/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning[E0170]: pattern binding `Foo` is named the same as one of the variants of the type `Foo`
   |
LL |         Foo => {}
LL |         Foo => {}
   |         ^^^ help: to match on the variant, qualify the path: `Foo::Foo`
   |
   = note: `#[warn(bindings_with_variant_name)]` on by default

warning[E0170]: pattern binding `Foo` is named the same as one of the variants of the type `Foo`
   |
LL |     let Foo = foo::Foo::Foo;
LL |     let Foo = foo::Foo::Foo;
   |         ^^^ help: to match on the variant, qualify the path: `Foo::Foo`

warning[E0170]: pattern binding `Foo` is named the same as one of the variants of the type `Foo`
   |
   |
LL |     fn in_param(Foo: foo::Foo) {}
   |                 ^^^ help: to match on the variant, qualify the path: `Foo::Foo`
warning: unused variable: `Foo`
  --> /checkout/src/test/ui/lint/lint-uppercase-variables.rs:22:9
   |
LL |         Foo => {}
---

warning: unused variable: `Foo`
  --> /checkout/src/test/ui/lint/lint-uppercase-variables.rs:33:17
   |
LL |     fn in_param(Foo: foo::Foo) {}
   |                 ^^^ help: if this is intentional, prefix it with an underscore: `_Foo`
warning: 6 warnings emitted

For more information about this error, try `rustc --explain E0170`.


------------------------------------------


---- [ui] ui/lint/must_use-array.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/must_use-in-stdlib-traits.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-in-stdlib-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-in-stdlib-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-in-stdlib-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/must_use-unit.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-unit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/must_use-tuple.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-tuple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-tuple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
- 
- warning: unused comparison that must be used
-   --> $DIR/must-use-ops.rs:15:5
-    |
- LL |     val != 1;
- 
- warning: unused comparison that must be used
-   --> $DIR/must-use-ops.rs:16:5
-    |
---
- 
- warning: unused bitwise operation that must be used
-   --> $DIR/must-use-ops.rs:34:5
-    |
- LL |     5 << val;
- 
- warning: unused bitwise operation that must be used
-   --> $DIR/must-use-ops.rs:35:5
-    |
-    |
- LL |     5 >> val;
- 
- warning: unused unary operation that must be used
-   --> $DIR/must-use-ops.rs:38:5
-    |
-    |
- LL |     !val;
- 
- warning: unused unary operation that must be used
-   --> $DIR/must-use-ops.rs:39:5
-    |
-    |
- LL |     -val;
-    |     ^^^^
- 
- warning: unused unary operation that must be used
-   --> $DIR/must-use-ops.rs:40:5
-    |
- LL |     *val_pointer;
- 
- warning: 21 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args lint/must-use-ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must-use-ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must-use-ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must-use-ops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/must_use-trait.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/lint/opaque-ty-ffi-unsafe.rs#full_tait stdout ----

error in revision `full_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/opaque-ty-ffi-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/opaque-ty-ffi-unsafe.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/opaque-ty-ffi-unsafe.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---


---- [ui] ui/lint/opaque-ty-ffi-unsafe.rs#min_tait stdout ----

error in revision `min_tait`: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/opaque-ty-ffi-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/opaque-ty-ffi-unsafe.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/opaque-ty-ffi-unsafe.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

11 LL | #![warn(elided_lifetimes_in_paths,
12    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
13 
- warning: variable `Social_exchange_psychology` should have a snake case name
-   --> $DIR/reasons.rs:30:9
-    |
- LL |     let Social_exchange_psychology = CheaterDetectionMechanism {};
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case (notice the capitalization): `social_exchange_psychology`
-    |
-    = note: people shouldn't have to change their usual style habits
-            to contribute to our project
-   --> $DIR/reasons.rs:9:5
-    |
- LL |     nonstandard_style,
-    |     ^^^^^^^^^^^^^^^^^
---
To only update this specific test, also pass `--test-args lint/reasons.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: hidden lifetime parameters in types are deprecated
   |
   |
LL |     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
   |                             ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: explicit anonymous lifetimes aid reasoning about ownership
  --> /checkout/src/test/ui/lint/reasons.rs:5:9
   |
LL | #![warn(elided_lifetimes_in_paths,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
---

---- [ui] ui/lint/special-upper-lower-cases.rs stdout ----
diff of stderr:

12 LL | struct 𝕟𝕠𝕥_𝕒_𝕔𝕒𝕞𝕖𝕝;
13    |        ^^^^^^^^^^^ should have an UpperCamelCase name
14 
- warning: static variable `𝗻𝗼𝗻𝘂𝗽𝗽𝗲𝗿𝗰𝗮𝘀𝗲` should have an upper case name
-   --> $DIR/special-upper-lower-cases.rs:17:8
-    |
- LL | static 𝗻𝗼𝗻𝘂𝗽𝗽𝗲𝗿𝗰𝗮𝘀𝗲: i32 = 1;
-    |        ^^^^^^^^^^^^ should have an UPPER_CASE name
-    = note: `#[warn(non_upper_case_globals)]` on by default
- 
- 
- warning: variable `𝓢𝓝𝓐𝓐𝓐𝓐𝓚𝓔𝓢` should have a snake case name
-   --> $DIR/special-upper-lower-cases.rs:21:9
-    |
- LL |     let 𝓢𝓝𝓐𝓐𝓐𝓐𝓚𝓔𝓢 = 1;
-    |         ^^^^^^^^^ should have a snake_case name
-    = note: `#[warn(non_snake_case)]` on by default
- 
- warning: 4 warnings emitted
+ warning: 2 warnings emitted
---
To only update this specific test, also pass `--test-args lint/special-upper-lower-cases.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/special-upper-lower-cases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/special-upper-lower-cases" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/special-upper-lower-cases/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: type `𝕟𝕠𝕥𝕒𝕔𝕒𝕞𝕖𝕝` should have an upper camel case name
   |
LL | struct 𝕟𝕠𝕥𝕒𝕔𝕒𝕞𝕖𝕝;
LL | struct 𝕟𝕠𝕥𝕒𝕔𝕒𝕞𝕖𝕝;
   |        ^^^^^^^^^ should have an UpperCamelCase name
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `𝕟𝕠𝕥_𝕒_𝕔𝕒𝕞𝕖𝕝` should have an upper camel case name
   |
LL | struct 𝕟𝕠𝕥_𝕒_𝕔𝕒𝕞𝕖𝕝;
LL | struct 𝕟𝕠𝕥_𝕒_𝕔𝕒𝕞𝕖𝕝;
   |        ^^^^^^^^^^^ should have an UpperCamelCase name
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/type-overflow.rs stdout ----
diff of stderr:

- warning: literal out of range for `i8`
-   --> $DIR/type-overflow.rs:5:17
- LL |     let error = 255i8;
-    |                 ^^^^^
-    |
- note: the lint level is defined here
- note: the lint level is defined here
-   --> $DIR/type-overflow.rs:2:9
-    |
- LL | #![warn(overflowing_literals)]
-    |         ^^^^^^^^^^^^^^^^^^^^
-    = note: the literal `255i8` does not fit into the type `i8` whose range is `-128..=127`
-    = help: consider using the type `u8` instead
- 
- warning: literal out of range for `i8`
-   --> $DIR/type-overflow.rs:10:16
-    |
- LL |     let fail = 0b1000_0001i8;
-    |                ^^^^^^^^^^^^^ help: consider using the type `u8` instead: `0b1000_0001u8`
-    |
-    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into the type `i8` and will become `-127i8`
- 
- warning: literal out of range for `i64`
-   --> $DIR/type-overflow.rs:12:16
-    |
- LL |     let fail = 0x8000_0000_0000_0000i64;
-    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using the type `u64` instead: `0x8000_0000_0000_0000u64`
-    |
-    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into the type `i64` and will become `-9223372036854775808i64`
- 
- warning: literal out of range for `u32`
-   --> $DIR/type-overflow.rs:14:16
-    |
- LL |     let fail = 0x1_FFFF_FFFFu32;
-    |                ^^^^^^^^^^^^^^^^ help: consider using the type `u64` instead: `0x1_FFFF_FFFFu64`
-    |
-    = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into the type `u32` and will become `4294967295u32`
- 
- warning: literal out of range for `i128`
-   --> $DIR/type-overflow.rs:16:22
-    |
- LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
-    |
-    |
-    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into the type `i128` and will become `-170141183460469231731687303715884105728i128`
-    = help: consider using the type `u128` instead
- 
- warning: literal out of range for `i32`
-   --> $DIR/type-overflow.rs:19:16
-    |
- LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE;
-    |
-    |
-    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into the type `i32` and will become `-2i32`
-    = help: consider using the type `i128` instead
- 
- warning: literal out of range for `i8`
-   --> $DIR/type-overflow.rs:21:17
-    |
- LL |     let fail = -0b1111_1111i8;
-    |                 ^^^^^^^^^^^^^ help: consider using the type `i16` instead: `0b1111_1111i16`
-    |
-    = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into the type `i8`
-    = note: and the value `-0b1111_1111i8` will become `1i8`
- warning: 7 warnings emitted
- 
- 

---
To only update this specific test, also pass `--test-args lint/type-overflow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/type-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/suggestions.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: denote infinite loops with `loop { ... }`
   |
LL |     while true {
   |     ^^^^^^^^^^ help: use `loop`
   |
   |
   = note: `#[warn(while_true)]` on by default
warning: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/suggestions.rs:48:31
   |
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:21
   |
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:48:13
   |
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |             |
   |             help: remove this `mut`
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:9
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:54:13
   |
---
---- [ui] ui/lint/unused-borrows.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused-borrows.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused-borrows" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused-borrows/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/unreachable_pub-pub_crate.rs stdout ----
diff of stderr:

- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:14:5
- LL |     pub use std::fmt;
-    |     ---^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
- note: the lint level is defined here
-   --> $DIR/unreachable_pub-pub_crate.rs:10:9
-    |
- LL | #![warn(unreachable_pub)]
- LL | #![warn(unreachable_pub)]
-    |         ^^^^^^^^^^^^^^^
-    = help: or consider exporting it for use by other crates
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:15:24
-    |
- LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
-    |     |
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:18:5
-    |
- LL |     pub struct Hydrogen {
-    |     ---^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` field
-   --> $DIR/unreachable_pub-pub_crate.rs:20:9
-    |
- LL |         pub neutrons: usize,
-    |         ---^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `pub(crate)`
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:26:9
-    |
- LL |         pub fn count_neutrons(&self) -> usize { self.neutrons }
-    |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `pub(crate)`
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:30:5
-    |
- LL |     pub enum Helium {}
-    |     ---^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:31:5
-    |
- LL |     pub union Lithium { c1: usize, c2: u8 }
-    |     ---^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:32:5
-    |
- LL |     pub fn beryllium() {}
-    |     ---^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:33:5
-    |
- LL |     pub trait Boron {}
-    |     ---^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:34:5
-    |
- LL |     pub const CARBON: usize = 1;
-    |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:35:5
-    |
- LL |     pub static NITROGEN: usize = 2;
-    |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:36:5
-    |
- LL |     pub type Oxygen = bool;
-    |     ---^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:39:47
-    |
- LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
- ...
- ...
- LL |     define_empty_struct_with_visibility!(pub, Fluorine);
-    |     |                                    |
-    |     |                                    |
-    |     |                                    help: consider restricting its visibility: `pub(crate)`
-    |
-    = help: or consider exporting it for use by other crates
-    = help: or consider exporting it for use by other crates
-    = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub-pub_crate.rs:45:9
-    |
- LL |         pub fn catalyze() -> bool;
-    |         ---^^^^^^^^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `pub(crate)`
-    = help: or consider exporting it for use by other crates
- 
- warning: 14 warnings emitted
- 
---
To only update this specific test, also pass `--test-args lint/unreachable_pub-pub_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/lint/unreachable_pub.rs stdout ----
diff of stderr:

- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:10:5
- LL |     pub use std::fmt;
-    |     ---^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     help: consider restricting its visibility: `crate`
- note: the lint level is defined here
-   --> $DIR/unreachable_pub.rs:6:9
-    |
- LL | #![warn(unreachable_pub)]
- LL | #![warn(unreachable_pub)]
-    |         ^^^^^^^^^^^^^^^
-    = help: or consider exporting it for use by other crates
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:11:24
-    |
- LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
-    |     |
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:14:5
-    |
- LL |     pub struct Hydrogen {
-    |     ---^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` field
-   --> $DIR/unreachable_pub.rs:16:9
-    |
- LL |         pub neutrons: usize,
-    |         ---^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `crate`
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:22:9
-    |
- LL |         pub fn count_neutrons(&self) -> usize { self.neutrons }
-    |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `crate`
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:26:5
-    |
- LL |     pub enum Helium {}
-    |     ---^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:27:5
-    |
- LL |     pub union Lithium { c1: usize, c2: u8 }
-    |     ---^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:28:5
-    |
- LL |     pub fn beryllium() {}
-    |     ---^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:29:5
-    |
- LL |     pub trait Boron {}
-    |     ---^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:30:5
-    |
- LL |     pub const CARBON: usize = 1;
-    |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:31:5
-    |
- LL |     pub static NITROGEN: usize = 2;
-    |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:32:5
-    |
- LL |     pub type Oxygen = bool;
-    |     ---^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:35:47
-    |
- LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
- ...
- ...
- LL |     define_empty_struct_with_visibility!(pub, Fluorine);
-    |     |                                    |
-    |     |                                    |
-    |     |                                    help: consider restricting its visibility: `crate`
-    |
-    = help: or consider exporting it for use by other crates
-    = help: or consider exporting it for use by other crates
-    = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- warning: unreachable `pub` item
-   --> $DIR/unreachable_pub.rs:41:9
-    |
- LL |         pub fn catalyze() -> bool;
-    |         ---^^^^^^^^^^^^^^^^^^^^^^^
-    |         |
-    |         help: consider restricting its visibility: `crate`
-    = help: or consider exporting it for use by other crates
- 
- warning: 14 warnings emitted
- 
- 
- 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/unreachable_pub.stderr
To only update this specific test, also pass `--test-args lint/unreachable_pub.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/lint/uninitialized-zeroed.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused `Result` that must be used
-   --> $DIR/must-use-in-macro-55516.rs:9:5
-    |
- LL |     write!(&mut example, "{}", 42);
-    |
-    |
-    = note: `-W unused-must-use` implied by `-W unused`
-    = note: this `Result` may be an `Err` variant, which should be handled
-    = note: this warning originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args macros/must-use-in-macro-55516.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/must-use-in-macro-55516.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/must-use-in-macro-55516" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Wunused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/must-use-in-macro-55516/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- warning: unused generator that must be used
-   --> $DIR/issue-48623-generator.rs:15:5
-    |
- LL |     move || { d; yield; &mut *r };
-    |
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: `#[warn(unused_must_use)]` on by default
-    = note: generators are lazy and do nothing unless resumed
- warning: 1 warning emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48623-generator/issue-48623-generator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-48623-generator.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-48623-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48623-generator/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48623-generator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/print_type_sizes/repr-align.rs stdout ----
diff of stdout:

- print-type-size type: `E`: 32 bytes, alignment: 16 bytes
- print-type-size     discriminant: 4 bytes
- print-type-size     variant `B`: 28 bytes
- print-type-size         padding: 12 bytes
- print-type-size         field `.0`: 16 bytes, alignment: 16 bytes
- print-type-size     variant `A`: 4 bytes
- print-type-size         field `.0`: 4 bytes
8 print-type-size type: `S`: 32 bytes, alignment: 16 bytes
9 print-type-size     field `.c`: 16 bytes
10 print-type-size     field `.a`: 4 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align/repr-align.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/repr-align.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/repr-align.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr-align/auxiliary"
------------------------------------------
------------------------------------------
print-type-size type: `S`: 32 bytes, alignment: 16 bytes
print-type-size     field `.c`: 16 bytes
print-type-size     field `.a`: 4 bytes
print-type-size     field `.b`: 4 bytes
print-type-size     field `.d`: 1 bytes
print-type-size     end padding: 7 bytes
print-type-size type: `A`: 16 bytes, alignment: 16 bytes
print-type-size     field `.0`: 4 bytes
print-type-size     end padding: 12 bytes
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] ui/print_type_sizes/padding.rs stdout ----
diff of stdout:

- print-type-size type: `E1`: 12 bytes, alignment: 4 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `B`: 11 bytes
- print-type-size         padding: 3 bytes
- print-type-size         field `.0`: 8 bytes, alignment: 4 bytes
- print-type-size     variant `A`: 7 bytes
- print-type-size         field `.1`: 1 bytes
- print-type-size         padding: 2 bytes
- print-type-size         field `.0`: 4 bytes, alignment: 4 bytes
- print-type-size type: `E2`: 12 bytes, alignment: 4 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `B`: 11 bytes
- print-type-size         padding: 3 bytes
- print-type-size         field `.0`: 8 bytes, alignment: 4 bytes
- print-type-size     variant `A`: 7 bytes
- print-type-size         field `.0`: 1 bytes
- print-type-size         padding: 2 bytes
- print-type-size         field `.1`: 4 bytes, alignment: 4 bytes
- print-type-size type: `S`: 8 bytes, alignment: 4 bytes
- print-type-size     field `.g`: 4 bytes
- print-type-size     field `.a`: 1 bytes
- print-type-size     field `.b`: 1 bytes
- print-type-size     end padding: 2 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/padding.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/padding.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/padding.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/padding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/padding/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/print_type_sizes/repr_int_c.rs stdout ----
diff of stdout:

- print-type-size type: `ReprCu8`: 4 bytes, alignment: 2 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `A`: 3 bytes
- print-type-size         padding: 1 bytes
- print-type-size         field `.0`: 2 bytes, alignment: 2 bytes
- print-type-size     variant `B`: 1 bytes
- print-type-size type: `Repru8`: 4 bytes, alignment: 2 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `A`: 3 bytes
- print-type-size         padding: 1 bytes
- print-type-size         field `.0`: 2 bytes, alignment: 2 bytes
- print-type-size     variant `B`: 0 bytes


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr_int_c/repr_int_c.stdout
To only update this specific test, also pass `--test-args print_type_sizes/repr_int_c.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/repr_int_c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr_int_c" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/repr_int_c/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

327    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
328    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
329 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/private-in-public-warn.rs:50:23
-    |
- LL |     pub type Alias<T: PrivTr> = T;
-    |
-    |
-    = note: `#[warn(type_alias_bounds)]` on by default
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL |     pub type Alias<T> = T;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/private-in-public-warn.rs:75:29
-    |
- LL |     pub type Alias<T> where T: PrivTr = T;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL |     pub type Alias<T>  = T;
- 
- error: aborting due to 36 previous errors; 2 warnings emitted
+ error: aborting due to 36 previous errors
354 
---
To only update this specific test, also pass `--test-args privacy/private-in-public-warn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-in-public-warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |     pub type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:5:9
   |
   |
LL | #![deny(private_in_public)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         V1(Priv), //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         V2 { field: Priv }, //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `types::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `types::Priv` declared as private
...
LL |         type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         fn f1(arg: Priv) {} //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         fn f2() -> Priv { panic!() } //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         pub static ES: Priv; //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         pub fn ef1(arg: Priv); //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `types::Priv` in public interface (error E0446)
   |
   |
LL |         pub fn ef2() -> Priv; //~ ERROR private type `types::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `types::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `types::Priv` declared as private
...
LL |         type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |     pub type Alias<T: PrivTr> = T; //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |     pub trait Tr1: PrivTr {} //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |     pub trait Tr2<T: PrivTr> {} //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |         type Alias: PrivTr;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |         fn f<T: PrivTr>(arg: T) {} //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |     impl<T: PrivTr> Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits::PrivTr` in public interface (error E0445)
   |
   |
LL |     impl<T: PrivTr> PubTr for Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits_where::PrivTr` in public interface (error E0445)
   |
   |
LL |     pub type Alias<T> where T: PrivTr = T;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits_where::PrivTr` in public interface (error E0445)
   |
   |
LL |     pub trait Tr2<T> where T: PrivTr {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits_where::PrivTr` in public interface (error E0445)
   |
   |
LL |         fn f<T>(arg: T) where T: PrivTr {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits_where::PrivTr` in public interface (error E0445)
   |
   |
LL |     impl<T> Pub<T> where T: PrivTr {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `traits_where::PrivTr` in public interface (error E0445)
   |
   |
LL |     impl<T> PubTr for Pub<T> where T: PrivTr {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `generics::PrivTr<generics::Pub>` in public interface (error E0445)
   |
   |
LL |     pub trait Tr1: PrivTr<Pub> {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `generics::Priv` in public interface (error E0446)
   |
   |
LL |     pub trait Tr2: PubTr<Priv> {} //~ ERROR private type `generics::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `generics::Priv` in public interface (error E0446)
   |
   |
LL |     pub trait Tr3: PubTr<[Priv; 1]> {} //~ ERROR private type `generics::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `generics::Priv` in public interface (error E0446)
   |
   |
LL |     pub trait Tr4: PubTr<Pub<Priv>> {} //~ ERROR private type `generics::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `impls::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `impls::Priv` declared as private
...
LL |         type Alias = Priv; //~ ERROR private type `impls::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error: private type `aliases_pub::Priv` in public interface (error E0446)
   |
   |
LL |         pub fn f(arg: Priv) {} //~ ERROR private type `aliases_pub::Priv` in public interface
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `aliases_pub::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `aliases_pub::Priv` declared as private
...
LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error[E0446]: private type `aliases_pub::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `aliases_pub::Priv` declared as private
...
LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error[E0446]: private type `aliases_pub::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `aliases_pub::Priv` declared as private
...
LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error[E0446]: private type `aliases_pub::Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `aliases_pub::Priv` declared as private
...
LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
   |         ^^^^^^^^^^^^^^^^^^ can't leak private type

error: private trait `PrivTr1` in public interface (error E0445)
   |
   |
LL |     pub trait Tr1: PrivUseAliasTr {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private trait `PrivTr1<Priv2>` in public interface (error E0445)
   |
   |
LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error: private type `Priv2` in public interface (error E0446)
   |
   |
LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

---
---- [ui] ui/rfc-2008-non-exhaustive/improper_ctypes/extern_crate_improper.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/improper_ctypes/extern_crate_improper.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/improper_ctypes/extern_crate_improper" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/improper_ctypes/extern_crate_improper/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/rust-2018/edition-lint-infer-outlives-multispan.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-multispan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-multispan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/rust-2018/edition-lint-infer-outlives.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

11    |         ^^^^^^
12    = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`
13 
- warning: variable `theTwo` should have a snake case name
-   --> $DIR/issue-24690.rs:12:9
-    |
- LL |     let theTwo = 2;
-    |         ^^^^^^ help: convert the identifier to snake case: `the_two`
-    = note: `#[warn(non_snake_case)]` on by default
- 
- 
- warning: variable `theOtherTwo` should have a snake case name
-   --> $DIR/issue-24690.rs:13:9
-    |
- LL |     let theOtherTwo = 2;
-    |         ^^^^^^^^^^^ help: convert the identifier to snake case: `the_other_two`
- warning: 3 warnings emitted
+ warning: 1 warning emitted
29 
30 
---
To only update this specific test, also pass `--test-args span/issue-24690.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `theOtherTwo`
   |
   |
LL |     let theOtherTwo = 2; //~ WARN should have a snake case name
   |         ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_theOtherTwo`
note: the lint level is defined here
  --> /checkout/src/test/ui/span/issue-24690.rs:8:9
   |
LL | #![warn(unused)]
---
---- [ui] ui/transmute/transmute-imut-to-mut.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmute/transmute-imut-to-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-imut-to-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-imut-to-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-copy.rs stdout ----
diff of stderr:

- warning: Trait bound String: Copy does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-copy.rs:5:51
-    |
- LL | fn copy_string(t: String) -> String where String: Copy {
-    |
-    |
-    = note: `#[warn(trivial_bounds)]` on by default
- 
- warning: Trait bound String: Copy does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-copy.rs:12:56
-    |
- LL | fn copy_out_string(t: &String) -> String where String: Copy {
- 
- 
- warning: Trait bound String: Copy does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-copy.rs:16:55
-    |
- LL | fn copy_string_with_param<T>(x: String) where String: Copy {
- 
- 
- warning: Trait bound for<'b> &'b mut i32: Copy does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-copy.rs:22:76
-    |
- LL | fn copy_mut<'a>(t: &&'a mut i32) -> &'a mut i32 where for<'b> &'b mut i32: Copy {
- 
- warning: 4 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-copy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-copy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/trivial-bounds/trivial-bounds-lint.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-projection.rs stdout ----
diff of stderr:

- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:21:8
- LL |     B: A
-    |        ^
-    |
-    |
-    = note: `#[warn(trivial_bounds)]` on by default
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:28:8
-    |
- LL |     B: A<X = i32>
- 
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:35:8
-    |
- LL |     B: A<X = u8>
- 
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:42:8
-    |
- LL |     B: A<X = i32> + A
- 
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:42:21
-    |
- LL |     B: A<X = i32> + A
- 
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:51:8
-    |
- LL |     B: A<X = u8> + A
- 
- 
- warning: Trait bound B: A does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-projection.rs:51:20
-    |
- LL |     B: A<X = u8> + A
- 
- warning: 7 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent-projection.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-projection/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-projection/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs stdout ----
diff of stderr:

- warning: Trait bound Vec<str>: Debug does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-well-formed.rs:7:30
-    |
- LL | pub fn foo() where Vec<str>: Debug, str: Copy {
-    |
-    |
-    = note: `#[warn(trivial_bounds)]` on by default
- 
- warning: Trait bound str: Copy does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-well-formed.rs:7:42
-    |
- LL | pub fn foo() where Vec<str>: Debug, str: Copy {
- 
- warning: 2 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent-well-formed.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-well-formed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent-sized.rs stdout ----
diff of stderr:

- warning: Trait bound str: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-sized.rs:14:31
-    |
- LL | struct S(str, str) where str: Sized;
-    |
-    |
-    = note: `#[warn(trivial_bounds)]` on by default
- 
- warning: Trait bound for<'a> T<(dyn A + 'a)>: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-sized.rs:17:49
-    |
- LL | fn unsized_local() where for<'a> T<dyn A + 'a>: Sized {
- 
- 
- warning: Trait bound str: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent-sized.rs:22:35
-    |
- LL | fn return_str() -> str where str: Sized {
- 
- warning: 3 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent-sized.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-sized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent-sized/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs stdout ----
diff of stderr:

- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:14:19
-    |
- LL | enum E where i32: Foo { V }
-    |
-    |
-    = note: `#[warn(trivial_bounds)]` on by default
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:16:21
-    |
- LL | struct S where i32: Foo;
- 
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:18:20
-    |
- LL | trait T where i32: Foo {}
- 
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:20:20
-    |
- LL | union U where i32: Foo { f: i32 }
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/trivial-bounds-inconsistent.rs:22:14
-    |
- LL | type Y where i32: Foo = ();
-    |
-    |
-    = note: `#[warn(type_alias_bounds)]` on by default
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type Y  = ();
- 
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:22:19
-    |
- LL | type Y where i32: Foo = ();
- 
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:26:28
-    |
- LL | impl Foo for () where i32: Foo {
- 
- 
- warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:34:19
-    |
- LL | fn f() where i32: Foo {
- 
- 
- warning: Trait bound &'static str: Foo does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:41:28
-    |
- LL | fn g() where &'static str: Foo {
- 
- 
- warning: Trait bound str: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:55:37
-    |
- LL | struct TwoStrs(str, str) where str: Sized;
- 
- 
- warning: Trait bound for<'a> Dst<(dyn A + 'a)>: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:57:51
-    |
- LL | fn unsized_local() where for<'a> Dst<dyn A + 'a>: Sized {
- 
- 
- warning: Trait bound str: Sized does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:61:35
-    |
- LL | fn return_str() -> str where str: Sized {
- 
- 
- warning: Trait bound String: Neg does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:65:46
-    |
- LL | fn use_op(s: String) -> String where String: ::std::ops::Neg<Output=String> {
- 
- 
- warning: Trait bound i32: Iterator does not depend on any type or lifetime parameters
-   --> $DIR/trivial-bounds-inconsistent.rs:70:25
-    |
- LL | fn use_for() where i32: Iterator {
- 
- warning: 14 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args trivial-bounds/trivial-bounds-inconsistent.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/type/issue-67690-type-alias-bound-diagnostic-crash.rs stdout ----
diff of stderr:

- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/issue-67690-type-alias-bound-diagnostic-crash.rs:5:15
-    |
- LL | pub type T<P: Send + Send + Send> = P;
-    |               ^^^^   ^^^^   ^^^^
-    |
-    = note: `#[warn(type_alias_bounds)]` on by default
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | pub type T<P> = P;
- 
- warning: 1 warning emitted
- 
- 
---
To only update this specific test, also pass `--test-args type/issue-67690-type-alias-bound-diagnostic-crash.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/issue-67690-type-alias-bound-diagnostic-crash.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-67690-type-alias-bound-diagnostic-crash" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-67690-type-alias-bound-diagnostic-crash/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/type/type-alias-bounds.rs stdout ----
diff of stderr:

- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:8:14
-    |
- LL | type SVec<T: Send + Send> = Vec<T>;
-    |              ^^^^   ^^^^
-    |
-    = note: `#[warn(type_alias_bounds)]` on by default
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type SVec<T> = Vec<T>;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:10:21
-    |
- LL | type S2Vec<T> where T: Send = Vec<T>;
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type S2Vec<T>  = Vec<T>;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:12:19
-    |
- LL | type VVec<'b, 'a: 'b + 'b> = (&'b u32, Vec<&'a i32>);
-    |                   ^^   ^^
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type VVec<'b, 'a> = (&'b u32, Vec<&'a i32>);
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:14:18
-    |
- LL | type WVec<'b, T: 'b + 'b> = (&'b u32, Vec<T>);
-    |                  ^^   ^^
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type WVec<'b, T> = (&'b u32, Vec<T>);
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:16:25
-    |
- LL | type W2Vec<'b, T> where T: 'b, T: 'b = (&'b u32, Vec<T>);
-    |
-    |
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type W2Vec<'b, T>  = (&'b u32, Vec<T>);
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:47:12
-    |
- LL | type T1<U: Bound> = U::Assoc;
-    |
-    |
- help: use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer to associated types in type aliases
-   --> $DIR/type-alias-bounds.rs:47:21
-    |
- LL | type T1<U: Bound> = U::Assoc;
-    |                     ^^^^^^^^
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type T1<U> = U::Assoc;
- 
- 
- warning: where clauses are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:48:18
-    |
- LL | type T2<U> where U: Bound = U::Assoc;
-    |
-    |
- help: use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer to associated types in type aliases
-   --> $DIR/type-alias-bounds.rs:48:29
-    |
- LL | type T2<U> where U: Bound = U::Assoc;
-    |                             ^^^^^^^^
- help: the clause will not be checked when the type alias is used, and should be removed
-    |
- LL | type T2<U>  = U::Assoc;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:56:12
-    |
- LL | type T5<U: Bound> = <U as Bound>::Assoc;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type T5<U> = <U as Bound>::Assoc;
- 
- 
- warning: bounds on generic parameters are not enforced in type aliases
-   --> $DIR/type-alias-bounds.rs:57:12
-    |
- LL | type T6<U: Bound> = ::std::vec::Vec<U>;
-    |
-    |
- help: the bound will not be checked when the type alias is used, and should be removed
-    |
- LL | type T6<U> = ::std::vec::Vec<U>;
- 
- warning: 9 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args type/type-alias-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-alias-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-alias-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-alias-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/union/union-repr-c.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-repr-c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-repr-c" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-repr-c/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/unused/unused-closure.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused/unused-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/unused/unused-result.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused/unused-result.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-result" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-result/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/variants/variant-size-differences.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variants/variant-size-differences.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variants/variant-size-differences" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variants/variant-size-differences/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/warn-path-statement.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/warn-path-statement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/warn-path-statement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "path-statements" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/warn-path-statement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11793 passed; 114 failed; 102 ignored; 0 measured; 0 filtered out; finished in 121.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:51
