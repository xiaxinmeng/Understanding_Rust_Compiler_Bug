plain
2019-08-15T22:30:34.2579323Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T22:30:34.2579394Z 
2019-08-15T22:30:34.2579680Z   git checkout -b <new-branch-name>
2019-08-15T22:30:34.2579725Z 
2019-08-15T22:30:34.2580028Z HEAD is now at 91c543e4b Auto merge of #63617 - Centril:rollup-zqsuhyy, r=Centril
2019-08-15T22:30:34.2738178Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T22:30:34.2741448Z ==============================================================================
2019-08-15T22:30:34.2741532Z Task         : Bash
2019-08-15T22:30:34.2741617Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T00:34:35.4085416Z 1 error: lifetime may not live long enough
2019-08-16T00:34:35.4085694Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:35:12
2019-08-16T00:34:35.4085986Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-16T00:34:35.4086062Z 3    |
2019-08-16T00:34:35.4086300Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4087976Z 5    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4088396Z 10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4088484Z 11 
2019-08-16T00:34:35.4088863Z 12 error: lifetime may not live long enough
2019-08-16T00:34:35.4089328Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-16T00:34:35.4089328Z -   --> $DIR/regions-outlives-projection-container-hrtb.rs:57:12
2019-08-16T00:34:35.4089621Z +   --> $DIR/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-16T00:34:35.4089883Z 14    |
2019-08-16T00:34:35.4090189Z 15 LL | fn with_assoc_sub<'a,'b>() {
2019-08-16T00:34:35.4090468Z 16    |                   -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4090556Z 
2019-08-16T00:34:35.4090978Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4090978Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4091439Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate.nll/regions-outlives-projection-container-hrtb.migrate.nll.stderr
2019-08-16T00:34:35.4091756Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T00:34:35.4092270Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`
2019-08-16T00:34:35.4092620Z 
2019-08-16T00:34:35.4092754Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-16T00:34:35.4092825Z status: exit code: 1
2019-08-16T00:34:35.4093951Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb.migrate.nll/auxiliary" "-A" "unused"
2019-08-16T00:34:35.4095231Z ------------------------------------------
2019-08-16T00:34:35.4095413Z 
2019-08-16T00:34:35.4095726Z ------------------------------------------
2019-08-16T00:34:35.4095947Z stderr:
2019-08-16T00:34:35.4095947Z stderr:
2019-08-16T00:34:35.4096279Z ------------------------------------------
2019-08-16T00:34:35.4096356Z error: lifetime may not live long enough
2019-08-16T00:34:35.4097655Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:30:12
2019-08-16T00:34:35.4097915Z    |
2019-08-16T00:34:35.4098260Z LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4098506Z    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4099020Z    |               lifetime `'a` defined here
2019-08-16T00:34:35.4099248Z ...
2019-08-16T00:34:35.4099248Z ...
2019-08-16T00:34:35.4099584Z LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
2019-08-16T00:34:35.4099889Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4100217Z error: lifetime may not live long enough
2019-08-16T00:34:35.4100747Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:50:12
2019-08-16T00:34:35.4100826Z    |
2019-08-16T00:34:35.4100826Z    |
2019-08-16T00:34:35.4101407Z LL | fn with_assoc_sub<'a,'b>() {
2019-08-16T00:34:35.4101816Z    |                   -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4102332Z    |                   lifetime `'a` defined here
2019-08-16T00:34:35.4102421Z ...
2019-08-16T00:34:35.4102421Z ...
2019-08-16T00:34:35.4102651Z LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
2019-08-16T00:34:35.4102950Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4103252Z error: aborting due to 2 previous errors
2019-08-16T00:34:35.4103293Z 
2019-08-16T00:34:35.4103325Z 
2019-08-16T00:34:35.4103700Z ------------------------------------------
---
2019-08-16T00:34:35.4104523Z 1 error: lifetime may not live long enough
2019-08-16T00:34:35.4104954Z -   --> $DIR/regions-outlives-projection-container-wc.rs:37:12
2019-08-16T00:34:35.4105388Z +   --> $DIR/regions-outlives-projection-container-wc.rs:33:12
2019-08-16T00:34:35.4105582Z 3    |
2019-08-16T00:34:35.4105872Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4106332Z 5    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4106439Z 
2019-08-16T00:34:35.4106644Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4106644Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4107411Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate.nll/regions-outlives-projection-container-wc.migrate.nll.stderr
2019-08-16T00:34:35.4107802Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T00:34:35.4108159Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`
2019-08-16T00:34:35.4108261Z 
2019-08-16T00:34:35.4108529Z error in revision `migrate`: 1 errors occurred comparing output.
2019-08-16T00:34:35.4108665Z status: exit code: 1
2019-08-16T00:34:35.4109966Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc.migrate.nll/auxiliary" "-A" "unused"
2019-08-16T00:34:35.4110903Z ------------------------------------------
2019-08-16T00:34:35.4111099Z 
2019-08-16T00:34:35.4111426Z ------------------------------------------
2019-08-16T00:34:35.4111497Z stderr:
2019-08-16T00:34:35.4111497Z stderr:
2019-08-16T00:34:35.4111908Z ------------------------------------------
2019-08-16T00:34:35.4112129Z error: lifetime may not live long enough
2019-08-16T00:34:35.4112478Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs:33:12
2019-08-16T00:34:35.4112584Z    |
2019-08-16T00:34:35.4112800Z LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4113231Z    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4113752Z    |               lifetime `'a` defined here
2019-08-16T00:34:35.4113821Z ...
2019-08-16T00:34:35.4113821Z ...
2019-08-16T00:34:35.4114085Z LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-16T00:34:35.4114370Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4114713Z error: aborting due to previous error
2019-08-16T00:34:35.4114892Z 
2019-08-16T00:34:35.4114926Z 
2019-08-16T00:34:35.4115378Z ------------------------------------------
---
2019-08-16T00:34:35.4116302Z 1 error: lifetime may not live long enough
2019-08-16T00:34:35.4116611Z -   --> $DIR/regions-outlives-projection-container.rs:40:13
2019-08-16T00:34:35.4116863Z +   --> $DIR/regions-outlives-projection-container.rs:36:13
2019-08-16T00:34:35.4116952Z 3    |
2019-08-16T00:34:35.4117821Z 4 LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4118301Z 5    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4118918Z 10    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4119008Z 11 
2019-08-16T00:34:35.4119102Z 12 error: lifetime may not live long enough
2019-08-16T00:34:35.4119363Z -   --> $DIR/regions-outlives-projection-container.rs:58:13
2019-08-16T00:34:35.4119363Z -   --> $DIR/regions-outlives-projection-container.rs:58:13
2019-08-16T00:34:35.4119834Z +   --> $DIR/regions-outlives-projection-container.rs:54:13
2019-08-16T00:34:35.4120040Z 14    |
2019-08-16T00:34:35.4120349Z 15 LL | fn without_assoc<'a,'b>() {
2019-08-16T00:34:35.4120986Z 16    |                  -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4121505Z 21    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4121609Z 22 
2019-08-16T00:34:35.4121687Z 23 error: lifetime may not live long enough
2019-08-16T00:34:35.4122108Z -   --> $DIR/regions-outlives-projection-container.rs:67:5
2019-08-16T00:34:35.4122108Z -   --> $DIR/regions-outlives-projection-container.rs:67:5
2019-08-16T00:34:35.4122537Z +   --> $DIR/regions-outlives-projection-container.rs:63:5
2019-08-16T00:34:35.4123498Z 25    |
2019-08-16T00:34:35.4123888Z 26 LL | fn call_with_assoc<'a,'b>() {
2019-08-16T00:34:35.4124309Z 27    |                    -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4124862Z 32    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4124961Z 33 
2019-08-16T00:34:35.4125019Z 34 error: lifetime may not live long enough
2019-08-16T00:34:35.4125442Z -   --> $DIR/regions-outlives-projection-container.rs:74:5
2019-08-16T00:34:35.4125442Z -   --> $DIR/regions-outlives-projection-container.rs:74:5
2019-08-16T00:34:35.4125848Z +   --> $DIR/regions-outlives-projection-container.rs:70:5
2019-08-16T00:34:35.4126062Z 36    |
2019-08-16T00:34:35.4126359Z 37 LL | fn call_without_assoc<'a,'b>() {
2019-08-16T00:34:35.4126926Z 38    |                       -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4127840Z 
2019-08-16T00:34:35.4128062Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4128062Z The actual stderr differed from the expected stderr.
2019-08-16T00:34:35.4128742Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container.nll/regions-outlives-projection-container.nll.stderr
2019-08-16T00:34:35.4129276Z To update references, rerun the tests and pass the `--bless` flag
2019-08-16T00:34:35.4129778Z To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`
2019-08-16T00:34:35.4130135Z error: 1 errors occurred comparing output.
2019-08-16T00:34:35.4130227Z status: exit code: 1
2019-08-16T00:34:35.4130227Z status: exit code: 1
2019-08-16T00:34:35.4131651Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container.nll/auxiliary" "-A" "unused"
2019-08-16T00:34:35.4132619Z ------------------------------------------
2019-08-16T00:34:35.4132796Z 
2019-08-16T00:34:35.4133099Z ------------------------------------------
2019-08-16T00:34:35.4133299Z stderr:
2019-08-16T00:34:35.4133299Z stderr:
2019-08-16T00:34:35.4133618Z ------------------------------------------
2019-08-16T00:34:35.4136068Z error: lifetime may not live long enough
2019-08-16T00:34:35.4137966Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:36:13
2019-08-16T00:34:35.4138287Z    |
2019-08-16T00:34:35.4138559Z LL | fn with_assoc<'a,'b>() {
2019-08-16T00:34:35.4138830Z    |               -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4140552Z    |               lifetime `'a` defined here
2019-08-16T00:34:35.4140767Z ...
2019-08-16T00:34:35.4140767Z ...
2019-08-16T00:34:35.4141078Z LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
2019-08-16T00:34:35.4145393Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4145572Z error: lifetime may not live long enough
2019-08-16T00:34:35.4145876Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:54:13
2019-08-16T00:34:35.4146189Z    |
2019-08-16T00:34:35.4146189Z    |
2019-08-16T00:34:35.4146552Z LL | fn without_assoc<'a,'b>() {
2019-08-16T00:34:35.4147561Z    |                  -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4148245Z    |                  lifetime `'a` defined here
2019-08-16T00:34:35.4148456Z ...
2019-08-16T00:34:35.4148456Z ...
2019-08-16T00:34:35.4149168Z LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
2019-08-16T00:34:35.4149498Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4149876Z error: lifetime may not live long enough
2019-08-16T00:34:35.4150238Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:63:5
2019-08-16T00:34:35.4150332Z    |
2019-08-16T00:34:35.4150332Z    |
2019-08-16T00:34:35.4151135Z LL | fn call_with_assoc<'a,'b>() {
2019-08-16T00:34:35.4162020Z    |                    -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4162685Z    |                    lifetime `'a` defined here
2019-08-16T00:34:35.4165606Z ...
2019-08-16T00:34:35.4165606Z ...
2019-08-16T00:34:35.4165953Z LL |     call::<&'a WithAssoc<TheType<'b>>>();
2019-08-16T00:34:35.4166228Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4166359Z error: lifetime may not live long enough
2019-08-16T00:34:35.4167595Z   --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:70:5
2019-08-16T00:34:35.4167866Z    |
2019-08-16T00:34:35.4167866Z    |
2019-08-16T00:34:35.4168470Z LL | fn call_without_assoc<'a,'b>() {
2019-08-16T00:34:35.4168761Z    |                       -- -- lifetime `'b` defined here
2019-08-16T00:34:35.4169596Z    |                       lifetime `'a` defined here
2019-08-16T00:34:35.4169691Z ...
2019-08-16T00:34:35.4169691Z ...
2019-08-16T00:34:35.4169976Z LL |     call::<&'a WithoutAssoc<TheType<'b>>>(); //~ ERROR reference has a longer lifetime
2019-08-16T00:34:35.4170492Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
2019-08-16T00:34:35.4171123Z error: aborting due to 4 previous errors
2019-08-16T00:34:35.4171164Z 
2019-08-16T00:34:35.4171195Z 
2019-08-16T00:34:35.4171570Z ------------------------------------------
---
2019-08-16T00:34:35.4184380Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-16T00:34:35.4184493Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-16T00:34:35.4184545Z 
2019-08-16T00:34:35.4184600Z 
2019-08-16T00:34:35.4186637Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-16T00:34:35.4187619Z 
2019-08-16T00:34:35.4187658Z 
2019-08-16T00:34:35.4187750Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-16T00:34:35.4187855Z Build completed unsuccessfully in 1:59:03
2019-08-16T00:34:35.4187855Z Build completed unsuccessfully in 1:59:03
2019-08-16T00:34:35.4233850Z == clock drift check ==
2019-08-16T00:34:35.4244638Z   local time: Fri Aug 16 00:34:35 UTC 2019
2019-08-16T00:34:35.4797284Z   network time: Fri, 16 Aug 2019 00:34:35 GMT
2019-08-16T00:34:35.4799576Z == end clock drift check ==
2019-08-16T00:34:36.2905015Z ##[error]Bash exited with code '1'.
2019-08-16T00:34:36.2949089Z ##[section]Starting: Upload CPU usage statistics
2019-08-16T00:34:36.2964217Z ==============================================================================
2019-08-16T00:34:36.2964334Z Task         : Bash
2019-08-16T00:34:36.2964407Z Description  : Run a Bash script on macOS, Linux, or Windows
