plain
2019-07-14T03:54:51.3711045Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T03:54:51.3712429Z 
2019-07-14T03:54:51.3713518Z   git checkout -b <new-branch-name>
2019-07-14T03:54:51.3714381Z 
2019-07-14T03:54:51.3716947Z HEAD is now at ad2ec3df1 Auto merge of #62665 - Mark-Simulacrum:rollup-ftz3q9m, r=Mark-Simulacrum
2019-07-14T03:54:51.3849866Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T03:54:51.3853049Z ==============================================================================
2019-07-14T03:54:51.3853122Z Task         : Bash
2019-07-14T03:54:51.3853193Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T05:23:32.3364953Z 
2019-07-14T05:23:32.3365534Z ---- [ui (nll)] ui/c-variadic/variadic-ffi-4.rs stdout ----
2019-07-14T05:23:32.3365641Z diff of stderr:
2019-07-14T05:23:32.3365697Z 
2019-07-14T05:23:32.3365941Z 27   --> $DIR/variadic-ffi-4.rs:20:5
2019-07-14T05:23:32.3366029Z 28    |
2019-07-14T05:23:32.3366108Z 29 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.3366493Z +    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.3366617Z +    |                                               |
2019-07-14T05:23:32.3367044Z +    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.3367126Z + LL |     *ap0 = ap1;
2019-07-14T05:23:32.3367386Z +    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.3367459Z + 
2019-07-14T05:23:32.3367504Z + error: lifetime may not live long enough
2019-07-14T05:23:32.3367698Z +   --> $DIR/variadic-ffi-4.rs:20:5
2019-07-14T05:23:32.4688503Z +    |
2019-07-14T05:23:32.4688701Z + LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4689213Z 30    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4689316Z 31    |                                               |
2019-07-14T05:23:32.4689788Z 32    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4689857Z 
2019-07-14T05:23:32.4690073Z 34    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4690152Z 35 
2019-07-14T05:23:32.4690203Z 36 error: lifetime may not live long enough
2019-07-14T05:23:32.4690425Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T05:23:32.4690622Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4690693Z 38    |
2019-07-14T05:23:32.4690763Z 39 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4691069Z 40    |                                               ---                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4691132Z 
2019-07-14T05:23:32.4691373Z 44    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4691438Z 45 
2019-07-14T05:23:32.4691508Z 46 error: lifetime may not live long enough
2019-07-14T05:23:32.4691705Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T05:23:32.4691916Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4691973Z 48    |
2019-07-14T05:23:32.4692050Z 49 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4692332Z 50    |                                               ---                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4692402Z 
2019-07-14T05:23:32.4692623Z 54    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4692751Z 55 
2019-07-14T05:23:32.4692814Z 56 error[E0384]: cannot assign to immutable argument `ap0`
2019-07-14T05:23:32.4693574Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T05:23:32.4694029Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4694119Z 58    |
2019-07-14T05:23:32.4694194Z 59 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4694535Z 60    |                                               --- help: make this binding mutable: `mut ap0`
2019-07-14T05:23:32.4694686Z 62    |     ^^^^^^^^^^^^^^ cannot assign to immutable argument
2019-07-14T05:23:32.4694756Z 63 
2019-07-14T05:23:32.4694756Z 63 
2019-07-14T05:23:32.4694840Z 64 error[E0597]: `ap1` does not live long enough
2019-07-14T05:23:32.4695084Z -   --> $DIR/variadic-ffi-4.rs:24:11
2019-07-14T05:23:32.4695337Z +   --> $DIR/variadic-ffi-4.rs:25:11
2019-07-14T05:23:32.4695406Z 66    |
2019-07-14T05:23:32.4695500Z 67 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4695916Z 
2019-07-14T05:23:32.4695916Z 
2019-07-14T05:23:32.4696183Z 76    | - `ap1` dropped here while still borrowed
2019-07-14T05:23:32.4696254Z 77 
2019-07-14T05:23:32.4696332Z 78 error: lifetime may not live long enough
2019-07-14T05:23:32.4696570Z -   --> $DIR/variadic-ffi-4.rs:32:5
2019-07-14T05:23:32.4696949Z +   --> $DIR/variadic-ffi-4.rs:33:12
2019-07-14T05:23:32.4697017Z 80    |
2019-07-14T05:23:32.4697076Z 81 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4697362Z +    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4697439Z +    |                                               |
2019-07-14T05:23:32.4697689Z +    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4697830Z + LL |     *ap0 = ap1.clone();
2019-07-14T05:23:32.4698090Z +    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4698159Z + 
2019-07-14T05:23:32.4698221Z + error: lifetime may not live long enough
2019-07-14T05:23:32.4698409Z +   --> $DIR/variadic-ffi-4.rs:33:12
2019-07-14T05:23:32.4698478Z +    |
2019-07-14T05:23:32.4698713Z + LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4699212Z 82    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4699292Z 83    |                                               |
2019-07-14T05:23:32.4699559Z 84    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4699609Z 
2019-07-14T05:23:32.4699673Z 85 LL |     *ap0 = ap1.clone();
2019-07-14T05:23:32.4699893Z -    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4700151Z +    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4700433Z - error: aborting due to 9 previous errors
2019-07-14T05:23:32.4700494Z + error: aborting due to 11 previous errors
2019-07-14T05:23:32.4700720Z 89 
2019-07-14T05:23:32.4700777Z 90 Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T05:23:32.4700777Z 90 Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T05:23:32.4701050Z 91 For more information about an error, try `rustc --explain E0384`.
2019-07-14T05:23:32.4701097Z 
2019-07-14T05:23:32.4701126Z 
2019-07-14T05:23:32.4701195Z The actual stderr differed from the expected stderr.
2019-07-14T05:23:32.4701495Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll/variadic-ffi-4.nll.stderr
2019-07-14T05:23:32.4701760Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T05:23:32.4702030Z To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`
2019-07-14T05:23:32.4702304Z error: 1 errors occurred comparing output.
2019-07-14T05:23:32.4702454Z status: exit code: 1
2019-07-14T05:23:32.4702454Z status: exit code: 1
2019-07-14T05:23:32.4703538Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll/auxiliary" "-A" "unused"
2019-07-14T05:23:32.4704113Z ------------------------------------------
2019-07-14T05:23:32.4704165Z 
2019-07-14T05:23:32.4704428Z ------------------------------------------
2019-07-14T05:23:32.4704499Z stderr:
2019-07-14T05:23:32.4704499Z stderr:
2019-07-14T05:23:32.4704761Z ------------------------------------------
2019-07-14T05:23:32.4704842Z error[E0621]: explicit lifetime required in the type of `ap`
2019-07-14T05:23:32.4705153Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
2019-07-14T05:23:32.4705406Z    |
2019-07-14T05:23:32.4705752Z LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
2019-07-14T05:23:32.4706137Z    |                                                       --- help: add explicit lifetime `'f` to the type of `ap`: `core::ffi::VaListImpl<'f>`
2019-07-14T05:23:32.4706267Z LL |     ap //~ ERROR: explicit lifetime required
2019-07-14T05:23:32.4706531Z    |     ^^ lifetime `'f` required
2019-07-14T05:23:32.4706739Z 
2019-07-14T05:23:32.4706791Z error[E0621]: explicit lifetime required in the type of `ap`
2019-07-14T05:23:32.4707190Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:12:5
2019-07-14T05:23:32.4707248Z    |
2019-07-14T05:23:32.4707749Z LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
2019-07-14T05:23:32.4708095Z    |                                                   --- help: add explicit lifetime `'static` to the type of `ap`: `core::ffi::VaListImpl<'static>`
2019-07-14T05:23:32.4708190Z LL |     ap //~ ERROR: explicit lifetime required
2019-07-14T05:23:32.4708401Z    |     ^^ lifetime `'static` required
2019-07-14T05:23:32.4708438Z 
2019-07-14T05:23:32.4708487Z error: lifetime may not live long enough
2019-07-14T05:23:32.4709055Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:16:33
2019-07-14T05:23:32.4709113Z    |
2019-07-14T05:23:32.4709360Z LL |     let _ = ap.with_copy(|ap| { ap }); //~ ERROR: cannot infer an appropriate lifetime
2019-07-14T05:23:32.4709636Z    |                           ---   ^^ returning this value requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4709709Z    |                           | |
2019-07-14T05:23:32.4709958Z    |                           | return type of closure is core::ffi::VaList<'2, '_>
2019-07-14T05:23:32.4710197Z    |                           has type `core::ffi::VaList<'1, '_>`
2019-07-14T05:23:32.4710248Z 
2019-07-14T05:23:32.4710313Z error: lifetime may not live long enough
2019-07-14T05:23:32.4710856Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:20:5
2019-07-14T05:23:32.4710930Z    |
2019-07-14T05:23:32.4710986Z LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4711272Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4711364Z    |                                               |
2019-07-14T05:23:32.4711600Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4711684Z LL |     *ap0 = ap1; //~ ERROR: mismatched types
2019-07-14T05:23:32.4711892Z    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4711930Z 
2019-07-14T05:23:32.4711997Z error: lifetime may not live long enough
2019-07-14T05:23:32.4712205Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:20:5
2019-07-14T05:23:32.4712522Z    |
2019-07-14T05:23:32.4712591Z LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4713269Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4713391Z    |                                               |
2019-07-14T05:23:32.4713701Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4713805Z LL |     *ap0 = ap1; //~ ERROR: mismatched types
2019-07-14T05:23:32.4714078Z    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4714144Z 
2019-07-14T05:23:32.4714206Z error: lifetime may not live long enough
2019-07-14T05:23:32.4714490Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4714574Z    |
2019-07-14T05:23:32.4714667Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4715020Z    |                                               ---                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4715133Z    |                                               |
2019-07-14T05:23:32.4715434Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4715535Z LL |     ap0 = &mut ap1;
2019-07-14T05:23:32.4715807Z    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4715877Z 
2019-07-14T05:23:32.4715942Z error: lifetime may not live long enough
2019-07-14T05:23:32.4716426Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4716482Z    |
2019-07-14T05:23:32.4716556Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4716893Z    |                                               ---                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4716999Z    |                                               |
2019-07-14T05:23:32.4717254Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4717333Z LL |     ap0 = &mut ap1;
2019-07-14T05:23:32.4717543Z    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4717600Z 
2019-07-14T05:23:32.4717651Z error[E0384]: cannot assign to immutable argument `ap0`
2019-07-14T05:23:32.4717875Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T05:23:32.4717932Z    |
2019-07-14T05:23:32.4718005Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4718248Z    |                                               --- help: make this binding mutable: `mut ap0`
2019-07-14T05:23:32.4718328Z LL |     ap0 = &mut ap1;
2019-07-14T05:23:32.4718392Z    |     ^^^^^^^^^^^^^^ cannot assign to immutable argument
2019-07-14T05:23:32.4718446Z 
2019-07-14T05:23:32.4718500Z error[E0597]: `ap1` does not live long enough
2019-07-14T05:23:32.4718726Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:11
2019-07-14T05:23:32.4718784Z    |
2019-07-14T05:23:32.4718858Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4719108Z    |                                                    - let's call the lifetime of this reference `'1`
2019-07-14T05:23:32.4719190Z LL |     ap0 = &mut ap1;
2019-07-14T05:23:32.4719361Z    |     ------^^^^^^^^
2019-07-14T05:23:32.4719428Z    |     |     |
2019-07-14T05:23:32.4719482Z    |     |     borrowed value does not live long enough
2019-07-14T05:23:32.4719706Z    |     assignment requires that `ap1` is borrowed for `'1`
2019-07-14T05:23:32.4719828Z LL | }
2019-07-14T05:23:32.4719828Z LL | }
2019-07-14T05:23:32.4720013Z    | - `ap1` dropped here while still borrowed
2019-07-14T05:23:32.4720057Z 
2019-07-14T05:23:32.4720119Z error: lifetime may not live long enough
2019-07-14T05:23:32.4720418Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:33:12
2019-07-14T05:23:32.4720496Z    |
2019-07-14T05:23:32.4720553Z LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4720837Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4721100Z    |                                               |
2019-07-14T05:23:32.4721340Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4721426Z LL |     *ap0 = ap1.clone(); //~ ERROR: mismatched types
2019-07-14T05:23:32.4721650Z    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4721710Z 
2019-07-14T05:23:32.4721758Z error: lifetime may not live long enough
2019-07-14T05:23:32.4721992Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:33:12
2019-07-14T05:23:32.4722053Z    |
2019-07-14T05:23:32.4722206Z LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T05:23:32.4722483Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T05:23:32.4722575Z    |                                               |
2019-07-14T05:23:32.4722908Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T05:23:32.4722995Z LL |     *ap0 = ap1.clone(); //~ ERROR: mismatched types
2019-07-14T05:23:32.4723547Z    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T05:23:32.4723684Z error: aborting due to 11 previous errors
2019-07-14T05:23:32.4723726Z 
2019-07-14T05:23:32.4723809Z Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T05:23:32.4724223Z For more information about an error, try `rustc --explain E0384`.
---
2019-07-14T05:23:32.4725863Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-14T05:23:32.4725965Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-14T05:23:32.4726021Z 
2019-07-14T05:23:32.4726075Z 
2019-07-14T05:23:32.4728876Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-07-14T05:23:32.4729635Z 
2019-07-14T05:23:32.4729755Z 
2019-07-14T05:23:32.4729811Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-14T05:23:32.4729891Z Build completed unsuccessfully in 1:24:15
2019-07-14T05:23:32.4729891Z Build completed unsuccessfully in 1:24:15
2019-07-14T05:23:32.8511776Z ##[error]Bash exited with code '1'.
2019-07-14T05:23:32.8555901Z ##[section]Starting: Upload CPU usage statistics
2019-07-14T05:23:32.8564276Z ==============================================================================
2019-07-14T05:23:32.8564371Z Task         : Bash
2019-07-14T05:23:32.8564456Z Description  : Run a Bash script on macOS, Linux, or Windows
