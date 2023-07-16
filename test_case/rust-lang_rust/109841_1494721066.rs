plain
........................................................................................  3344/14753
......................................i.................................................  3432/14753
.....i..................................................................................  3520/14753
........................................................................................  3608/14753
........................F...........................................................iiii  3696/14753
........................................................................................  3872/14753
........................................................................................  3960/14753
........................................................................................  4048/14753
........................................................................................  4136/14753
---

---- [ui] tests/ui/diagnostic-flags/terminal_urls.rs stdout ----
diff of stderr:

1 error[ttps://doc.rust-lang.org/error_codes/E0308.htmlE0308ismatched types
-   --> ile://$DIR/terminal_urls.rs$DIR/terminal_urls.rs]8;;:3:9
+   --> ile://5f70d2ad0bb5$DIR/terminal_urls.rs$DIR/terminal_urls.rs]8;;:3:9
-    |     let () = 4;
+ LL |     let () = 4;
5    |         ^^   - this expression has type `{integer}`
6    |         |
---
To only update this specific test, also pass `--test-args diagnostic-flags/terminal_urls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/diagnostic-flags/terminal_urls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diagnostic-flags/terminal_urls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diagnostic-flags/terminal_urls/auxiliary" "-Zterminal-urls=yes"
stdout: none
--- stderr -------------------------------
error[ttps://doc.rust-lang.org/error_codes/E0308.htmlE0308ismatched types
  --> ile://5f70d2ad0bb5/checkout/tests/ui/diagnostic-flags/terminal_urls.rsfake-test-src-base/diagnostic-flags/terminal_urls.rs]8;;:3:9
   |
LL |     let () = 4; //~ ERROR
   |         ^^   - this expression has type `{integer}`
   |         expected integer, found `()`

error: aborting due to previous error

