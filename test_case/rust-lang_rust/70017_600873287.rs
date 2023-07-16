plain
2020-03-18T20:43:07.3306796Z ========================== Starting Command Output ===========================
2020-03-18T20:43:07.3309253Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4e1ccbe8-a053-4397-9d0a-1bad7936445a.sh
2020-03-18T20:43:07.3309503Z 
2020-03-18T20:43:07.3313496Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T20:43:07.3328949Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-18T20:43:07.3331568Z Task         : Get sources
2020-03-18T20:43:07.3331817Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T20:43:07.3332073Z Version      : 1.0.0
2020-03-18T20:43:07.3332235Z Author       : Microsoft
---
2020-03-18T20:43:08.6731461Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T20:43:08.6736076Z ##[command]git config gc.auto 0
2020-03-18T20:43:08.6738597Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T20:43:08.6740914Z ##[command]git config --get-all http.proxy
2020-03-18T20:43:08.6746021Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70017/merge:refs/remotes/pull/70017/merge
---
2020-03-18T21:32:40.2561362Z .................................................................................................... 1700/9798
2020-03-18T21:32:43.9127050Z ...............................................FFF.................................................. 1800/9798
2020-03-18T21:32:53.6219701Z ............................................................................i....................... 1900/9798
2020-03-18T21:32:59.0286957Z .................................................................................................... 2000/9798
2020-03-18T21:33:05.9616291Z ..................................................................iiiii............................. 2100/9798
2020-03-18T21:33:21.4629011Z .................................................................................................... 2300/9798
2020-03-18T21:33:23.3727269Z .................................................................................................... 2400/9798
2020-03-18T21:33:25.7436658Z .................................................................................................... 2500/9798
2020-03-18T21:33:42.7413809Z .................................................................................................... 2600/9798
---
2020-03-18T21:36:00.9803857Z ......................................i...............i............................................. 5000/9798
2020-03-18T21:36:08.7609007Z .................................................................................................... 5100/9798
2020-03-18T21:36:14.2622790Z .................................................................................i.................. 5200/9798
2020-03-18T21:36:19.0265810Z .................................................................................................... 5300/9798
2020-03-18T21:36:27.5754145Z ..............................................................ii.ii........i...i.................... 5400/9798
2020-03-18T21:36:34.4968808Z .i.................................................................................................. 5600/9798
2020-03-18T21:36:42.5996518Z .................................................................................................... 5700/9798
2020-03-18T21:36:48.0077988Z .......................................................i............................................ 5800/9798
2020-03-18T21:36:53.5328175Z .................................................................................................... 5900/9798
2020-03-18T21:36:53.5328175Z .................................................................................................... 5900/9798
2020-03-18T21:37:00.6213461Z .................................................................................................... 6000/9798
2020-03-18T21:37:07.2888360Z .................................................ii...i..ii...........i............................. 6100/9798
2020-03-18T21:37:25.0967776Z .................................................................................................... 6300/9798
2020-03-18T21:37:28.2084919Z .................................................................................................... 6400/9798
2020-03-18T21:37:28.2084919Z .................................................................................................... 6400/9798
2020-03-18T21:37:31.7334321Z ...............................................................................i..ii................ 6500/9798
2020-03-18T21:37:51.6563298Z .................................................................................................... 6700/9798
2020-03-18T21:37:59.4860318Z .............................................................................i...................... 6800/9798
2020-03-18T21:38:01.2644646Z .................................................................................................... 6900/9798
2020-03-18T21:38:03.0914226Z .................................................................................................... 7000/9798
---
2020-03-18T21:39:32.1261837Z .................................................................................................... 7800/9798
2020-03-18T21:39:36.8438832Z .................................................................................................... 7900/9798
2020-03-18T21:39:42.1121157Z ...............................................................i.................................... 8000/9798
2020-03-18T21:39:50.9254009Z .................................................................................................... 8100/9798
2020-03-18T21:39:55.5997325Z ............iiiiiiiiii.i............................................................................ 8200/9798
2020-03-18T21:40:07.3718466Z .................................................................................................... 8400/9798
2020-03-18T21:40:14.2863854Z .................................................................................................... 8500/9798
2020-03-18T21:40:25.8747662Z .................................................................................................... 8600/9798
2020-03-18T21:40:31.4622466Z .................................................................................................... 8700/9798
---
2020-03-18T21:41:57.9338698Z .................................................................................................... 9700/9798
2020-03-18T21:42:10.4653408Z ..i...............................................................................................
2020-03-18T21:42:10.4658432Z failures:
2020-03-18T21:42:10.4678281Z 
2020-03-18T21:42:10.4679867Z ---- [ui] ui/associated-const/lints-used-unused.rs#unused stdout ----
2020-03-18T21:42:10.4680698Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4681359Z   --> $DIR/lints-used-unused.rs:16:20
2020-03-18T21:42:10.4681684Z    |
2020-03-18T21:42:10.4681975Z LL |     const N: i32 = 1 << 42;
---
2020-03-18T21:42:10.4693809Z 
2020-03-18T21:42:10.4694050Z 
2020-03-18T21:42:10.4694821Z 
2020-03-18T21:42:10.4695109Z The actual stderr differed from the expected stderr.
2020-03-18T21:42:10.4696692Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused/lints-used-unused.unused.stderr
2020-03-18T21:42:10.4698246Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T21:42:10.4699537Z To only update this specific test, also pass `--test-args associated-const/lints-used-unused.rs`
2020-03-18T21:42:10.4702334Z error in revision `unused`: 1 errors occurred comparing output.
2020-03-18T21:42:10.4703644Z status: exit code: 1
2020-03-18T21:42:10.4703644Z status: exit code: 1
2020-03-18T21:42:10.4705938Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/lints-used-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unused" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused/auxiliary"
2020-03-18T21:42:10.4712204Z ------------------------------------------
2020-03-18T21:42:10.4713559Z 
2020-03-18T21:42:10.4714675Z ------------------------------------------
2020-03-18T21:42:10.4717045Z stderr:
---
2020-03-18T21:42:10.4732794Z 
2020-03-18T21:42:10.4733339Z ------------------------------------------
2020-03-18T21:42:10.4733643Z 
2020-03-18T21:42:10.4733861Z 
2020-03-18T21:42:10.4734427Z ---- [ui] ui/associated-const/lints-used-unused.rs#used stdout ----
2020-03-18T21:42:10.4735201Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4735801Z   --> $DIR/lints-used-unused.rs:16:20
2020-03-18T21:42:10.4736136Z    |
2020-03-18T21:42:10.4736451Z LL |     const N: i32 = 1 << 42;
---
2020-03-18T21:42:10.4745679Z 
2020-03-18T21:42:10.4745787Z 
2020-03-18T21:42:10.4745876Z 
2020-03-18T21:42:10.4746072Z The actual stderr differed from the expected stderr.
2020-03-18T21:42:10.4746748Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used/lints-used-unused.used.stderr
2020-03-18T21:42:10.4747385Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T21:42:10.4747960Z To only update this specific test, also pass `--test-args associated-const/lints-used-unused.rs`
2020-03-18T21:42:10.4748435Z error in revision `used`: 1 errors occurred comparing output.
2020-03-18T21:42:10.4748694Z status: exit code: 1
2020-03-18T21:42:10.4748694Z status: exit code: 1
2020-03-18T21:42:10.4753150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/lints-used-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "used" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used/auxiliary"
2020-03-18T21:42:10.4754837Z ------------------------------------------
2020-03-18T21:42:10.4755000Z 
2020-03-18T21:42:10.4755334Z ------------------------------------------
2020-03-18T21:42:10.4755519Z stderr:
---
2020-03-18T21:42:10.4763053Z ---- [ui] ui/consts/const-eval/ice-generic-assoc-const.rs stdout ----
2020-03-18T21:42:10.4763228Z 
2020-03-18T21:42:10.4763554Z error: test compilation failed although it shouldn't!
2020-03-18T21:42:10.4763763Z status: exit code: 1
2020-03-18T21:42:10.4765441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ice-generic-assoc-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ice-generic-assoc-const/auxiliary"
2020-03-18T21:42:10.4766791Z ------------------------------------------
2020-03-18T21:42:10.4766935Z 
2020-03-18T21:42:10.4767222Z ------------------------------------------
2020-03-18T21:42:10.4767384Z stderr:
2020-03-18T21:42:10.4767384Z stderr:
2020-03-18T21:42:10.4767692Z ------------------------------------------
2020-03-18T21:42:10.4767929Z error[E0080]: it is undefined behavior to use this value
2020-03-18T21:42:10.4768391Z   --> /checkout/src/test/ui/consts/const-eval/ice-generic-assoc-const.rs:10:5
2020-03-18T21:42:10.4768626Z    |
2020-03-18T21:42:10.4768818Z LL |     const NULL: Self = core::ptr::null::<T>();
2020-03-18T21:42:10.4769189Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined pointer
2020-03-18T21:42:10.4769458Z    |
2020-03-18T21:42:10.4770176Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-18T21:42:10.4770730Z error: aborting due to previous error
2020-03-18T21:42:10.4770901Z 
2020-03-18T21:42:10.4771273Z For more information about this error, try `rustc --explain E0080`.
2020-03-18T21:42:10.4771446Z 
---
2020-03-18T21:42:10.4773052Z 8 
2020-03-18T21:42:10.4773223Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4773596Z +   --> $DIR/issue-69020.rs:21:22
2020-03-18T21:42:10.4773759Z +    |
2020-03-18T21:42:10.4774129Z + LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4774731Z +    |                      |
2020-03-18T21:42:10.4774951Z +    |                      attempt to negate with overflow
2020-03-18T21:42:10.4775151Z +    |
2020-03-18T21:42:10.4775325Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4775325Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4775491Z + 
2020-03-18T21:42:10.4775675Z 9 error: this arithmetic operation will overflow
2020-03-18T21:42:10.4776030Z 10   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4776190Z 11    |
2020-03-18T21:42:10.4776280Z 
2020-03-18T21:42:10.4776484Z 12 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4778978Z 14 
2020-03-18T21:42:10.4779167Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4779691Z +   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4779851Z +    |
2020-03-18T21:42:10.4779851Z +    |
2020-03-18T21:42:10.4780059Z + LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4780665Z +    |                      |
2020-03-18T21:42:10.4780892Z +    |                      attempt to add with overflow
2020-03-18T21:42:10.4781065Z + 
2020-03-18T21:42:10.4781229Z 15 error: this operation will panic at runtime
---
2020-03-18T21:42:10.4782335Z 22 
2020-03-18T21:42:10.4782506Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4782872Z +   --> $DIR/issue-69020.rs:25:22
2020-03-18T21:42:10.4783029Z +    |
2020-03-18T21:42:10.4783205Z + LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4783779Z +    |                      |
2020-03-18T21:42:10.4783987Z +    |                      attempt to divide by zero
2020-03-18T21:42:10.4784154Z + 
2020-03-18T21:42:10.4784335Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4784335Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4784684Z 24   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4784842Z 25    |
2020-03-18T21:42:10.4784945Z 
2020-03-18T21:42:10.4785123Z 26 LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4785696Z 28 
2020-03-18T21:42:10.4785995Z - error: aborting due to 4 previous errors
2020-03-18T21:42:10.4786224Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4786782Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4786782Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4786979Z +    |
2020-03-18T21:42:10.4787441Z + LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4788235Z +    |                      |
2020-03-18T21:42:10.4788540Z +    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4788796Z + 
2020-03-18T21:42:10.4789043Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4789043Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4789225Z 30 
2020-03-18T21:42:10.4789339Z 31 
2020-03-18T21:42:10.4789436Z 
2020-03-18T21:42:10.4789630Z 
2020-03-18T21:42:10.4789823Z The actual stderr differed from the expected stderr.
2020-03-18T21:42:10.4790473Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.noopt/issue-69020.noopt.stderr
2020-03-18T21:42:10.4791077Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T21:42:10.4791606Z To only update this specific test, also pass `--test-args consts/issue-69020.rs`
2020-03-18T21:42:10.4791825Z 
2020-03-18T21:42:10.4792059Z error in revision `noopt`: 1 errors occurred comparing output.
2020-03-18T21:42:10.4792322Z status: exit code: 1
2020-03-18T21:42:10.4794336Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-69020.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.noopt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.noopt/auxiliary"
2020-03-18T21:42:10.4796132Z ------------------------------------------
2020-03-18T21:42:10.4796318Z 
2020-03-18T21:42:10.4796678Z ------------------------------------------
2020-03-18T21:42:10.4797659Z stderr:
2020-03-18T21:42:10.4797659Z stderr:
2020-03-18T21:42:10.4798029Z ------------------------------------------
2020-03-18T21:42:10.4798289Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4798891Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4799124Z    |
2020-03-18T21:42:10.4799635Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4800360Z    |
2020-03-18T21:42:10.4800571Z    = note: `#[deny(arithmetic_overflow)]` on by default
2020-03-18T21:42:10.4800754Z 
2020-03-18T21:42:10.4800942Z error: any use of this value will cause an error
2020-03-18T21:42:10.4800942Z error: any use of this value will cause an error
2020-03-18T21:42:10.4801433Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4801645Z    |
2020-03-18T21:42:10.4802010Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4802672Z    |                      |
2020-03-18T21:42:10.4803024Z    |                      attempt to negate with overflow
2020-03-18T21:42:10.4808116Z    |
2020-03-18T21:42:10.4808307Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4808307Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4808455Z 
2020-03-18T21:42:10.4808615Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4814877Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4815132Z    |
2020-03-18T21:42:10.4815355Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4815895Z 
2020-03-18T21:42:10.4816084Z error: any use of this value will cause an error
2020-03-18T21:42:10.4816809Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4817044Z    |
2020-03-18T21:42:10.4817044Z    |
2020-03-18T21:42:10.4817260Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4817966Z    |                      |
2020-03-18T21:42:10.4818204Z    |                      attempt to add with overflow
2020-03-18T21:42:10.4818520Z 
2020-03-18T21:42:10.4818718Z error: this operation will panic at runtime
2020-03-18T21:42:10.4818718Z error: this operation will panic at runtime
2020-03-18T21:42:10.4819183Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4819395Z    |
2020-03-18T21:42:10.4819681Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4820161Z    |
2020-03-18T21:42:10.4820369Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-18T21:42:10.4820570Z 
2020-03-18T21:42:10.4820756Z error: any use of this value will cause an error
2020-03-18T21:42:10.4820756Z error: any use of this value will cause an error
2020-03-18T21:42:10.4821224Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4821449Z    |
2020-03-18T21:42:10.4821642Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4822276Z    |                      |
2020-03-18T21:42:10.4822507Z    |                      attempt to divide by zero
2020-03-18T21:42:10.4822676Z 
2020-03-18T21:42:10.4822859Z error: this operation will panic at runtime
2020-03-18T21:42:10.4822859Z error: this operation will panic at runtime
2020-03-18T21:42:10.4823319Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4823669Z    |
2020-03-18T21:42:10.4823874Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4824503Z 
2020-03-18T21:42:10.4824693Z error: any use of this value will cause an error
2020-03-18T21:42:10.4825177Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4825388Z    |
2020-03-18T21:42:10.4825388Z    |
2020-03-18T21:42:10.4825590Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4826437Z    |                      |
2020-03-18T21:42:10.4826959Z    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4827204Z 
2020-03-18T21:42:10.4827401Z error: aborting due to 8 previous errors
---
2020-03-18T21:42:10.4829904Z 8 
2020-03-18T21:42:10.4830115Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4830543Z +   --> $DIR/issue-69020.rs:21:22
2020-03-18T21:42:10.4830904Z +    |
2020-03-18T21:42:10.4831329Z + LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4832046Z +    |                      |
2020-03-18T21:42:10.4832329Z +    |                      attempt to negate with overflow
2020-03-18T21:42:10.4832553Z +    |
2020-03-18T21:42:10.4832773Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4832773Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4832992Z + 
2020-03-18T21:42:10.4833197Z 9 error: this arithmetic operation will overflow
2020-03-18T21:42:10.4833637Z 10   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4833856Z 11    |
2020-03-18T21:42:10.4833967Z 
2020-03-18T21:42:10.4834198Z 12 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4834802Z 14 
2020-03-18T21:42:10.4835014Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4835451Z +   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4835665Z +    |
2020-03-18T21:42:10.4835665Z +    |
2020-03-18T21:42:10.4836048Z + LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4836822Z +    |                      |
2020-03-18T21:42:10.4837084Z +    |                      attempt to add with overflow
2020-03-18T21:42:10.4837295Z + 
2020-03-18T21:42:10.4837588Z 15 error: this operation will panic at runtime
---
2020-03-18T21:42:10.4839051Z 22 
2020-03-18T21:42:10.4839390Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4839819Z +   --> $DIR/issue-69020.rs:25:22
2020-03-18T21:42:10.4839999Z +    |
2020-03-18T21:42:10.4840200Z + LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4840851Z +    |                      |
2020-03-18T21:42:10.4841090Z +    |                      attempt to divide by zero
2020-03-18T21:42:10.4841300Z + 
2020-03-18T21:42:10.4841490Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4841490Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4841893Z 24   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4842204Z 25    |
2020-03-18T21:42:10.4842318Z 
2020-03-18T21:42:10.4842523Z 26 LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4843252Z 28 
2020-03-18T21:42:10.4843558Z - error: aborting due to 4 previous errors
2020-03-18T21:42:10.4843789Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4844154Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4844154Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4844311Z +    |
2020-03-18T21:42:10.4844495Z + LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4845080Z +    |                      |
2020-03-18T21:42:10.4845445Z +    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4845685Z + 
2020-03-18T21:42:10.4845842Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4845842Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4845998Z 30 
2020-03-18T21:42:10.4846111Z 31 
2020-03-18T21:42:10.4846198Z 
2020-03-18T21:42:10.4846277Z 
2020-03-18T21:42:10.4846442Z The actual stderr differed from the expected stderr.
2020-03-18T21:42:10.4847011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt/issue-69020.opt.stderr
2020-03-18T21:42:10.4847530Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T21:42:10.4847992Z To only update this specific test, also pass `--test-args consts/issue-69020.rs`
2020-03-18T21:42:10.4848483Z error in revision `opt`: 1 errors occurred comparing output.
2020-03-18T21:42:10.4848705Z status: exit code: 1
2020-03-18T21:42:10.4848705Z status: exit code: 1
2020-03-18T21:42:10.4850301Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-69020.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt/auxiliary"
2020-03-18T21:42:10.4851901Z ------------------------------------------
2020-03-18T21:42:10.4852061Z 
2020-03-18T21:42:10.4852514Z ------------------------------------------
2020-03-18T21:42:10.4852725Z stderr:
2020-03-18T21:42:10.4852725Z stderr:
2020-03-18T21:42:10.4853075Z ------------------------------------------
2020-03-18T21:42:10.4853328Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4853792Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4854006Z    |
2020-03-18T21:42:10.4854378Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4855034Z    |
2020-03-18T21:42:10.4855294Z    = note: `#[deny(arithmetic_overflow)]` on by default
2020-03-18T21:42:10.4855480Z 
2020-03-18T21:42:10.4855681Z error: any use of this value will cause an error
2020-03-18T21:42:10.4855681Z error: any use of this value will cause an error
2020-03-18T21:42:10.4856155Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4856530Z    |
2020-03-18T21:42:10.4856938Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4857587Z    |                      |
2020-03-18T21:42:10.4857843Z    |                      attempt to negate with overflow
2020-03-18T21:42:10.4858221Z    |
2020-03-18T21:42:10.4858427Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4858427Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4858608Z 
2020-03-18T21:42:10.4858820Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4859308Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4859534Z    |
2020-03-18T21:42:10.4859882Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4860391Z 
2020-03-18T21:42:10.4860590Z error: any use of this value will cause an error
2020-03-18T21:42:10.4861044Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4861259Z    |
2020-03-18T21:42:10.4861259Z    |
2020-03-18T21:42:10.4861472Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4862266Z    |                      |
2020-03-18T21:42:10.4862502Z    |                      attempt to add with overflow
2020-03-18T21:42:10.4862691Z 
2020-03-18T21:42:10.4862869Z error: this operation will panic at runtime
2020-03-18T21:42:10.4862869Z error: this operation will panic at runtime
2020-03-18T21:42:10.4863324Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4863549Z    |
2020-03-18T21:42:10.4863743Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4864239Z    |
2020-03-18T21:42:10.4864453Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-18T21:42:10.4864637Z 
2020-03-18T21:42:10.4864822Z error: any use of this value will cause an error
2020-03-18T21:42:10.4864822Z error: any use of this value will cause an error
2020-03-18T21:42:10.4865289Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4865505Z    |
2020-03-18T21:42:10.4865697Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4866332Z    |                      |
2020-03-18T21:42:10.4866561Z    |                      attempt to divide by zero
2020-03-18T21:42:10.4866731Z 
2020-03-18T21:42:10.4866924Z error: this operation will panic at runtime
2020-03-18T21:42:10.4866924Z error: this operation will panic at runtime
2020-03-18T21:42:10.4867367Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4867578Z    |
2020-03-18T21:42:10.4867796Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4868400Z 
2020-03-18T21:42:10.4868608Z error: any use of this value will cause an error
2020-03-18T21:42:10.4869061Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4869268Z    |
2020-03-18T21:42:10.4869268Z    |
2020-03-18T21:42:10.4869650Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4870214Z    |                      |
2020-03-18T21:42:10.4870485Z    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4870866Z 
2020-03-18T21:42:10.4871039Z error: aborting due to 8 previous errors
---
2020-03-18T21:42:10.4873272Z 8 
2020-03-18T21:42:10.4873495Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4873915Z +   --> $DIR/issue-69020.rs:21:22
2020-03-18T21:42:10.4874185Z +    |
2020-03-18T21:42:10.4874573Z + LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4875254Z +    |                      |
2020-03-18T21:42:10.4875504Z +    |                      attempt to negate with overflow
2020-03-18T21:42:10.4875712Z +    |
2020-03-18T21:42:10.4875929Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4875929Z +    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4876118Z + 
2020-03-18T21:42:10.4876310Z 9 error: this arithmetic operation will overflow
2020-03-18T21:42:10.4876729Z 10   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4876913Z 11    |
2020-03-18T21:42:10.4877015Z 
2020-03-18T21:42:10.4877227Z 12 LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4877789Z 14 
2020-03-18T21:42:10.4877983Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4878405Z +   --> $DIR/issue-69020.rs:23:22
2020-03-18T21:42:10.4879043Z +    |
2020-03-18T21:42:10.4879043Z +    |
2020-03-18T21:42:10.4879285Z + LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4880055Z +    |                      |
2020-03-18T21:42:10.4880317Z +    |                      attempt to add with overflow
2020-03-18T21:42:10.4880544Z + 
2020-03-18T21:42:10.4880750Z 15 error: this operation will panic at runtime
---
2020-03-18T21:42:10.4882279Z 22 
2020-03-18T21:42:10.4882493Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4882945Z +   --> $DIR/issue-69020.rs:25:22
2020-03-18T21:42:10.4883154Z +    |
2020-03-18T21:42:10.4883372Z + LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4884079Z +    |                      |
2020-03-18T21:42:10.4884336Z +    |                      attempt to divide by zero
2020-03-18T21:42:10.4884542Z + 
2020-03-18T21:42:10.4884743Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4884743Z 23 error: this operation will panic at runtime
2020-03-18T21:42:10.4885191Z 24   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4885388Z 25    |
2020-03-18T21:42:10.4885499Z 
2020-03-18T21:42:10.4885733Z 26 LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4886429Z 28 
2020-03-18T21:42:10.4886813Z - error: aborting due to 4 previous errors
2020-03-18T21:42:10.4887102Z + error: any use of this value will cause an error
2020-03-18T21:42:10.4887538Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4887538Z +   --> $DIR/issue-69020.rs:27:22
2020-03-18T21:42:10.4887748Z +    |
2020-03-18T21:42:10.4887977Z + LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4888684Z +    |                      |
2020-03-18T21:42:10.4889023Z +    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4889302Z + 
2020-03-18T21:42:10.4889495Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4889495Z + error: aborting due to 8 previous errors
2020-03-18T21:42:10.4889702Z 30 
2020-03-18T21:42:10.4889824Z 31 
2020-03-18T21:42:10.4889928Z 
2020-03-18T21:42:10.4890024Z 
2020-03-18T21:42:10.4890243Z The actual stderr differed from the expected stderr.
2020-03-18T21:42:10.4891015Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt_with_overflow_checks/issue-69020.opt_with_overflow_checks.stderr
2020-03-18T21:42:10.4891899Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T21:42:10.4892466Z To only update this specific test, also pass `--test-args consts/issue-69020.rs`
2020-03-18T21:42:10.4892793Z 
2020-03-18T21:42:10.4893016Z error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
2020-03-18T21:42:10.4893325Z status: exit code: 1
2020-03-18T21:42:10.4895091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-69020.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt_with_overflow_checks" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-69020.opt_with_overflow_checks/auxiliary"
2020-03-18T21:42:10.4896503Z ------------------------------------------
2020-03-18T21:42:10.4896654Z 
2020-03-18T21:42:10.4896940Z ------------------------------------------
2020-03-18T21:42:10.4897105Z stderr:
2020-03-18T21:42:10.4897105Z stderr:
2020-03-18T21:42:10.4897396Z ------------------------------------------
2020-03-18T21:42:10.4897629Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4898023Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4898209Z    |
2020-03-18T21:42:10.4898543Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4899001Z    |
2020-03-18T21:42:10.4899196Z    = note: `#[deny(arithmetic_overflow)]` on by default
2020-03-18T21:42:10.4899355Z 
2020-03-18T21:42:10.4899517Z error: any use of this value will cause an error
2020-03-18T21:42:10.4899517Z error: any use of this value will cause an error
2020-03-18T21:42:10.4899929Z   --> /checkout/src/test/ui/consts/issue-69020.rs:21:22
2020-03-18T21:42:10.4900113Z    |
2020-03-18T21:42:10.4900431Z LL |     const NEG: i32 = -i32::MIN + T::NEG;
2020-03-18T21:42:10.4905358Z    |                      |
2020-03-18T21:42:10.4905674Z    |                      attempt to negate with overflow
2020-03-18T21:42:10.4905846Z    |
2020-03-18T21:42:10.4906030Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4906030Z    = note: `#[deny(const_err)]` on by default
2020-03-18T21:42:10.4906177Z 
2020-03-18T21:42:10.4906337Z error: this arithmetic operation will overflow
2020-03-18T21:42:10.4906875Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4907061Z    |
2020-03-18T21:42:10.4907248Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4907705Z 
2020-03-18T21:42:10.4907868Z error: any use of this value will cause an error
2020-03-18T21:42:10.4908275Z   --> /checkout/src/test/ui/consts/issue-69020.rs:23:22
2020-03-18T21:42:10.4908476Z    |
2020-03-18T21:42:10.4908476Z    |
2020-03-18T21:42:10.4908661Z LL |     const ADD: i32 = (i32::MAX+1) + T::ADD;
2020-03-18T21:42:10.4909263Z    |                      |
2020-03-18T21:42:10.4909469Z    |                      attempt to add with overflow
2020-03-18T21:42:10.4909753Z 
2020-03-18T21:42:10.4909910Z error: this operation will panic at runtime
2020-03-18T21:42:10.4909910Z error: this operation will panic at runtime
2020-03-18T21:42:10.4910324Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4910509Z    |
2020-03-18T21:42:10.4910855Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4911350Z    |
2020-03-18T21:42:10.4911556Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-18T21:42:10.4911752Z 
2020-03-18T21:42:10.4911936Z error: any use of this value will cause an error
2020-03-18T21:42:10.4911936Z error: any use of this value will cause an error
2020-03-18T21:42:10.4912527Z   --> /checkout/src/test/ui/consts/issue-69020.rs:25:22
2020-03-18T21:42:10.4912755Z    |
2020-03-18T21:42:10.4912949Z LL |     const DIV: i32 = (1/0) + T::DIV;
2020-03-18T21:42:10.4913644Z    |                      |
2020-03-18T21:42:10.4913890Z    |                      attempt to divide by zero
2020-03-18T21:42:10.4914062Z 
2020-03-18T21:42:10.4914240Z error: this operation will panic at runtime
2020-03-18T21:42:10.4914240Z error: this operation will panic at runtime
2020-03-18T21:42:10.4914715Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4914924Z    |
2020-03-18T21:42:10.4915126Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4915745Z 
2020-03-18T21:42:10.4915935Z error: any use of this value will cause an error
2020-03-18T21:42:10.4916390Z   --> /checkout/src/test/ui/consts/issue-69020.rs:27:22
2020-03-18T21:42:10.4919146Z    |
2020-03-18T21:42:10.4919146Z    |
2020-03-18T21:42:10.4919369Z LL |     const OOB: i32 = [1][1] + T::OOB;
2020-03-18T21:42:10.4920151Z    |                      |
2020-03-18T21:42:10.4920449Z    |                      index out of bounds: the len is 1 but the index is 1
2020-03-18T21:42:10.4920692Z 
2020-03-18T21:42:10.4921090Z error: aborting due to 8 previous errors
2020-03-18T21:42:10.4921090Z error: aborting due to 8 previous errors
2020-03-18T21:42:10.4921258Z 
2020-03-18T21:42:10.4921347Z 
2020-03-18T21:42:10.4921697Z ------------------------------------------
2020-03-18T21:42:10.4921853Z 
2020-03-18T21:42:10.4921957Z 
2020-03-18T21:42:10.4922045Z 
2020-03-18T21:42:10.4922167Z failures:
2020-03-18T21:42:10.4922545Z     [ui] ui/associated-const/lints-used-unused.rs#unused
2020-03-18T21:42:10.4923077Z     [ui] ui/associated-const/lints-used-unused.rs#used
2020-03-18T21:42:10.4923827Z     [ui] ui/consts/issue-69020.rs#noopt
2020-03-18T21:42:10.4924178Z     [ui] ui/consts/issue-69020.rs#opt
2020-03-18T21:42:10.4924558Z     [ui] ui/consts/issue-69020.rs#opt_with_overflow_checks
2020-03-18T21:42:10.4924720Z 
2020-03-18T21:42:10.4924720Z 
2020-03-18T21:42:10.4925139Z test result: FAILED. 9735 passed; 6 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-18T21:42:10.4925359Z 
2020-03-18T21:42:10.4925823Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T21:42:10.4926155Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T21:42:10.4926356Z 
2020-03-18T21:42:10.4926434Z 
2020-03-18T21:42:10.4929960Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T21:42:10.4932462Z 
2020-03-18T21:42:10.4932556Z 
2020-03-18T21:42:10.4933133Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T21:42:10.4933498Z Build completed unsuccessfully in 0:55:22
2020-03-18T21:42:10.4933498Z Build completed unsuccessfully in 0:55:22
2020-03-18T21:42:10.4933767Z == clock drift check ==
2020-03-18T21:42:10.4934062Z   local time: Wed Mar 18 21:42:10 UTC 2020
2020-03-18T21:42:11.0350174Z   network time: Wed, 18 Mar 2020 21:42:11 GMT
2020-03-18T21:42:11.0350700Z == end clock drift check ==
2020-03-18T21:42:11.4253542Z 
2020-03-18T21:42:11.4328681Z ##[error]Bash exited with code '1'.
2020-03-18T21:42:11.4339621Z ##[section]Finishing: Run build
2020-03-18T21:42:11.4385114Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-18T21:42:11.4389310Z Task         : Get sources
2020-03-18T21:42:11.4389724Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T21:42:11.4389999Z Version      : 1.0.0
2020-03-18T21:42:11.4390183Z Author       : Microsoft
2020-03-18T21:42:11.4390183Z Author       : Microsoft
2020-03-18T21:42:11.4390477Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T21:42:11.4390820Z ==============================================================================
2020-03-18T21:42:11.7314538Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T21:42:11.7354570Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-18T21:42:11.7427381Z Cleaning up task key
2020-03-18T21:42:11.7428467Z Start cleaning up orphan processes.
2020-03-18T21:42:11.7677438Z Terminate orphan process: pid (3805) (python)
2020-03-18T21:42:11.7716406Z ##[section]Finishing: Finalize Job
