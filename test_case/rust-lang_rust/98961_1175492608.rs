plain
........................................................................................ 11792/13151
........................................................................................ 11880/13151
........................................................................................ 11968/13151
........................................................................................ 12056/13151
.....................................................................F....F.....F....... 12144/13151
........................................................................i............... 12320/13151
........................................................................................ 12408/13151
........................................................................................ 12496/13151
........................................................................................ 12584/13151
---

---- [ui] src/test/ui/traits/vtable/vtable-diamond.rs stdout ----
diff of stderr:

13 LL | trait D: B + C {
15 
15 
- error: vtable entries for `<S as C>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as A>::foo_a),
-            Method(<S as C>::foo_c),
-   --> $DIR/vtable-diamond.rs:15:1
-    |
-    |
- LL | trait C: A {
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
29 
---
To only update this specific test, also pass `--test-args traits/vtable/vtable-diamond.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-diamond.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-diamond" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-diamond/auxiliary"
stdout: none
--- stderr -------------------------------
error: vtable entries for `<S as D>`: [
           MetadataDropInPlace,
           MetadataAlign,
           MetadataAlign,
           Method(<S as A>::foo_a),
           Method(<S as B>::foo_b),
           Method(<S as C>::foo_c),
           TraitVPtr(<S as C>),
           Method(<S as D>::foo_d),
  --> /checkout/src/test/ui/traits/vtable/vtable-diamond.rs:21:1
   |
   |
LL | trait D: B + C {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/vtable/vtable-multiple.rs stdout ----
diff of stderr:

12 LL | trait C: A + B {
14 
14 
- error: vtable entries for `<S as B>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as B>::foo_b),
-   --> $DIR/vtable-multiple.rs:10:1
-    |
-    |
- LL | trait B {
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
27 
---
To only update this specific test, also pass `--test-args traits/vtable/vtable-multiple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multiple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multiple/auxiliary"
stdout: none
--- stderr -------------------------------
error: vtable entries for `<S as C>`: [
           MetadataDropInPlace,
           MetadataAlign,
           MetadataAlign,
           Method(<S as A>::foo_a),
           Method(<S as B>::foo_b),
           TraitVPtr(<S as B>),
           Method(<S as C>::foo_c),
  --> /checkout/src/test/ui/traits/vtable/vtable-multiple.rs:16:1
   |
   |
LL | trait C: A + B {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/vtable/vtable-multi-level.rs stdout ----
diff of stderr:

34 LL | trait O: G + N {
36 
36 
- error: vtable entries for `<S as B>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as B>::foo_b),
-   --> $DIR/vtable-multi-level.rs:19:1
-    |
-    |
- LL | trait B {
- 
- 
- error: vtable entries for `<S as D>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as D>::foo_d),
-   --> $DIR/vtable-multi-level.rs:30:1
-    |
-    |
- LL | trait D {
- 
- 
- error: vtable entries for `<S as E>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as E>::foo_e),
-   --> $DIR/vtable-multi-level.rs:36:1
-    |
-    |
- LL | trait E {
- 
- 
- error: vtable entries for `<S as F>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as D>::foo_d),
-            Method(<S as E>::foo_e),
-            TraitVPtr(<S as E>),
-            Method(<S as F>::foo_f),
-   --> $DIR/vtable-multi-level.rs:42:1
-    |
-    |
- LL | trait F: D + E {
- 
- 
- error: vtable entries for `<S as H>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as H>::foo_h),
-   --> $DIR/vtable-multi-level.rs:53:1
-    |
-    |
- LL | trait H {
- 
- 
- error: vtable entries for `<S as I>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as I>::foo_i),
-   --> $DIR/vtable-multi-level.rs:59:1
-    |
-    |
- LL | trait I {
- 
- 
- error: vtable entries for `<S as J>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as H>::foo_h),
-            Method(<S as I>::foo_i),
-            TraitVPtr(<S as I>),
-            Method(<S as J>::foo_j),
-   --> $DIR/vtable-multi-level.rs:65:1
-    |
-    |
- LL | trait J: H + I {
- 
- 
- error: vtable entries for `<S as K>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as K>::foo_k),
-   --> $DIR/vtable-multi-level.rs:71:1
-    |
-    |
- LL | trait K {
- 
- 
- error: vtable entries for `<S as L>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as L>::foo_l),
-   --> $DIR/vtable-multi-level.rs:77:1
-    |
-    |
- LL | trait L {
- 
- 
- error: vtable entries for `<S as M>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as K>::foo_k),
-            Method(<S as L>::foo_l),
-            TraitVPtr(<S as L>),
-            Method(<S as M>::foo_m),
-   --> $DIR/vtable-multi-level.rs:83:1
-    |
-    |
- LL | trait M: K + L {
- 
- 
- error: vtable entries for `<S as N>`: [
-            MetadataDropInPlace,
-            MetadataAlign,
-            MetadataAlign,
-            Method(<S as H>::foo_h),
-            Method(<S as I>::foo_i),
-            TraitVPtr(<S as I>),
-            Method(<S as J>::foo_j),
-            Method(<S as K>::foo_k),
-            TraitVPtr(<S as K>),
-            Method(<S as L>::foo_l),
-            TraitVPtr(<S as L>),
-            Method(<S as M>::foo_m),
-            TraitVPtr(<S as M>),
-            Method(<S as N>::foo_n),
-   --> $DIR/vtable-multi-level.rs:89:1
-    |
-    |
- LL | trait N: J + M {
- 
- error: aborting due to 12 previous errors
+ error: aborting due to previous error
178 
178 
179 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level/vtable-multi-level.stderr
To only update this specific test, also pass `--test-args traits/vtable/vtable-multi-level.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-multi-level.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level/auxiliary"
stdout: none
--- stderr -------------------------------
error: vtable entries for `<S as O>`: [
           MetadataDropInPlace,
           MetadataAlign,
           MetadataAlign,
           Method(<S as A>::foo_a),
           Method(<S as B>::foo_b),
           TraitVPtr(<S as B>),
           Method(<S as C>::foo_c),
           Method(<S as D>::foo_d),
           TraitVPtr(<S as D>),
           Method(<S as E>::foo_e),
           TraitVPtr(<S as E>),
           Method(<S as F>::foo_f),
           TraitVPtr(<S as F>),
           Method(<S as G>::foo_g),
           Method(<S as H>::foo_h),
           TraitVPtr(<S as H>),
           Method(<S as I>::foo_i),
           TraitVPtr(<S as I>),
           Method(<S as J>::foo_j),
           TraitVPtr(<S as J>),
           Method(<S as K>::foo_k),
           TraitVPtr(<S as K>),
           Method(<S as L>::foo_l),
           TraitVPtr(<S as L>),
           Method(<S as M>::foo_m),
           TraitVPtr(<S as M>),
           Method(<S as N>::foo_n),
           TraitVPtr(<S as N>),
           Method(<S as O>::foo_o),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:95:1
   |
   |
LL | trait O: G + N {

error: aborting due to previous error
------------------------------------------

