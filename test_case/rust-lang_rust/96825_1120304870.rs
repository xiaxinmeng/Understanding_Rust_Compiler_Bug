plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck  driver.rs -o ""/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck"/driver"
--- stderr -------------------------------
error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> driver.rs:71:54
   |
   |
71 |                 if matches!(tcx.def_kind(id.def_id), DefKind::Fn) {
   |                                                      ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> driver.rs:77:54
   |
   |
77 |                 if matches!(tcx.def_kind(id.def_id), DefKind::AssocFn) {
   |                                                      ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> driver.rs:88:54
   |
   |
88 |                 if matches!(tcx.def_kind(id.def_id), DefKind::AssocFn) {
   |                                                      ^^^^^^^ use of undeclared type `DefKind`
error[E0425]: cannot find value `visitor` in this scope
  --> driver.rs:94:27
   |
   |
94 |             for def_id in visitor.bodies {

warning: ignoring --out-dir flag due to -o flag

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> driver.rs:78:47
    |
78  |                     let trait_item = hir.item(id);
    |                                          ---- ^^ expected struct `ItemId`, found struct `TraitItemId`
    |                                          arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:383:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:383:12
    |
383 |     pub fn item(self, id: ItemId) -> &'hir Item<'hir> {

error[E0308]: mismatched types
  --> driver.rs:79:28
   |
   |
79 |                     if let rustc_hir::TraitItemKind::Fn(_, trait_fn) = &trait_item.kind {
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   ---------------- this expression has type `&ItemKind<'_>`
   |                            |
   |                            expected enum `ItemKind`, found enum `TraitItemKind`
error: aborting due to 6 previous errors; 1 warning emitted

Some errors have detailed explanations: E0308, E0425, E0433.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
make: *** [Makefile:19: all] Error 1



failures:
