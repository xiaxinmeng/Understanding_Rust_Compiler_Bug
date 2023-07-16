plain
  --> $DIR/drop_order.rs:43:12
   |
LL |           if {
   |  ____________^
LL | |             if self.option_loud_drop(7).is_some() && self.option_loud_drop(6).is_some() {
...
LL |               }
   |  ______________^
LL | |         } {
LL | |         } {
   | |_________^
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL ~         if if self.option_loud_drop(7).is_some() && self.option_loud_drop(6).is_some() {
LL |                 self.loud_drop(8);
LL |                 false
LL ~             } {
   |

---
To only update this specific test, also pass `--test-args drop/drop_order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/drop_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop_order/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop_order/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary braces around `if` condition
   |
LL |           if {
   |  ____________^
   |  ____________^
LL | |             if self.option_loud_drop(7).is_some() && self.option_loud_drop(6).is_some() {
...
LL |               }
   |  ______________^
LL | |         } {
LL | |         } {
   | |_________^
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL ~         if if self.option_loud_drop(7).is_some() && self.option_loud_drop(6).is_some() {
LL |                 self.loud_drop(8);
LL |                 false
LL ~             } {
   |

---
normalized stderr:
warning: unnecessary braces around block return value
  --> $DIR/nullable-pointer-iotareduction.rs:68:38
   |
LL |        check_type!(main, fn(), |pthing| {
   |   ______________________________________^
LL |  |         assert_eq!(main as fn(), *pthing as fn())
   |  |________^_________________________________________^
   | |
LL | |      });
   | |______^
   |
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL -     check_type!(main, fn(), |pthing| {
LL +     check_type!(main, fn(), |pthing| assert_eq!(main as fn(), *pthing as fn()));

warning: unnecessary braces around block return value
  --> $DIR/nullable-pointer-iotareduction.rs:68:38
   |
   |
LL |        check_type!(main, fn(), |pthing| {
   |   ______________________________________^
LL |  |         assert_eq!(main as fn(), *pthing as fn())
   |  |________^_________________________________________^
   | |
LL | |      });
   | |______^
   |
   |
help: remove these braces
   |
LL -     check_type!(main, fn(), |pthing| {
LL +     check_type!(main, fn(), |pthing| assert_eq!(main as fn(), *pthing as fn()));

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args nullable-pointer-iotareduction.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nullable-pointer-iotareduction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nullable-pointer-iotareduction/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nullable-pointer-iotareduction/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary braces around block return value
   |
   |
LL |        check_type!(main, fn(), |pthing| {
   |   ______________________________________^
LL |  |         assert_eq!(main as fn(), *pthing as fn())
   |  |________^_________________________________________^
   | |
LL | |      });
   | |______^
   |
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL -     check_type!(main, fn(), |pthing| {
LL +     check_type!(main, fn(), |pthing| assert_eq!(main as fn(), *pthing as fn()));

warning: unnecessary braces around block return value
  --> /checkout/src/test/ui/nullable-pointer-iotareduction.rs:68:38
   |
   |
LL |        check_type!(main, fn(), |pthing| {
   |   ______________________________________^
LL |  |         assert_eq!(main as fn(), *pthing as fn())
   |  |________^_________________________________________^
   | |
LL | |      });
   | |______^
   |
   |
help: remove these braces
   |
LL -     check_type!(main, fn(), |pthing| {
LL +     check_type!(main, fn(), |pthing| assert_eq!(main as fn(), *pthing as fn()));

warning: 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui/statics/static-promotion.rs stdout ----
normalized stderr:
warning: unnecessary braces around assigned value
  --> $DIR/static-promotion.rs:16:27
   |
LL |   static C: A<B<B<[u8]>>> = {
   |  ___________________________^
LL | |     A(&B {
   | |____^
LL |           x: &B { x: STR },
   |  _______^
LL | | };
   | |_^
   |
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL ~ static C: A<B<B<[u8]>>> = A(&B {
LL |         x: &B { x: STR },
LL ~     });

warning: 1 warning emitted


---
To only update this specific test, also pass `--test-args statics/static-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/statics/static-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/static-promotion/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/static-promotion/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary braces around assigned value
   |
   |
LL |   static C: A<B<B<[u8]>>> = {
   |  ___________________________^
LL | |     A(&B {
   | |____^
LL |           x: &B { x: STR },
   |  _______^
LL | | };
   | |_^
   |
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
LL ~ static C: A<B<B<[u8]>>> = A(&B {
LL |         x: &B { x: STR },
LL ~     });

warning: 1 warning emitted
------------------------------------------

