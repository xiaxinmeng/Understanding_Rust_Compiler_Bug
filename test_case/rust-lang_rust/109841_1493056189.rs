plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................  3344/14753
.......................................i................................................  3432/14753
......i.................................................................................  3520/14753
........................................................................................  3608/14753
..........................F..........................................................iii  3696/14753
........................................................................................  3872/14753
........................................................................................  3960/14753
........................................................................................  4048/14753
........................................................................................  4136/14753
---

---- [ui] tests/ui/diagnostic-flags/terminal_urls.rs stdout ----
diff of stderr:

1 error[ttps://doc.rust-lang.org/error_codes/E0308.htmlE0308ismatched types
-   --> $DIR/terminal_urls.rs:3:9
+   --> ile://$DIR/terminal_urls.rs$DIR/terminal_urls.rs]8;;:3:9
4 LL |     let () = 4;
5    |         ^^   - this expression has type `{integer}`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diagnostic-flags/terminal_urls/terminal_urls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args diagnostic-flags/terminal_urls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/diagnostic-flags/terminal_urls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diagnostic-flags/terminal_urls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diagnostic-flags/terminal_urls/auxiliary" "-Zterminal-urls=yes"
stdout: none
--- stderr -------------------------------
error[ttps://doc.rust-lang.org/error_codes/E0308.htmlE0308ismatched types
  --> ile:///checkout/tests/ui/diagnostic-flags/terminal_urls.rsfake-test-src-base/diagnostic-flags/terminal_urls.rs]8;;:3:9
   |
LL |     let () = 4; //~ ERROR
   |         ^^   - this expression has type `{integer}`
   |         expected integer, found `()`

error: aborting due to previous error

