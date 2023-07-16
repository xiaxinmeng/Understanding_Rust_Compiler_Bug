plain
........................................................................................ 11616/13442
........................................................................................ 11704/13442
........................................................................................ 11792/13442
...........................................................i.......i........i.....i..... 11880/13442
..F.FF...F......i....................................................................... 11968/13442
........................................................................................ 12144/13442
........................................................................................ 12232/13442
........................................................................................ 12320/13442
........................................................................................ 12408/13442
---

---- [ui] src/test/ui/target-feature/tied-features-cli.rs#three stdout ----
diff of stderr:

- error: target features paca, pacg must all be enabled or disabled together
+ error: the target features paca, pacg must all be enabled or disabled together
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.three/tied-features-cli.three.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args target-feature/tied-features-cli.rs`


error in revision `three`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/tied-features-cli.rs" "-Zthreads=1" "--cfg" "three" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.three" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "--target=aarch64-unknown-linux-gnu" "-C" "target-feature=+paca,+pacg,-paca" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.three/auxiliary"
stdout: none
--- stderr -------------------------------
error: the target features paca, pacg must all be enabled or disabled together
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/target-feature/tied-features-cli.rs#two stdout ----
diff of stderr:

- error: target features paca, pacg must all be enabled or disabled together
+ error: the target features paca, pacg must all be enabled or disabled together
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.two/tied-features-cli.two.stderr
To only update this specific test, also pass `--test-args target-feature/tied-features-cli.rs`


error in revision `two`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/tied-features-cli.rs" "-Zthreads=1" "--cfg" "two" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.two" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "--target=aarch64-unknown-linux-gnu" "-C" "target-feature=-pacg,+pacg" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.two/auxiliary"
stdout: none
--- stderr -------------------------------
error: the target features paca, pacg must all be enabled or disabled together
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/target-feature/tied-features-cli.rs#one stdout ----
diff of stderr:

- error: target features paca, pacg must all be enabled or disabled together
+ error: the target features paca, pacg must all be enabled or disabled together
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.one/tied-features-cli.one.stderr
To only update this specific test, also pass `--test-args target-feature/tied-features-cli.rs`


error in revision `one`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/tied-features-cli.rs" "-Zthreads=1" "--cfg" "one" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.one" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "--target=aarch64-unknown-linux-gnu" "-C" "target-feature=+paca" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features-cli.one/auxiliary"
stdout: none
--- stderr -------------------------------
error: the target features paca, pacg must all be enabled or disabled together
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/target-feature/tied-features.rs stdout ----
diff of stderr:

- error: the target features paca, pacg must all be either enabled or disabled together
+ error: the target features paca, pacg must all be enabled or disabled together
2   --> $DIR/tied-features.rs:12:5
3    |
4 LL |     #[target_feature(enable = "pacg")]
6    |
7    = help: add the missing features in a `target_feature` attribute
8 
8 
- error: the target features paca, pacg must all be either enabled or disabled together
+ error: the target features paca, pacg must all be enabled or disabled together
10   --> $DIR/tied-features.rs:24:1
11    |
12 LL | #[target_feature(enable = "paca")]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features/tied-features.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args target-feature/tied-features.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/tied-features.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "--target=aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features/auxiliary"
stdout: none
--- stderr -------------------------------
error: the target features paca, pacg must all be enabled or disabled together
   |
   |
LL |     #[target_feature(enable = "pacg")]
   |
   = help: add the missing features in a `target_feature` attribute


error: the target features paca, pacg must all be enabled or disabled together
   |
   |
LL | #[target_feature(enable = "paca")]
   |
   = help: add the missing features in a `target_feature` attribute

error: aborting due to 2 previous errors
