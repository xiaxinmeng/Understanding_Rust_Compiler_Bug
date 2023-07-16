plain
2020-02-28T13:42:32.4949036Z ========================== Starting Command Output ===========================
2020-02-28T13:42:32.4952272Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ddfaa812-b26e-4f84-8cc2-ef14ac6a524c.sh
2020-02-28T13:42:32.4952527Z 
2020-02-28T13:42:32.4957103Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-28T13:42:32.4973985Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69550/merge to s
2020-02-28T13:42:32.4976685Z Task         : Get sources
2020-02-28T13:42:32.4976943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T13:42:32.4977207Z Version      : 1.0.0
2020-02-28T13:42:32.4977370Z Author       : Microsoft
---
2020-02-28T13:42:33.4926685Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-28T13:42:33.4935994Z ##[command]git config gc.auto 0
2020-02-28T13:42:33.4942259Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-28T13:42:33.4947562Z ##[command]git config --get-all http.proxy
2020-02-28T13:42:33.4952992Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69550/merge:refs/remotes/pull/69550/merge
---
2020-02-28T14:39:55.9466820Z .................................................................................................... 1700/9735
2020-02-28T14:40:00.0546151Z .................................................................................................... 1800/9735
2020-02-28T14:40:10.4893027Z ......................................................................i............................. 1900/9735
2020-02-28T14:40:16.4291036Z .................................................................................................... 2000/9735
2020-02-28T14:40:30.3592971Z ............................................................iiiii................................... 2100/9735
2020-02-28T14:40:39.9325512Z .................................................................................................... 2300/9735
2020-02-28T14:40:42.1000866Z .................................................................................................... 2400/9735
2020-02-28T14:40:44.9388810Z .................................................................................................... 2500/9735
2020-02-28T14:41:03.1735899Z .................................................................................................... 2600/9735
---
2020-02-28T14:43:30.6034952Z .....................i...............i.............................................................. 5000/9735
2020-02-28T14:43:39.4670904Z .................................................................................................... 5100/9735
2020-02-28T14:43:44.4842722Z ................................................................i................................... 5200/9735
2020-02-28T14:43:50.2326890Z .................................................................................................... 5300/9735
2020-02-28T14:43:58.3770644Z .........................................ii.ii........i...i......................................... 5400/9735
2020-02-28T14:44:06.3287778Z .................................................................................................... 5600/9735
2020-02-28T14:44:15.0385383Z .................................................................................................... 5700/9735
2020-02-28T14:44:21.2715855Z ................................i................................................................... 5800/9735
2020-02-28T14:44:26.7181616Z .................................................................................................... 5900/9735
2020-02-28T14:44:26.7181616Z .................................................................................................... 5900/9735
2020-02-28T14:44:36.8199276Z .................................................................................................... 6000/9735
2020-02-28T14:44:45.6062121Z .......................ii...i..ii...........i....................................................... 6100/9735
2020-02-28T14:45:00.4741479Z .................................................................................................... 6300/9735
2020-02-28T14:45:03.9315037Z .................................................................................................... 6400/9735
2020-02-28T14:45:03.9315037Z .................................................................................................... 6400/9735
2020-02-28T14:45:10.2625287Z ......................................................i..ii......................................... 6500/9735
2020-02-28T14:45:34.6872681Z .................................................................................................... 6700/9735
2020-02-28T14:45:36.9100914Z ..............................................i..................................................... 6800/9735
2020-02-28T14:45:38.9111967Z .................................................................................................... 6900/9735
2020-02-28T14:45:40.9603219Z ............................................................................i....................... 7000/9735
---
2020-02-28T14:47:10.6985671Z .................................................................................................... 7700/9735
2020-02-28T14:47:15.0701775Z .................................................................................................... 7800/9735
2020-02-28T14:47:19.9647329Z .................................................................................................... 7900/9735
2020-02-28T14:47:27.2064332Z .....................i.............................................................................. 8000/9735
2020-02-28T14:47:35.0134402Z ......................................................................iiiiiii.i..................... 8100/9735
2020-02-28T14:47:49.1891990Z ...........i......i................................................................................. 8300/9735
2020-02-28T14:47:54.1400581Z .................................................................................................... 8400/9735
2020-02-28T14:48:06.2206744Z .................................................................................................... 8500/9735
2020-02-28T14:48:14.9565192Z .................................................................................................... 8600/9735
---
2020-02-28T14:50:03.9645957Z diff of stderr:
2020-02-28T14:50:03.9646199Z 
2020-02-28T14:50:03.9646658Z 2   --> $DIR/const-err4.rs:8:11
2020-02-28T14:50:03.9646952Z 3    |
2020-02-28T14:50:03.9647294Z 4 LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
2020-02-28T14:50:03.9648170Z -    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9649223Z +    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9650807Z 6    |
2020-02-28T14:50:03.9652070Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9652955Z 
2020-02-28T14:50:03.9653168Z 
2020-02-28T14:50:03.9653784Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9654628Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/const-err4.stderr
2020-02-28T14:50:03.9654628Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/const-err4.stderr
2020-02-28T14:50:03.9655416Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9656131Z To only update this specific test, also pass `--test-args consts/const-err4.rs`
2020-02-28T14:50:03.9656814Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9657166Z status: exit code: 1
2020-02-28T14:50:03.9657166Z status: exit code: 1
2020-02-28T14:50:03.9659195Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/auxiliary"
2020-02-28T14:50:03.9661019Z ------------------------------------------
2020-02-28T14:50:03.9661317Z 
2020-02-28T14:50:03.9661802Z ------------------------------------------
2020-02-28T14:50:03.9662144Z stderr:
2020-02-28T14:50:03.9662144Z stderr:
2020-02-28T14:50:03.9662637Z ------------------------------------------
2020-02-28T14:50:03.9663062Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9663918Z   --> /checkout/src/test/ui/consts/const-err4.rs:8:11
2020-02-28T14:50:03.9664306Z    |
2020-02-28T14:50:03.9664698Z LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
2020-02-28T14:50:03.9668727Z    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9669270Z    |
2020-02-28T14:50:03.9670076Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9670713Z error: aborting due to previous error
2020-02-28T14:50:03.9670870Z 
2020-02-28T14:50:03.9671278Z For more information about this error, try `rustc --explain E0080`.
2020-02-28T14:50:03.9671480Z 
---
2020-02-28T14:50:03.9672901Z diff of stderr:
2020-02-28T14:50:03.9673017Z 
2020-02-28T14:50:03.9673414Z 2   --> $DIR/const-pointer-values-in-various-types.rs:25:5
2020-02-28T14:50:03.9673663Z 3    |
2020-02-28T14:50:03.9673965Z 4 LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
2020-02-28T14:50:03.9674876Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9675929Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9676367Z 6    |
2020-02-28T14:50:03.9677161Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9677731Z 
2020-02-28T14:50:03.9678144Z 36   --> $DIR/const-pointer-values-in-various-types.rs:37:5
2020-02-28T14:50:03.9678381Z 37    |
2020-02-28T14:50:03.9678381Z 37    |
2020-02-28T14:50:03.9678690Z 38 LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
2020-02-28T14:50:03.9679614Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9680659Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9681102Z 40    |
2020-02-28T14:50:03.9681909Z 41    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9682482Z 
2020-02-28T14:50:03.9682895Z 44   --> $DIR/const-pointer-values-in-various-types.rs:40:5
2020-02-28T14:50:03.9683129Z 45    |
2020-02-28T14:50:03.9683129Z 45    |
2020-02-28T14:50:03.9683438Z 46 LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
2020-02-28T14:50:03.9684629Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9685727Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9686188Z 48    |
2020-02-28T14:50:03.9686989Z 49    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9687558Z 
2020-02-28T14:50:03.9687967Z 76   --> $DIR/const-pointer-values-in-various-types.rs:52:5
2020-02-28T14:50:03.9688200Z 77    |
2020-02-28T14:50:03.9688200Z 77    |
2020-02-28T14:50:03.9688668Z 78 LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
2020-02-28T14:50:03.9689997Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9691129Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9691605Z 80    |
2020-02-28T14:50:03.9692464Z 81    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9693156Z 
2020-02-28T14:50:03.9693609Z 84   --> $DIR/const-pointer-values-in-various-types.rs:55:5
2020-02-28T14:50:03.9693861Z 85    |
2020-02-28T14:50:03.9693861Z 85    |
2020-02-28T14:50:03.9694198Z 86 LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
2020-02-28T14:50:03.9695711Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9697076Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9697733Z 88    |
2020-02-28T14:50:03.9698862Z 89    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9699615Z 
2020-02-28T14:50:03.9700789Z 100   --> $DIR/const-pointer-values-in-various-types.rs:61:5
2020-02-28T14:50:03.9701062Z 101    |
2020-02-28T14:50:03.9701062Z 101    |
2020-02-28T14:50:03.9701401Z 102 LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
2020-02-28T14:50:03.9702530Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9703571Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9704033Z 104    |
2020-02-28T14:50:03.9704826Z 105    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9705402Z 
2020-02-28T14:50:03.9705819Z 148   --> $DIR/const-pointer-values-in-various-types.rs:79:5
2020-02-28T14:50:03.9706056Z 149    |
2020-02-28T14:50:03.9706056Z 149    |
2020-02-28T14:50:03.9706354Z 150 LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
2020-02-28T14:50:03.9707251Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9708268Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9708711Z 152    |
2020-02-28T14:50:03.9709496Z 153    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9710087Z 
2020-02-28T14:50:03.9710486Z 188   --> $DIR/const-pointer-values-in-various-types.rs:94:5
2020-02-28T14:50:03.9710724Z 189    |
2020-02-28T14:50:03.9710724Z 189    |
2020-02-28T14:50:03.9711093Z 190 LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
2020-02-28T14:50:03.9711998Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9714155Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9714616Z 192    |
2020-02-28T14:50:03.9715575Z 193    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9716264Z 
2020-02-28T14:50:03.9716868Z 212   --> $DIR/const-pointer-values-in-various-types.rs:103:5
2020-02-28T14:50:03.9717127Z 213    |
2020-02-28T14:50:03.9717127Z 213    |
2020-02-28T14:50:03.9717467Z 214 LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
2020-02-28T14:50:03.9718424Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9719518Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9720003Z 216    |
2020-02-28T14:50:03.9720854Z 217    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9721488Z 
2020-02-28T14:50:03.9721586Z 
2020-02-28T14:50:03.9721794Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9722625Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.stderr
2020-02-28T14:50:03.9722625Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.stderr
2020-02-28T14:50:03.9723361Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9724030Z To only update this specific test, also pass `--test-args consts/const-eval/const-pointer-values-in-various-types.rs`
2020-02-28T14:50:03.9724542Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9724780Z status: exit code: 1
2020-02-28T14:50:03.9724780Z status: exit code: 1
2020-02-28T14:50:03.9726994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/auxiliary"
2020-02-28T14:50:03.9728873Z ------------------------------------------
2020-02-28T14:50:03.9729036Z 
2020-02-28T14:50:03.9729474Z ------------------------------------------
2020-02-28T14:50:03.9729666Z stderr:
2020-02-28T14:50:03.9729666Z stderr:
2020-02-28T14:50:03.9730029Z ------------------------------------------
2020-02-28T14:50:03.9730301Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9730878Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:25:5
2020-02-28T14:50:03.9731182Z    |
2020-02-28T14:50:03.9731472Z LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
2020-02-28T14:50:03.9732413Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9732846Z    |
2020-02-28T14:50:03.9733630Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9734278Z error: any use of this value will cause an error
2020-02-28T14:50:03.9734836Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:28:43
2020-02-28T14:50:03.9735135Z    |
2020-02-28T14:50:03.9735135Z    |
2020-02-28T14:50:03.9735423Z LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
2020-02-28T14:50:03.9736428Z    |                                           |
2020-02-28T14:50:03.9736428Z    |                                           |
2020-02-28T14:50:03.9736810Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9737324Z    = note: `#[deny(const_err)]` on by default
2020-02-28T14:50:03.9737494Z 
2020-02-28T14:50:03.9737683Z error: any use of this value will cause an error
2020-02-28T14:50:03.9738254Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:31:45
2020-02-28T14:50:03.9738254Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:31:45
2020-02-28T14:50:03.9738534Z    |
2020-02-28T14:50:03.9738821Z LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
2020-02-28T14:50:03.9739776Z    |                                             |
2020-02-28T14:50:03.9739776Z    |                                             |
2020-02-28T14:50:03.9740176Z    |                                             a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9740648Z error: any use of this value will cause an error
2020-02-28T14:50:03.9741205Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:34:45
2020-02-28T14:50:03.9741503Z    |
2020-02-28T14:50:03.9741503Z    |
2020-02-28T14:50:03.9741790Z LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
2020-02-28T14:50:03.9742743Z    |                                             |
2020-02-28T14:50:03.9742743Z    |                                             |
2020-02-28T14:50:03.9743125Z    |                                             a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9743630Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9744204Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:37:5
2020-02-28T14:50:03.9744482Z    |
2020-02-28T14:50:03.9744482Z    |
2020-02-28T14:50:03.9744785Z LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
2020-02-28T14:50:03.9745652Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9746085Z    |
2020-02-28T14:50:03.9746874Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9747527Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9748111Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:40:5
2020-02-28T14:50:03.9748396Z    |
2020-02-28T14:50:03.9748396Z    |
2020-02-28T14:50:03.9748691Z LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
2020-02-28T14:50:03.9749607Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9750107Z    |
2020-02-28T14:50:03.9750901Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9751536Z error: any use of this value will cause an error
2020-02-28T14:50:03.9752105Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:43:43
2020-02-28T14:50:03.9752386Z    |
2020-02-28T14:50:03.9752386Z    |
2020-02-28T14:50:03.9752669Z LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
2020-02-28T14:50:03.9753657Z    |                                           |
2020-02-28T14:50:03.9753657Z    |                                           |
2020-02-28T14:50:03.9754031Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9754519Z error: any use of this value will cause an error
2020-02-28T14:50:03.9755076Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:46:45
2020-02-28T14:50:03.9755357Z    |
2020-02-28T14:50:03.9755357Z    |
2020-02-28T14:50:03.9755663Z LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
2020-02-28T14:50:03.9756597Z    |                                             |
2020-02-28T14:50:03.9756597Z    |                                             |
2020-02-28T14:50:03.9756992Z    |                                             a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9757465Z error: any use of this value will cause an error
2020-02-28T14:50:03.9758031Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:49:45
2020-02-28T14:50:03.9758313Z    |
2020-02-28T14:50:03.9758313Z    |
2020-02-28T14:50:03.9758604Z LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
2020-02-28T14:50:03.9759547Z    |                                             |
2020-02-28T14:50:03.9759547Z    |                                             |
2020-02-28T14:50:03.9759927Z    |                                             a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9760430Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9760998Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:52:5
2020-02-28T14:50:03.9761289Z    |
2020-02-28T14:50:03.9761289Z    |
2020-02-28T14:50:03.9761580Z LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
2020-02-28T14:50:03.9762436Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9762880Z    |
2020-02-28T14:50:03.9763652Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9764314Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9764881Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:55:5
2020-02-28T14:50:03.9765161Z    |
2020-02-28T14:50:03.9765161Z    |
2020-02-28T14:50:03.9765470Z LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
2020-02-28T14:50:03.9766366Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9766821Z    |
2020-02-28T14:50:03.9767650Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9768284Z error: any use of this value will cause an error
2020-02-28T14:50:03.9768865Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:58:45
2020-02-28T14:50:03.9769147Z    |
2020-02-28T14:50:03.9769147Z    |
2020-02-28T14:50:03.9769591Z LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
2020-02-28T14:50:03.9770556Z    |                                             |
2020-02-28T14:50:03.9770556Z    |                                             |
2020-02-28T14:50:03.9771012Z    |                                             a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9771515Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9772098Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:61:5
2020-02-28T14:50:03.9772378Z    |
2020-02-28T14:50:03.9772378Z    |
2020-02-28T14:50:03.9772684Z LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
2020-02-28T14:50:03.9773553Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9774004Z    |
2020-02-28T14:50:03.9774768Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9775400Z error: any use of this value will cause an error
2020-02-28T14:50:03.9775971Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:64:47
2020-02-28T14:50:03.9776252Z    |
2020-02-28T14:50:03.9776252Z    |
2020-02-28T14:50:03.9776558Z LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
2020-02-28T14:50:03.9777562Z    |                                               |
2020-02-28T14:50:03.9777562Z    |                                               |
2020-02-28T14:50:03.9777965Z    |                                               a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9778639Z error: any use of this value will cause an error
2020-02-28T14:50:03.9779205Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:67:47
2020-02-28T14:50:03.9779502Z    |
2020-02-28T14:50:03.9779502Z    |
2020-02-28T14:50:03.9779804Z LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
2020-02-28T14:50:03.9780781Z    |                                               |
2020-02-28T14:50:03.9780781Z    |                                               |
2020-02-28T14:50:03.9781170Z    |                                               a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9781659Z error: any use of this value will cause an error
2020-02-28T14:50:03.9782214Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:70:39
2020-02-28T14:50:03.9782494Z    |
2020-02-28T14:50:03.9782494Z    |
2020-02-28T14:50:03.9782782Z LL |     const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };
2020-02-28T14:50:03.9783655Z    |                                       |
2020-02-28T14:50:03.9783655Z    |                                       |
2020-02-28T14:50:03.9784035Z    |                                       a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9784499Z error: any use of this value will cause an error
2020-02-28T14:50:03.9785254Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:73:41
2020-02-28T14:50:03.9785541Z    |
2020-02-28T14:50:03.9785541Z    |
2020-02-28T14:50:03.9785818Z LL |     const STR_U16_UNION: u16 = unsafe { Nonsense { stringy: "3" }.uint_16 };
2020-02-28T14:50:03.9786750Z    |                                         |
2020-02-28T14:50:03.9786750Z    |                                         |
2020-02-28T14:50:03.9787118Z    |                                         a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9787594Z error: any use of this value will cause an error
2020-02-28T14:50:03.9788149Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:76:41
2020-02-28T14:50:03.9788477Z    |
2020-02-28T14:50:03.9788477Z    |
2020-02-28T14:50:03.9788768Z LL |     const STR_U32_UNION: u32 = unsafe { Nonsense { stringy: "3" }.uint_32 };
2020-02-28T14:50:03.9789676Z    |                                         |
2020-02-28T14:50:03.9789676Z    |                                         |
2020-02-28T14:50:03.9790059Z    |                                         a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9790539Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9791121Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:79:5
2020-02-28T14:50:03.9791400Z    |
2020-02-28T14:50:03.9791400Z    |
2020-02-28T14:50:03.9791674Z LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
2020-02-28T14:50:03.9792523Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9792946Z    |
2020-02-28T14:50:03.9793716Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9794364Z error: any use of this value will cause an error
2020-02-28T14:50:03.9794919Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:82:43
2020-02-28T14:50:03.9795216Z    |
2020-02-28T14:50:03.9795216Z    |
2020-02-28T14:50:03.9795499Z LL |     const STR_U128_UNION: u128 = unsafe { Nonsense { stringy: "3" }.uint_128 };
2020-02-28T14:50:03.9796431Z    |                                           |
2020-02-28T14:50:03.9796431Z    |                                           |
2020-02-28T14:50:03.9796806Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9797293Z error: any use of this value will cause an error
2020-02-28T14:50:03.9797848Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:85:39
2020-02-28T14:50:03.9798129Z    |
2020-02-28T14:50:03.9798129Z    |
2020-02-28T14:50:03.9798418Z LL |     const STR_I8_UNION: i8 = unsafe { Nonsense { stringy: "3" }.int_8 };
2020-02-28T14:50:03.9799289Z    |                                       |
2020-02-28T14:50:03.9799289Z    |                                       |
2020-02-28T14:50:03.9799666Z    |                                       a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9800130Z error: any use of this value will cause an error
2020-02-28T14:50:03.9800685Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:88:41
2020-02-28T14:50:03.9800984Z    |
2020-02-28T14:50:03.9800984Z    |
2020-02-28T14:50:03.9801260Z LL |     const STR_I16_UNION: i16 = unsafe { Nonsense { stringy: "3" }.int_16 };
2020-02-28T14:50:03.9802161Z    |                                         |
2020-02-28T14:50:03.9802161Z    |                                         |
2020-02-28T14:50:03.9802595Z    |                                         a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9803079Z error: any use of this value will cause an error
2020-02-28T14:50:03.9803643Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:91:41
2020-02-28T14:50:03.9803924Z    |
2020-02-28T14:50:03.9803924Z    |
2020-02-28T14:50:03.9804215Z LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
2020-02-28T14:50:03.9805105Z    |                                         |
2020-02-28T14:50:03.9805105Z    |                                         |
2020-02-28T14:50:03.9805534Z    |                                         a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9806518Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9807206Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:94:5
2020-02-28T14:50:03.9807508Z    |
2020-02-28T14:50:03.9807508Z    |
2020-02-28T14:50:03.9807804Z LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
2020-02-28T14:50:03.9808714Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9809170Z    |
2020-02-28T14:50:03.9810240Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9810953Z error: any use of this value will cause an error
2020-02-28T14:50:03.9811554Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:97:43
2020-02-28T14:50:03.9811872Z    |
2020-02-28T14:50:03.9811872Z    |
2020-02-28T14:50:03.9812177Z LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
2020-02-28T14:50:03.9813183Z    |                                           |
2020-02-28T14:50:03.9813183Z    |                                           |
2020-02-28T14:50:03.9813588Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9814093Z error: any use of this value will cause an error
2020-02-28T14:50:03.9814713Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:100:41
2020-02-28T14:50:03.9815017Z    |
2020-02-28T14:50:03.9815017Z    |
2020-02-28T14:50:03.9815315Z LL |     const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };
2020-02-28T14:50:03.9816309Z    |                                         |
2020-02-28T14:50:03.9816309Z    |                                         |
2020-02-28T14:50:03.9816710Z    |                                         a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9817244Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9817863Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:103:5
2020-02-28T14:50:03.9818181Z    |
2020-02-28T14:50:03.9818181Z    |
2020-02-28T14:50:03.9818477Z LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
2020-02-28T14:50:03.9819380Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9819960Z    |
2020-02-28T14:50:03.9820736Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9821414Z error: any use of this value will cause an error
2020-02-28T14:50:03.9822048Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:106:43
2020-02-28T14:50:03.9822333Z    |
2020-02-28T14:50:03.9822333Z    |
2020-02-28T14:50:03.9822641Z LL |     const STR_BOOL_UNION: bool = unsafe { Nonsense { stringy: "3" }.truthy_falsey };
2020-02-28T14:50:03.9823597Z    |                                           |
2020-02-28T14:50:03.9823597Z    |                                           |
2020-02-28T14:50:03.9823985Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9824455Z error: any use of this value will cause an error
2020-02-28T14:50:03.9825089Z   --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:109:43
2020-02-28T14:50:03.9825375Z    |
2020-02-28T14:50:03.9825375Z    |
2020-02-28T14:50:03.9825657Z LL |     const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: "3" }.character };
2020-02-28T14:50:03.9826815Z    |                                           |
2020-02-28T14:50:03.9826815Z    |                                           |
2020-02-28T14:50:03.9827223Z    |                                           a raw memory access tried to access part of a pointer value as raw bytes
2020-02-28T14:50:03.9827732Z error: aborting due to 29 previous errors
2020-02-28T14:50:03.9827905Z 
2020-02-28T14:50:03.9828333Z For more information about this error, try `rustc --explain E0080`.
2020-02-28T14:50:03.9828549Z 
---
2020-02-28T14:50:03.9829870Z diff of stderr:
2020-02-28T14:50:03.9829995Z 
2020-02-28T14:50:03.9830354Z 2   --> $DIR/ref_to_int_match.rs:25:1
2020-02-28T14:50:03.9830572Z 3    |
2020-02-28T14:50:03.9830818Z 4 LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
2020-02-28T14:50:03.9831603Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9832550Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9832951Z 6    |
2020-02-28T14:50:03.9833787Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9834414Z 
2020-02-28T14:50:03.9834512Z 
2020-02-28T14:50:03.9834720Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9835432Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.stderr
2020-02-28T14:50:03.9835432Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.stderr
2020-02-28T14:50:03.9836091Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9836717Z To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`
2020-02-28T14:50:03.9837175Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9837414Z status: exit code: 1
2020-02-28T14:50:03.9837414Z status: exit code: 1
2020-02-28T14:50:03.9839443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
2020-02-28T14:50:03.9841160Z ------------------------------------------
2020-02-28T14:50:03.9841335Z 
2020-02-28T14:50:03.9841705Z ------------------------------------------
2020-02-28T14:50:03.9841908Z stderr:
2020-02-28T14:50:03.9841908Z stderr:
2020-02-28T14:50:03.9842273Z ------------------------------------------
2020-02-28T14:50:03.9842582Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9843135Z   --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:25:1
2020-02-28T14:50:03.9843394Z    |
2020-02-28T14:50:03.9843737Z LL | const BAR: Int = unsafe { Foo { r: &42 }.f }; //~ ERROR it is undefined behavior to use this value
2020-02-28T14:50:03.9844638Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9845030Z    |
2020-02-28T14:50:03.9845879Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9846547Z error: could not evaluate constant pattern
2020-02-28T14:50:03.9847084Z   --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
2020-02-28T14:50:03.9847342Z    |
2020-02-28T14:50:03.9847342Z    |
2020-02-28T14:50:03.9847610Z LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern
2020-02-28T14:50:03.9848032Z 
2020-02-28T14:50:03.9848222Z error: could not evaluate constant pattern
2020-02-28T14:50:03.9848756Z   --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
2020-02-28T14:50:03.9849014Z    |
2020-02-28T14:50:03.9849014Z    |
2020-02-28T14:50:03.9849265Z LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern
2020-02-28T14:50:03.9849807Z 
2020-02-28T14:50:03.9849991Z error: aborting due to 3 previous errors
2020-02-28T14:50:03.9850160Z 
2020-02-28T14:50:03.9850616Z For more information about this error, try `rustc --explain E0080`.
---
2020-02-28T14:50:03.9852099Z diff of stderr:
2020-02-28T14:50:03.9852223Z 
2020-02-28T14:50:03.9852574Z 18   --> $DIR/ub-ref.rs:17:1
2020-02-28T14:50:03.9852759Z 19    |
2020-02-28T14:50:03.9853027Z 20 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
2020-02-28T14:50:03.9853897Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9854902Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9855426Z 22    |
2020-02-28T14:50:03.9856291Z 23    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9856869Z 
2020-02-28T14:50:03.9856959Z 
2020-02-28T14:50:03.9857150Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9857750Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
2020-02-28T14:50:03.9857750Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/ub-ref.stderr
2020-02-28T14:50:03.9858341Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9858882Z To only update this specific test, also pass `--test-args consts/const-eval/ub-ref.rs`
2020-02-28T14:50:03.9859312Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9859530Z status: exit code: 1
2020-02-28T14:50:03.9859530Z status: exit code: 1
2020-02-28T14:50:03.9861394Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref/auxiliary"
2020-02-28T14:50:03.9862886Z ------------------------------------------
2020-02-28T14:50:03.9863063Z 
2020-02-28T14:50:03.9863462Z ------------------------------------------
2020-02-28T14:50:03.9863649Z stderr:
2020-02-28T14:50:03.9863649Z stderr:
2020-02-28T14:50:03.9864002Z ------------------------------------------
2020-02-28T14:50:03.9864276Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9864757Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:7:1
2020-02-28T14:50:03.9864994Z    |
2020-02-28T14:50:03.9865237Z LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
2020-02-28T14:50:03.9865766Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned reference (required 2 byte alignment but found 1)
2020-02-28T14:50:03.9866184Z    |
2020-02-28T14:50:03.9866957Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9867621Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9868109Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:11:1
2020-02-28T14:50:03.9868344Z    |
2020-02-28T14:50:03.9868344Z    |
2020-02-28T14:50:03.9868572Z LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
2020-02-28T14:50:03.9869052Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
2020-02-28T14:50:03.9869438Z    |
2020-02-28T14:50:03.9870209Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9870877Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9871358Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:17:1
2020-02-28T14:50:03.9871578Z    |
2020-02-28T14:50:03.9871578Z    |
2020-02-28T14:50:03.9871831Z LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
2020-02-28T14:50:03.9872581Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9872978Z    |
2020-02-28T14:50:03.9873749Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9874409Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9874890Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:20:1
2020-02-28T14:50:03.9875111Z    |
2020-02-28T14:50:03.9875111Z    |
2020-02-28T14:50:03.9875389Z LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
2020-02-28T14:50:03.9876219Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9876649Z    |
2020-02-28T14:50:03.9877428Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9878139Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9878634Z   --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:23:1
2020-02-28T14:50:03.9878853Z    |
2020-02-28T14:50:03.9878853Z    |
2020-02-28T14:50:03.9879314Z LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
2020-02-28T14:50:03.9879857Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (created from integer)
2020-02-28T14:50:03.9880254Z    |
2020-02-28T14:50:03.9881036Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9881701Z error: aborting due to 5 previous errors
2020-02-28T14:50:03.9881875Z 
2020-02-28T14:50:03.9882279Z For more information about this error, try `rustc --explain E0080`.
2020-02-28T14:50:03.9882480Z 
---
2020-02-28T14:50:03.9883730Z diff of stderr:
2020-02-28T14:50:03.9883846Z 
2020-02-28T14:50:03.9884194Z 2   --> $DIR/union-const-eval-field.rs:28:5
2020-02-28T14:50:03.9884395Z 3    |
2020-02-28T14:50:03.9884638Z 4 LL |     const FIELD3: Field3 = unsafe { UNION.field3 };
2020-02-28T14:50:03.9885411Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9886344Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9886758Z 6    |
2020-02-28T14:50:03.9887537Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9888117Z 
2020-02-28T14:50:03.9888207Z 
2020-02-28T14:50:03.9888397Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9889082Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/union-const-eval-field.stderr
2020-02-28T14:50:03.9889082Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/union-const-eval-field.stderr
2020-02-28T14:50:03.9889937Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9890535Z To only update this specific test, also pass `--test-args consts/const-eval/union-const-eval-field.rs`
2020-02-28T14:50:03.9891162Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9891399Z status: exit code: 1
2020-02-28T14:50:03.9891399Z status: exit code: 1
2020-02-28T14:50:03.9893496Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-const-eval-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/auxiliary"
2020-02-28T14:50:03.9895181Z ------------------------------------------
2020-02-28T14:50:03.9895355Z 
2020-02-28T14:50:03.9895728Z ------------------------------------------
2020-02-28T14:50:03.9895931Z stderr:
2020-02-28T14:50:03.9895931Z stderr:
2020-02-28T14:50:03.9896293Z ------------------------------------------
2020-02-28T14:50:03.9896586Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9897171Z   --> /checkout/src/test/ui/consts/const-eval/union-const-eval-field.rs:28:5
2020-02-28T14:50:03.9897508Z    |
2020-02-28T14:50:03.9897763Z LL |     const FIELD3: Field3 = unsafe { UNION.field3 };
2020-02-28T14:50:03.9898563Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9898984Z    |
2020-02-28T14:50:03.9899831Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9900496Z error: aborting due to previous error
2020-02-28T14:50:03.9900752Z 
2020-02-28T14:50:03.9901190Z For more information about this error, try `rustc --explain E0080`.
2020-02-28T14:50:03.9901405Z 
---
2020-02-28T14:50:03.9902778Z diff of stderr:
2020-02-28T14:50:03.9902894Z 
2020-02-28T14:50:03.9903206Z 2   --> $DIR/union-ice.rs:14:1
2020-02-28T14:50:03.9903382Z 3    |
2020-02-28T14:50:03.9903613Z 4 LL | const FIELD3: Field3 = unsafe { UNION.field3 };
2020-02-28T14:50:03.9904363Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9905269Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9905677Z 6    |
2020-02-28T14:50:03.9906453Z 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9907034Z 
2020-02-28T14:50:03.9907188Z 13 LL | |     a: 42,
2020-02-28T14:50:03.9907188Z 13 LL | |     a: 42,
2020-02-28T14:50:03.9907413Z 14 LL | |     b: unsafe { UNION.field3 },
2020-02-28T14:50:03.9907632Z 15 LL | | };
2020-02-28T14:50:03.9908207Z -    | |__^ type validation failed: encountered uninitialized bytes at .b, but expected initialized plain (non-pointer) bytes
2020-02-28T14:50:03.9908956Z +    | |__^ type validation failed: encountered uninitialized bytes at .b, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9909273Z 17    |
2020-02-28T14:50:03.9910060Z 18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9910648Z 
2020-02-28T14:50:03.9910739Z 
2020-02-28T14:50:03.9910944Z The actual stderr differed from the expected stderr.
2020-02-28T14:50:03.9911556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/union-ice.stderr
2020-02-28T14:50:03.9911556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/union-ice.stderr
2020-02-28T14:50:03.9912132Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:50:03.9912694Z To only update this specific test, also pass `--test-args consts/const-eval/union-ice.rs`
2020-02-28T14:50:03.9913107Z error: 1 errors occurred comparing output.
2020-02-28T14:50:03.9913324Z status: exit code: 1
2020-02-28T14:50:03.9913324Z status: exit code: 1
2020-02-28T14:50:03.9915215Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/auxiliary"
2020-02-28T14:50:03.9916720Z ------------------------------------------
2020-02-28T14:50:03.9916880Z 
2020-02-28T14:50:03.9917208Z ------------------------------------------
2020-02-28T14:50:03.9917411Z stderr:
2020-02-28T14:50:03.9917411Z stderr:
2020-02-28T14:50:03.9917749Z ------------------------------------------
2020-02-28T14:50:03.9918022Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9918527Z   --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:14:1
2020-02-28T14:50:03.9918754Z    |
2020-02-28T14:50:03.9919051Z LL | const FIELD3: Field3 = unsafe { UNION.field3 }; //~ ERROR it is undefined behavior to use this value
2020-02-28T14:50:03.9919917Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9920332Z    |
2020-02-28T14:50:03.9921118Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9921771Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9922276Z   --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:16:1
2020-02-28T14:50:03.9922502Z    |
2020-02-28T14:50:03.9922502Z    |
2020-02-28T14:50:03.9922781Z LL | / const FIELD_PATH: Struct = Struct { //~ ERROR it is undefined behavior to use this value
2020-02-28T14:50:03.9923100Z LL | |     a: 42,
2020-02-28T14:50:03.9923312Z LL | |     b: unsafe { UNION.field3 },
2020-02-28T14:50:03.9923506Z LL | | };
2020-02-28T14:50:03.9924052Z    | |__^ type validation failed: encountered uninitialized bytes at .b, but expected plain (non-pointer) bytes
2020-02-28T14:50:03.9924357Z    |
2020-02-28T14:50:03.9925126Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9925791Z error[E0080]: it is undefined behavior to use this value
2020-02-28T14:50:03.9926295Z   --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:26:1
2020-02-28T14:50:03.9926522Z    |
2020-02-28T14:50:03.9926522Z    |
2020-02-28T14:50:03.9926805Z LL | / const FIELD_PATH2: Struct2 = Struct2 { //~ ERROR it is undefined behavior to use this value
2020-02-28T14:50:03.9927110Z LL | |     b: [
2020-02-28T14:50:03.9927292Z LL | |         21,
2020-02-28T14:50:03.9927492Z LL | |         unsafe { UNION.field3 },
2020-02-28T14:50:03.9927839Z LL | |     a: 42,
2020-02-28T14:50:03.9927999Z LL | | };
2020-02-28T14:50:03.9927999Z LL | | };
2020-02-28T14:50:03.9928251Z    | |__^ type validation failed: encountered undefined bytes at .b[1]
2020-02-28T14:50:03.9928505Z    |
2020-02-28T14:50:03.9929280Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-28T14:50:03.9930104Z error: aborting due to 3 previous errors
2020-02-28T14:50:03.9930263Z 
2020-02-28T14:50:03.9930676Z For more information about this error, try `rustc --explain E0080`.
2020-02-28T14:50:03.9930877Z 
---
2020-02-28T14:50:03.9935905Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-28T14:50:03.9936337Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T14:50:03.9936570Z 
2020-02-28T14:50:03.9936667Z 
2020-02-28T14:50:03.9940412Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-28T14:50:03.9943095Z 
2020-02-28T14:50:03.9943196Z 
2020-02-28T14:50:03.9943449Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-28T14:50:03.9943777Z Build completed unsuccessfully in 1:00:31
2020-02-28T14:50:03.9943777Z Build completed unsuccessfully in 1:00:31
2020-02-28T14:50:03.9944026Z == clock drift check ==
2020-02-28T14:50:03.9944291Z   local time: Fri Feb 28 14:50:03 UTC 2020
2020-02-28T14:50:03.9944598Z   network time: Fri, 28 Feb 2020 14:50:03 GMT
2020-02-28T14:50:03.9944860Z == end clock drift check ==
2020-02-28T14:50:04.0317705Z 
2020-02-28T14:50:04.0379763Z ##[error]Bash exited with code '1'.
2020-02-28T14:50:04.0393492Z ##[section]Finishing: Run build
2020-02-28T14:50:04.0441064Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69550/merge to s
2020-02-28T14:50:04.0446042Z Task         : Get sources
2020-02-28T14:50:04.0446402Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T14:50:04.0446739Z Version      : 1.0.0
2020-02-28T14:50:04.0446965Z Author       : Microsoft
2020-02-28T14:50:04.0446965Z Author       : Microsoft
2020-02-28T14:50:04.0447323Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-28T14:50:04.0447746Z ==============================================================================
2020-02-28T14:50:04.3570649Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-28T14:50:04.3628472Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69550/merge to s
2020-02-28T14:50:04.3710804Z Cleaning up task key
2020-02-28T14:50:04.3711974Z Start cleaning up orphan processes.
2020-02-28T14:50:04.3863986Z Terminate orphan process: pid (3851) (python)
2020-02-28T14:50:04.4081191Z ##[section]Finishing: Finalize Job
