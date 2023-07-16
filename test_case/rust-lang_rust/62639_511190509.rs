plain
2019-07-14T08:49:31.9673556Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T08:49:31.9673620Z 
2019-07-14T08:49:31.9673790Z   git checkout -b <new-branch-name>
2019-07-14T08:49:31.9673823Z 
2019-07-14T08:49:31.9674053Z HEAD is now at b389cbb9c Auto merge of #62639 - immunant:invariant_valistimpl, r=eddyb
2019-07-14T08:49:31.9795270Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T08:49:31.9797612Z ==============================================================================
2019-07-14T08:49:31.9797683Z Task         : Bash
2019-07-14T08:49:31.9797755Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T10:19:26.7364442Z 
2019-07-14T10:19:26.7365090Z ---- [ui (nll)] ui/c-variadic/variadic-ffi-4.rs stdout ----
2019-07-14T10:19:26.7365160Z diff of stderr:
2019-07-14T10:19:26.7365194Z 
2019-07-14T10:19:26.7365595Z 27   --> $DIR/variadic-ffi-4.rs:20:5
2019-07-14T10:19:26.7365659Z 28    |
2019-07-14T10:19:26.7365740Z 29 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7366028Z +    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7366166Z +    |                                               |
2019-07-14T10:19:26.7366444Z +    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7366513Z + LL |     *ap0 = ap1;
2019-07-14T10:19:26.7366748Z +    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7366810Z + 
2019-07-14T10:19:26.7366876Z + error: lifetime may not live long enough
2019-07-14T10:19:26.7367070Z +   --> $DIR/variadic-ffi-4.rs:20:5
2019-07-14T10:19:26.7367142Z +    |
2019-07-14T10:19:26.7367204Z + LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7367503Z 30    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7367585Z 31    |                                               |
2019-07-14T10:19:26.7368035Z 32    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7368263Z 
2019-07-14T10:19:26.7368526Z 34    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7368591Z 35 
2019-07-14T10:19:26.7368820Z 36 error: lifetime may not live long enough
2019-07-14T10:19:26.7369013Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T10:19:26.7369218Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7369273Z 38    |
2019-07-14T10:19:26.7369351Z 39 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7369630Z 40    |                                               ---                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7369700Z 
2019-07-14T10:19:26.7369920Z 44    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7370001Z 45 
2019-07-14T10:19:26.7370051Z 46 error: lifetime may not live long enough
2019-07-14T10:19:26.7370257Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T10:19:26.7370916Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7371150Z 48    |
2019-07-14T10:19:26.7371225Z 49 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7371624Z 50    |                                               ---                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7371690Z 
2019-07-14T10:19:26.7371956Z 54    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7372050Z 55 
2019-07-14T10:19:26.7372117Z 56 error[E0384]: cannot assign to immutable argument `ap0`
2019-07-14T10:19:26.7372374Z -   --> $DIR/variadic-ffi-4.rs:24:5
2019-07-14T10:19:26.7372619Z +   --> $DIR/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7372686Z 58    |
2019-07-14T10:19:26.7372776Z 59 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7373094Z 60    |                                               --- help: make this binding mutable: `mut ap0`
2019-07-14T10:19:26.7373390Z 62    |     ^^^^^^^^^^^^^^ cannot assign to immutable argument
2019-07-14T10:19:26.7373460Z 63 
2019-07-14T10:19:26.7373460Z 63 
2019-07-14T10:19:26.7373541Z 64 error[E0597]: `ap1` does not live long enough
2019-07-14T10:19:26.7373804Z -   --> $DIR/variadic-ffi-4.rs:24:11
2019-07-14T10:19:26.7374057Z +   --> $DIR/variadic-ffi-4.rs:25:11
2019-07-14T10:19:26.7374284Z 66    |
2019-07-14T10:19:26.7374359Z 67 LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7374684Z 
2019-07-14T10:19:26.7374684Z 
2019-07-14T10:19:26.7374878Z 76    | - `ap1` dropped here while still borrowed
2019-07-14T10:19:26.7374948Z 77 
2019-07-14T10:19:26.7374997Z 78 error: lifetime may not live long enough
2019-07-14T10:19:26.7375200Z -   --> $DIR/variadic-ffi-4.rs:32:5
2019-07-14T10:19:26.7375384Z +   --> $DIR/variadic-ffi-4.rs:33:12
2019-07-14T10:19:26.7375462Z 80    |
2019-07-14T10:19:26.7375529Z 81 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7375818Z +    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7375894Z +    |                                               |
2019-07-14T10:19:26.7376146Z +    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7376229Z + LL |     *ap0 = ap1.clone();
2019-07-14T10:19:26.7376450Z +    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7376527Z + 
2019-07-14T10:19:26.7376574Z + error: lifetime may not live long enough
2019-07-14T10:19:26.7376782Z +   --> $DIR/variadic-ffi-4.rs:33:12
2019-07-14T10:19:26.7376837Z +    |
2019-07-14T10:19:26.7376912Z + LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7377282Z 82    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7377378Z 83    |                                               |
2019-07-14T10:19:26.7377622Z 84    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7377685Z 
2019-07-14T10:19:26.7377732Z 85 LL |     *ap0 = ap1.clone();
2019-07-14T10:19:26.7377961Z -    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7378186Z +    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7378443Z - error: aborting due to 9 previous errors
2019-07-14T10:19:26.7378516Z + error: aborting due to 11 previous errors
2019-07-14T10:19:26.7378568Z 89 
2019-07-14T10:19:26.7378637Z 90 Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T10:19:26.7378637Z 90 Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T10:19:26.7378893Z 91 For more information about an error, try `rustc --explain E0384`.
2019-07-14T10:19:26.7378951Z 
2019-07-14T10:19:26.7378997Z 
2019-07-14T10:19:26.7379051Z The actual stderr differed from the expected stderr.
2019-07-14T10:19:26.7379364Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll/variadic-ffi-4.nll.stderr
2019-07-14T10:19:26.7379611Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T10:19:26.7379881Z To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`
2019-07-14T10:19:26.7379994Z error: 1 errors occurred comparing output.
2019-07-14T10:19:26.7380050Z status: exit code: 1
2019-07-14T10:19:26.7380050Z status: exit code: 1
2019-07-14T10:19:26.7381329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll/auxiliary" "-A" "unused"
2019-07-14T10:19:26.7381887Z ------------------------------------------
2019-07-14T10:19:26.7381950Z 
2019-07-14T10:19:26.7382187Z ------------------------------------------
2019-07-14T10:19:26.7382271Z stderr:
2019-07-14T10:19:26.7382271Z stderr:
2019-07-14T10:19:26.7382497Z ------------------------------------------
2019-07-14T10:19:26.7382591Z error[E0621]: explicit lifetime required in the type of `ap`
2019-07-14T10:19:26.7382861Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
2019-07-14T10:19:26.7382954Z    |
2019-07-14T10:19:26.7383233Z LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
2019-07-14T10:19:26.7383627Z    |                                                       --- help: add explicit lifetime `'f` to the type of `ap`: `core::ffi::VaListImpl<'f>`
2019-07-14T10:19:26.7383740Z LL |     ap //~ ERROR: explicit lifetime required
2019-07-14T10:19:26.7384003Z    |     ^^ lifetime `'f` required
2019-07-14T10:19:26.7384048Z 
2019-07-14T10:19:26.7384325Z error[E0621]: explicit lifetime required in the type of `ap`
2019-07-14T10:19:26.7384703Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:12:5
2019-07-14T10:19:26.7384777Z    |
2019-07-14T10:19:26.7384996Z LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
2019-07-14T10:19:26.7385314Z    |                                                   --- help: add explicit lifetime `'static` to the type of `ap`: `core::ffi::VaListImpl<'static>`
2019-07-14T10:19:26.7385397Z LL |     ap //~ ERROR: explicit lifetime required
2019-07-14T10:19:26.7385601Z    |     ^^ lifetime `'static` required
2019-07-14T10:19:26.7385636Z 
2019-07-14T10:19:26.7385699Z error: lifetime may not live long enough
2019-07-14T10:19:26.7386008Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:16:33
2019-07-14T10:19:26.7386083Z    |
2019-07-14T10:19:26.7386141Z LL |     let _ = ap.with_copy(|ap| { ap }); //~ ERROR: cannot infer an appropriate lifetime
2019-07-14T10:19:26.7386411Z    |                           ---   ^^ returning this value requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7386480Z    |                           | |
2019-07-14T10:19:26.7386720Z    |                           | return type of closure is core::ffi::VaList<'2, '_>
2019-07-14T10:19:26.7389956Z    |                           has type `core::ffi::VaList<'1, '_>`
2019-07-14T10:19:26.7390051Z 
2019-07-14T10:19:26.7390104Z error: lifetime may not live long enough
2019-07-14T10:19:26.7391060Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:20:5
2019-07-14T10:19:26.7391145Z    |
2019-07-14T10:19:26.7391241Z LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7393265Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7393587Z    |                                               |
2019-07-14T10:19:26.7393975Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7394088Z LL |     *ap0 = ap1; //~ ERROR: mismatched types
2019-07-14T10:19:26.7394685Z    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7394731Z 
2019-07-14T10:19:26.7394781Z error: lifetime may not live long enough
2019-07-14T10:19:26.7395014Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:20:5
2019-07-14T10:19:26.7395079Z    |
2019-07-14T10:19:26.7395162Z LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7395618Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7395842Z    |                                               |
2019-07-14T10:19:26.7396114Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7396201Z LL |     *ap0 = ap1; //~ ERROR: mismatched types
2019-07-14T10:19:26.7396427Z    |     ^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7396468Z 
2019-07-14T10:19:26.7396516Z error: lifetime may not live long enough
2019-07-14T10:19:26.7396741Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7396798Z    |
2019-07-14T10:19:26.7396874Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7397142Z    |                                               ---                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7397234Z    |                                               |
2019-07-14T10:19:26.7397663Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7397746Z LL |     ap0 = &mut ap1;
2019-07-14T10:19:26.7397989Z    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7398034Z 
2019-07-14T10:19:26.7398083Z error: lifetime may not live long enough
2019-07-14T10:19:26.7398313Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7398372Z    |
2019-07-14T10:19:26.7398448Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7398720Z    |                                               ---                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7398814Z    |                                               |
2019-07-14T10:19:26.7399072Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7399140Z LL |     ap0 = &mut ap1;
2019-07-14T10:19:26.7399375Z    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7399517Z 
2019-07-14T10:19:26.7399578Z error[E0384]: cannot assign to immutable argument `ap0`
2019-07-14T10:19:26.7399837Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:5
2019-07-14T10:19:26.7399900Z    |
2019-07-14T10:19:26.7399975Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7400247Z    |                                               --- help: make this binding mutable: `mut ap0`
2019-07-14T10:19:26.7400316Z LL |     ap0 = &mut ap1;
2019-07-14T10:19:26.7400390Z    |     ^^^^^^^^^^^^^^ cannot assign to immutable argument
2019-07-14T10:19:26.7400429Z 
2019-07-14T10:19:26.7400480Z error[E0597]: `ap1` does not live long enough
2019-07-14T10:19:26.7401176Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:25:11
2019-07-14T10:19:26.7401253Z    |
2019-07-14T10:19:26.7401343Z LL | pub unsafe extern "C" fn no_escape4(_: usize, ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7401676Z    |                                                    - let's call the lifetime of this reference `'1`
2019-07-14T10:19:26.7401787Z LL |     ap0 = &mut ap1;
2019-07-14T10:19:26.7402006Z    |     ------^^^^^^^^
2019-07-14T10:19:26.7402091Z    |     |     |
2019-07-14T10:19:26.7402161Z    |     |     borrowed value does not live long enough
2019-07-14T10:19:26.7402442Z    |     assignment requires that `ap1` is borrowed for `'1`
2019-07-14T10:19:26.7402591Z LL | }
2019-07-14T10:19:26.7402591Z LL | }
2019-07-14T10:19:26.7402850Z    | - `ap1` dropped here while still borrowed
2019-07-14T10:19:26.7402899Z 
2019-07-14T10:19:26.7402959Z error: lifetime may not live long enough
2019-07-14T10:19:26.7403240Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:33:12
2019-07-14T10:19:26.7403314Z    |
2019-07-14T10:19:26.7403405Z LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7403743Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7403971Z    |                                               |
2019-07-14T10:19:26.7404438Z    |                                               has type `&mut core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7404694Z LL |     *ap0 = ap1.clone(); //~ ERROR: mismatched types
2019-07-14T10:19:26.7405092Z    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7405135Z 
2019-07-14T10:19:26.7405182Z error: lifetime may not live long enough
2019-07-14T10:19:26.7405406Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:33:12
2019-07-14T10:19:26.7405465Z    |
2019-07-14T10:19:26.7405540Z LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
2019-07-14T10:19:26.7405810Z    |                                               -------                   ------- has type `core::ffi::VaListImpl<'1>`
2019-07-14T10:19:26.7405906Z    |                                               |
2019-07-14T10:19:26.7406174Z    |                                               has type `&mut core::ffi::VaListImpl<'2>`
2019-07-14T10:19:26.7406246Z LL |     *ap0 = ap1.clone(); //~ ERROR: mismatched types
2019-07-14T10:19:26.7406484Z    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
2019-07-14T10:19:26.7406575Z error: aborting due to 11 previous errors
2019-07-14T10:19:26.7406624Z 
2019-07-14T10:19:26.7406677Z Some errors have detailed explanations: E0384, E0597, E0621.
2019-07-14T10:19:26.7406928Z For more information about an error, try `rustc --explain E0384`.
---
2019-07-14T10:19:26.7409379Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-14T10:19:26.7409485Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-14T10:19:26.7409531Z 
2019-07-14T10:19:26.7409559Z 
2019-07-14T10:19:26.7411588Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-07-14T10:19:26.7412255Z 
2019-07-14T10:19:26.7412293Z 
2019-07-14T10:19:26.7412379Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-14T10:19:26.7412457Z Build completed unsuccessfully in 1:25:00
2019-07-14T10:19:26.7412457Z Build completed unsuccessfully in 1:25:00
2019-07-14T10:19:27.9035962Z ##[error]Bash exited with code '1'.
2019-07-14T10:19:27.9083754Z ##[section]Starting: Upload CPU usage statistics
2019-07-14T10:19:27.9087634Z ==============================================================================
2019-07-14T10:19:27.9087722Z Task         : Bash
2019-07-14T10:19:27.9087773Z Description  : Run a Bash script on macOS, Linux, or Windows
