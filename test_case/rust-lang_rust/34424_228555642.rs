
--- /Users/rustbuild/src/rust-buildbot/slave/try-mac/build/obj/x86_64-apple-darwin/test/run-make/pretty-expanded-hygiene.stage2-x86_64-apple-darwin/input.out.replaced  2016-06-25 11:38:14.000000000 -0400
+++ /Users/rustbuild/src/rust-buildbot/slave/try-mac/build/obj/x86_64-apple-darwin/test/run-make/pretty-expanded-hygiene.stage2-x86_64-apple-darwin/input.pp.rs 2016-06-25 11:38:14.000000000 -0400
@@ -13,6 +13,6 @@
 #![no_core]


-fn bar /* Sat Jun 25 11:38:14 EDT 2016 */() { let x /* Sat Jun 25 11:38:14 EDT 2016 */ = 1 y /* Sat Jun 25 11:38:14 EDT 2016 */ + x /* Sat Jun 25 11:38:14 EDT 2016 */ }
+fn bar /* Sat Jun 25 11:38:14 EDT 2016 */() { let x /* Sat Jun 25 11:38:14 EDT 2016 */ = 1; y /* Sat Jun 25 11:38:14 EDT 2016 */ + x /* Sat Jun 25 11:38:14 EDT 2016 */ }

 fn y /* Sat Jun 25 11:38:14 EDT 2016 */() { }

------------------------------------------
stderr:
