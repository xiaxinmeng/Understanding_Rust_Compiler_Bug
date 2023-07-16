plain
...........................iiiii...i.....ii............................................. 10912/13721
........................................................................................ 11000/13721
...............i........................................................................ 11088/13721
.........................iiiiii.i..iiiiiiiii.i.......................................... 11176/13721
..F...FFF...F...FFFFFFFF.F.............................F.F.............................. 11264/13721
........................................................................................ 11440/13721
........................................................................................ 11528/13721
........................................................................................ 11616/13721
........................................................................................ 11704/13721
---
---- [ui] src/test/ui/self/elision/lt-alias.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
   |                         ---------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Alias(self: Alias<'a>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias.rs:20:57
   |
   |
LL |     fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
   |                             --------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias.rs:24:66
   |
   |
LL |     fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
   |                                 -------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias.rs:28:55
   |
   |
LL |     fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
   |                            -------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias.rs:32:64
   |
   |
LL |     fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
   |                                ------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &'a u32 {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0106`.
For more information about this error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-alias-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-alias-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias-async/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
   |                               ---------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Alias(self: Alias<'a>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:21:63
   |
   |
LL |     async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
   |                                   --------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:25:72
   |
   |
LL |     async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
   |                                       -------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:29:61
   |
   |
LL |     async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
   |                                  -------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:33:70
   |
   |
LL |     async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
   |                                      ------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &'a u32 {

error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:18:9
   |
   |
LL |     async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
   |                                             ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:22:9
   |
   |
LL |     async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
   |                                                      ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:26:9
   |
   |
LL |     async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
   |                                                               ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:30:9
   |
   |
LL |     async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
   |                                                    ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-alias-async.rs:34:9
   |
   |
LL |     async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
   |                                                             ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0106, E0621.
For more information about an error, try `rustc --explain E0106`.
For more information about an error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-assoc-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-assoc-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc-async/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
   |                                   --------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:27:90
   |
   |
LL |     async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                       -------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:34:10
   |
   |
LL |         self: Box<Box<<Struct<'a> as Trait>::AssocType>>,
LL |         f: &u32
   |            ----
LL |     ) -> &u32 {
   |          ^ expected named lifetime parameter
   |          ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
LL |     ) -> &'a u32 {
   |           ++

error[E0106]: missing lifetime specifier
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:38:88
   |
LL |     async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                      ------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:45:10
   |
   |
LL |         self: Box<Rc<<Struct<'a> as Trait>::AssocType>>,
LL |         f: &u32
   |            ----
LL |     ) -> &u32 {
   |          ^ expected named lifetime parameter
   |          ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
LL |     ) -> &'a u32 {
   |           ++

error[E0621]: explicit lifetime required in the type of `f`
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:24:9
   |
LL |     async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
   |                                                                        ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:28:9
   |
   |
LL |     async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                                                                 ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:35:9
   |
LL |         f: &u32
LL |         f: &u32
   |            ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |     ) -> &u32 {
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:39:9
   |
   |
LL |     async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                                                               ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |         f
   |         ^ lifetime `'a` required
error[E0621]: explicit lifetime required in the type of `f`
  --> /checkout/src/test/ui/self/elision/lt-assoc-async.rs:46:9
   |
LL |         f: &u32
LL |         f: &u32
   |            ---- help: add explicit lifetime `'a` to the type of `f`: `&'a u32`
LL |     ) -> &u32 {
LL |         f
   |         ^ lifetime `'a` required
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0106, E0621.
For more information about an error, try `rustc --explain E0106`.
For more information about an error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-assoc.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-assoc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
   |                             --------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc.rs:26:84
   |
   |
LL |     fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                 -------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc.rs:30:93
   |
   |
LL |     fn take_Box_Box_AssocType(self: Box<Box<<Struct<'a> as Trait>::AssocType>>, f: &u32) -> &u32 {
   |                                     ------------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Box_AssocType(self: Box<Box<<Struct<'a> as Trait>::AssocType>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc.rs:34:82
   |
   |
LL |     fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
   |                                ------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-assoc.rs:38:91
   |
   |
LL |     fn take_Box_Rc_AssocType(self: Box<Rc<<Struct<'a> as Trait>::AssocType>>, f: &u32) -> &u32 {
   |                                    -----------------------------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Rc_AssocType(self: Box<Rc<<Struct<'a> as Trait>::AssocType>>, f: &u32) -> &'a u32 {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0106`.
For more information about this error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-struct.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     fn take_Struct(self: Struct<'a>, f: &u32) -> &u32 {
   |                          ----------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Struct(self: Struct<'a>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-struct.rs:18:59
   |
   |
LL |     fn take_Box_Struct(self: Box<Struct<'a>>, f: &u32) -> &u32 {
   |                              ---------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Struct(self: Box<Struct<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-struct.rs:22:68
   |
   |
LL |     fn take_Box_Box_Struct(self: Box<Box<Struct<'a>>>, f: &u32) -> &u32 {
   |                                  --------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Box_Struct(self: Box<Box<Struct<'a>>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-struct.rs:26:57
   |
   |
LL |     fn take_Rc_Struct(self: Rc<Struct<'a>>, f: &u32) -> &u32 {
   |                             --------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Rc_Struct(self: Rc<Struct<'a>>, f: &u32) -> &'a u32 {

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/lt-struct.rs:30:66
   |
   |
LL |     fn take_Box_Rc_Struct(self: Box<Rc<Struct<'a>>>, f: &u32) -> &u32 {
   |                                 -------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider using the `'a` lifetime
   |
LL |     fn take_Box_Rc_Struct(self: Box<Rc<Struct<'a>>>, f: &u32) -> &'a u32 {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0106`.
For more information about this error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/self/elision/multiple-ref-self.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/multiple-ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/multiple-ref-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/multiple-ref-self/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL |     fn wrap_ref_Self_ref_Self(self: Wrap<&Self, &Self>, f: &u8) -> &u8 {
   |                                     ------------------     ---     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from one of `self`'s 2 lifetimes or `f`
help: consider introducing a named lifetime parameter
   |
LL |     fn wrap_ref_Self_ref_Self<'a>(self: Wrap<&'a Self, &'a Self>, f: &'a u8) -> &'a u8 {
   |                              ++++             ++        ++            ++         ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/multiple-ref-self.rs:26:78
   |
   |
LL |     fn box_wrap_ref_Self_ref_Self(self: Box<Wrap<&Self, &Self>>, f: &u32) -> &u32 {
   |                                         -----------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from one of `self`'s 2 lifetimes or `f`
help: consider introducing a named lifetime parameter
   |
LL |     fn box_wrap_ref_Self_ref_Self<'a>(self: Box<Wrap<&'a Self, &'a Self>>, f: &'a u32) -> &'a u32 {
   |                                  ++++                 ++        ++             ++          ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/self/elision/multiple-ref-self.rs:30:78
   |
   |
LL |     fn pin_wrap_ref_Self_ref_Self(self: Pin<Wrap<&Self, &Self>>, f: &u32) -> &u32 {
   |                                         -----------------------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from one of `self`'s 2 lifetimes or `f`
help: consider introducing a named lifetime parameter
   |
LL |     fn pin_wrap_ref_Self_ref_Self<'a>(self: Pin<Wrap<&'a Self, &'a Self>>, f: &'a u32) -> &'a u32 {
   |                                  ++++                 ++        ++             ++          ++
error[E0106]: missing lifetime specifier
---
To only update this specific test, also pass `--test-args traits/issue-102989.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-102989.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-102989/auxiliary"
stdout: none
--- stderr -------------------------------
error: `self` parameter is only allowed in associated functions
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |               ^^^^ not semantically valid as function parameter
   |
   = note: associated functions are those in `impl` or `trait` definitions
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:42
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |                     -------     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
help: consider introducing a named lifetime parameter
   |
LL | fn ref_Struct<'a>(self: &'a Struct, f: &'a u32) -> &'a u32 {
   |              ++++        ++             ++          ++
error[E0412]: cannot find type `Struct` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:22
   |
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/traits/issue-102989.rs:11:13
   |
   |
LL |     let x = x << 1;
   |             ^ help: a local variable with a similar name exists: `f`

error[E0152]: found duplicate lang item `sized`
   |
   |
LL | trait Sized { } //~ ERROR found duplicate lang item `sized`
   |
   |
   = note: the lang item is first defined in crate `core` (which `std` depends on)
   = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fa992565d2130c71.rlib
   = note: second definition in the local crate (`issue_102989`)
error[E0277]: the size for values of type `{integer}` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/issue-102989.rs:11:15
   |
LL |     let x = x << 1;
LL |     let x = x << 1;
   |               ^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `{integer}`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/traits/issue-102989.rs:7:42
   |
LL | fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |    ----------                            ^^^^ expected `&u32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
help: consider returning the local binding `x`
   |
   |
LL ~     let x = x << 1;
LL +     x

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0106, E0152, E0277, E0308, E0412, E0425.
