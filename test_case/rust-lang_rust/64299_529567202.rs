plain
2019-09-09T15:37:20.7144346Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T15:37:20.7352959Z ##[command]git config gc.auto 0
2019-09-09T15:37:20.7416755Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T15:37:20.7467698Z ##[command]git config --get-all http.proxy
2019-09-09T15:37:20.7610848Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64299/merge:refs/remotes/pull/64299/merge
---
2019-09-09T16:36:24.5385224Z .................................................................................................... 1500/9009
2019-09-09T16:36:29.9047386Z .................................................................................................... 1600/9009
2019-09-09T16:36:41.5158511Z ......................................................i...............i............................. 1700/9009
2019-09-09T16:36:48.7005277Z .................................................................................................... 1800/9009
2019-09-09T16:37:02.0165335Z .............................................iiiii.................................................. 1900/9009
2019-09-09T16:37:12.0294380Z .................................................................................................... 2100/9009
2019-09-09T16:37:14.2599421Z .................................................................................................... 2200/9009
2019-09-09T16:37:17.4965179Z .................................................................................................... 2300/9009
2019-09-09T16:37:24.8532277Z .................................................................................................... 2400/9009
---
2019-09-09T16:40:08.7790142Z ...................................i...............i................................................ 4700/9009
2019-09-09T16:40:19.4337372Z .................................................................................................... 4800/9009
2019-09-09T16:40:25.3190680Z .................................................................................................... 4900/9009
2019-09-09T16:40:35.1328601Z .................................................................................................... 5000/9009
2019-09-09T16:40:40.6831279Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T16:40:50.2743348Z .................................................................................................... 5300/9009
2019-09-09T16:40:59.5456038Z ................................................................................i................... 5400/9009
2019-09-09T16:41:06.7151121Z .................................................................................................... 5500/9009
2019-09-09T16:41:12.1704564Z .................................................................................................... 5600/9009
2019-09-09T16:41:12.1704564Z .................................................................................................... 5600/9009
2019-09-09T16:41:22.0124157Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T16:41:45.1066891Z .................................................................................................... 5900/9009
2019-09-09T16:41:54.0977504Z .................................................................................................... 6000/9009
2019-09-09T16:41:54.0977504Z .................................................................................................... 6000/9009
2019-09-09T16:41:59.4946683Z ............................................................................i..ii................... 6100/9009
2019-09-09T16:42:12.6307661Z .................................................................................................... 6200/9009
2019-09-09T16:42:26.1606452Z .................................................FF..FFF..FFFFF..................................... 6300/9009
2019-09-09T16:42:29.8877727Z .................................................................................................... 6500/9009
2019-09-09T16:42:32.1635084Z .......i............................................................................................ 6600/9009
2019-09-09T16:42:36.4876030Z ..................................................................F................................. 6700/9009
2019-09-09T16:42:54.1589827Z .................................................................................................... 6800/9009
---
2019-09-09T16:46:19.8541093Z ---- [ui] ui/packed-struct/packed-struct-transmute.rs stdout ----
2019-09-09T16:46:19.8541151Z 
2019-09-09T16:46:19.8541196Z error: ui test compiled successfully!
2019-09-09T16:46:19.8541236Z status: exit code: 0
2019-09-09T16:46:19.8541939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed-struct/packed-struct-transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed-struct/packed-struct-transmute" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed-struct/packed-struct-transmute/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8542551Z ------------------------------------------
2019-09-09T16:46:19.8542584Z 
2019-09-09T16:46:19.8542774Z ------------------------------------------
2019-09-09T16:46:19.8542816Z stderr:
---
2019-09-09T16:46:19.8543666Z ---- [ui] ui/packed-struct/packed-struct-generic-transmute.rs stdout ----
2019-09-09T16:46:19.8543697Z 
2019-09-09T16:46:19.8543737Z error: ui test compiled successfully!
2019-09-09T16:46:19.8543975Z status: exit code: 0
2019-09-09T16:46:19.8545142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed-struct/packed-struct-generic-transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed-struct/packed-struct-generic-transmute" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed-struct/packed-struct-generic-transmute/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8546266Z ------------------------------------------
2019-09-09T16:46:19.8546307Z 
2019-09-09T16:46:19.8546578Z ------------------------------------------
2019-09-09T16:46:19.8546622Z stderr:
---
2019-09-09T16:46:19.8547392Z ---- [ui] ui/packed/packed-struct-generic-layout.rs stdout ----
2019-09-09T16:46:19.8547444Z 
2019-09-09T16:46:19.8547671Z error: test compilation failed although it shouldn't!
2019-09-09T16:46:19.8547879Z status: exit code: 1
2019-09-09T16:46:19.8548690Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-struct-generic-layout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-generic-layout/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-generic-layout/auxiliary"
2019-09-09T16:46:19.8549024Z ------------------------------------------
2019-09-09T16:46:19.8549059Z 
2019-09-09T16:46:19.8549432Z ------------------------------------------
2019-09-09T16:46:19.8549488Z stderr:
2019-09-09T16:46:19.8549488Z stderr:
2019-09-09T16:46:19.8549677Z ------------------------------------------
2019-09-09T16:46:19.8550084Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2019-09-09T16:46:19.8550372Z    |
2019-09-09T16:46:19.8550372Z    |
2019-09-09T16:46:19.8550411Z LL |         let transd : [u8; 9] = mem::transmute(s);
2019-09-09T16:46:19.8550509Z    |
2019-09-09T16:46:19.8550509Z    |
2019-09-09T16:46:19.8550549Z    = note: source type: `S<u32, i32>` (96 bits)
2019-09-09T16:46:19.8550590Z    = note: target type: `[u8; 9]` (72 bits)
2019-09-09T16:46:19.8550672Z error: aborting due to previous error
2019-09-09T16:46:19.8550789Z 
2019-09-09T16:46:19.8551026Z For more information about this error, try `rustc --explain E0512`.
2019-09-09T16:46:19.8551076Z 
2019-09-09T16:46:19.8551076Z 
2019-09-09T16:46:19.8551264Z ------------------------------------------
2019-09-09T16:46:19.8551454Z 
2019-09-09T16:46:19.8551477Z 
2019-09-09T16:46:19.8551848Z ---- [ui] ui/packed/packed-struct-layout.rs stdout ----
2019-09-09T16:46:19.8551877Z 
2019-09-09T16:46:19.8552059Z error: test compilation failed although it shouldn't!
2019-09-09T16:46:19.8552098Z status: exit code: 1
2019-09-09T16:46:19.8552696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-struct-layout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-layout/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-layout/auxiliary"
2019-09-09T16:46:19.8552967Z ------------------------------------------
2019-09-09T16:46:19.8552993Z 
2019-09-09T16:46:19.8553168Z ------------------------------------------
2019-09-09T16:46:19.8553220Z stderr:
2019-09-09T16:46:19.8553220Z stderr:
2019-09-09T16:46:19.8553561Z ------------------------------------------
2019-09-09T16:46:19.8553775Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2019-09-09T16:46:19.8554028Z    |
2019-09-09T16:46:19.8554028Z    |
2019-09-09T16:46:19.8554064Z LL |         let transd : [u8; 5] = mem::transmute(s5);
2019-09-09T16:46:19.8554151Z    |
2019-09-09T16:46:19.8554151Z    |
2019-09-09T16:46:19.8554185Z    = note: source type: `S5` (64 bits)
2019-09-09T16:46:19.8554239Z    = note: target type: `[u8; 5]` (40 bits)
2019-09-09T16:46:19.8554304Z error: aborting due to previous error
2019-09-09T16:46:19.8554327Z 
2019-09-09T16:46:19.8554520Z For more information about this error, try `rustc --explain E0512`.
2019-09-09T16:46:19.8554562Z 
---
2019-09-09T16:46:19.8563254Z ---- [ui] ui/packed/packed-tuple-struct-layout.rs stdout ----
2019-09-09T16:46:19.8563298Z 
2019-09-09T16:46:19.8563476Z error: test compilation failed although it shouldn't!
2019-09-09T16:46:19.8563521Z status: exit code: 1
2019-09-09T16:46:19.8564189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-tuple-struct-layout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-tuple-struct-layout/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-tuple-struct-layout/auxiliary"
2019-09-09T16:46:19.8564485Z ------------------------------------------
2019-09-09T16:46:19.8564512Z 
2019-09-09T16:46:19.8564685Z ------------------------------------------
2019-09-09T16:46:19.8564721Z stderr:
2019-09-09T16:46:19.8564721Z stderr:
2019-09-09T16:46:19.8564903Z ------------------------------------------
2019-09-09T16:46:19.8565112Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2019-09-09T16:46:19.8565376Z    |
2019-09-09T16:46:19.8565376Z    |
2019-09-09T16:46:19.8565413Z LL |         let transd : [u8; 5] = mem::transmute(s5);
2019-09-09T16:46:19.8565511Z    |
2019-09-09T16:46:19.8565511Z    |
2019-09-09T16:46:19.8565545Z    = note: source type: `S5` (64 bits)
2019-09-09T16:46:19.8565582Z    = note: target type: `[u8; 5]` (40 bits)
2019-09-09T16:46:19.8565657Z error: aborting due to previous error
2019-09-09T16:46:19.8565680Z 
2019-09-09T16:46:19.8565874Z For more information about this error, try `rustc --explain E0512`.
2019-09-09T16:46:19.8565900Z 
---
2019-09-09T16:46:19.8571677Z 
2019-09-09T16:46:19.8571863Z ---- [ui] ui/print_type_sizes/packed.rs stdout ----
2019-09-09T16:46:19.8571903Z diff of stdout:
2019-09-09T16:46:19.8571926Z 
2019-09-09T16:46:19.8572213Z - print-type-size type: `Packed2C`: 12 bytes, alignment: 2 bytes
2019-09-09T16:46:19.8572455Z + print-type-size type: `Packed2C`: 14 bytes, alignment: 2 bytes
2019-09-09T16:46:19.8572640Z 2 print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8572837Z 3 print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8573019Z 4 print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8573223Z + print-type-size     padding: 2 bytes
2019-09-09T16:46:19.8573223Z + print-type-size     padding: 2 bytes
2019-09-09T16:46:19.8573420Z 5 print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8573598Z 6 print-type-size     padding: 1 bytes
2019-09-09T16:46:19.8573787Z 7 print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8573814Z 
2019-09-09T16:46:19.8574006Z 8 print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8574186Z 9 print-type-size     end padding: 1 bytes
2019-09-09T16:46:19.8574384Z - print-type-size type: `Padded`: 12 bytes, alignment: 4 bytes
2019-09-09T16:46:19.8574590Z - print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8574773Z - print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8575131Z + print-type-size type: `Packed1`: 13 bytes, alignment: 1 bytes
2019-09-09T16:46:19.8575320Z 13 print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8575492Z 14 print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8575662Z + print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8575847Z + print-type-size     padding: 2 bytes
2019-09-09T16:46:19.8576020Z 15 print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8576656Z + print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8576928Z + print-type-size     padding: 1 bytes
2019-09-09T16:46:19.8577558Z 16 print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8577775Z - print-type-size     end padding: 2 bytes
2019-09-09T16:46:19.8578010Z - print-type-size type: `Packed1`: 10 bytes, alignment: 1 bytes
2019-09-09T16:46:19.8578268Z + print-type-size type: `Padded`: 12 bytes, alignment: 4 bytes
2019-09-09T16:46:19.8578493Z + print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8578712Z + print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8578948Z 19 print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8579164Z 20 print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8579378Z - print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8579610Z 22 print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8579992Z - print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8580173Z 24 print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8580371Z + print-type-size     end padding: 2 bytes
2019-09-09T16:46:19.8580575Z 25 print-type-size type: `Packed2`: 10 bytes, alignment: 2 bytes
2019-09-09T16:46:19.8580768Z 26 print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8580949Z 27 print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8581013Z 
2019-09-09T16:46:19.8581052Z The actual stdout differed from the expected stdout.
2019-09-09T16:46:19.8581626Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/packed.stdout
2019-09-09T16:46:19.8581626Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/packed.stdout
2019-09-09T16:46:19.8581841Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T16:46:19.8582105Z To only update this specific test, also pass `--test-args print_type_sizes/packed.rs`
2019-09-09T16:46:19.8582183Z error: 1 errors occurred comparing output.
2019-09-09T16:46:19.8582218Z status: exit code: 0
2019-09-09T16:46:19.8582218Z status: exit code: 0
2019-09-09T16:46:19.8582887Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/packed/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8583194Z ------------------------------------------
2019-09-09T16:46:19.8583194Z ------------------------------------------
2019-09-09T16:46:19.8583402Z print-type-size type: `Packed2C`: 14 bytes, alignment: 2 bytes
2019-09-09T16:46:19.8583579Z print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8583766Z print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8583939Z print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8584109Z print-type-size     padding: 2 bytes
2019-09-09T16:46:19.8584282Z print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8584469Z print-type-size     padding: 1 bytes
2019-09-09T16:46:19.8584648Z print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8584816Z print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8585011Z print-type-size     end padding: 1 bytes
2019-09-09T16:46:19.8585201Z print-type-size type: `Packed1`: 13 bytes, alignment: 1 bytes
2019-09-09T16:46:19.8585551Z print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8585743Z print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8586118Z print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8586933Z print-type-size     padding: 2 bytes
2019-09-09T16:46:19.8587178Z print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8587391Z print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8587600Z print-type-size     padding: 1 bytes
2019-09-09T16:46:19.8587811Z print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8588063Z print-type-size type: `Padded`: 12 bytes, alignment: 4 bytes
2019-09-09T16:46:19.8588281Z print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8588653Z print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8588883Z print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8589093Z print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8595726Z print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8601095Z print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8604272Z print-type-size     end padding: 2 bytes
2019-09-09T16:46:19.8604512Z print-type-size type: `Packed2`: 10 bytes, alignment: 2 bytes
2019-09-09T16:46:19.8604720Z print-type-size     field `.g`: 4 bytes
2019-09-09T16:46:19.8604902Z print-type-size     field `.h`: 2 bytes
2019-09-09T16:46:19.8605239Z print-type-size     field `.a`: 1 bytes
2019-09-09T16:46:19.8605430Z print-type-size     field `.b`: 1 bytes
2019-09-09T16:46:19.8605607Z print-type-size     field `.c`: 1 bytes
2019-09-09T16:46:19.8605779Z print-type-size     field `.d`: 1 bytes
2019-09-09T16:46:19.8606000Z ------------------------------------------
2019-09-09T16:46:19.8606048Z stderr:
2019-09-09T16:46:19.8606680Z ------------------------------------------
2019-09-09T16:46:19.8606722Z 
2019-09-09T16:46:19.8606722Z 
2019-09-09T16:46:19.8606956Z ------------------------------------------
2019-09-09T16:46:19.8606988Z 
2019-09-09T16:46:19.8607014Z 
2019-09-09T16:46:19.8607273Z ---- [ui] ui/rfc-2126-extern-absolute-paths/non-existent-3.rs stdout ----
2019-09-09T16:46:19.8607326Z 
2019-09-09T16:46:19.8607375Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T16:46:19.8607422Z status: signal: 11
2019-09-09T16:46:19.8608247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2126-extern-absolute-paths/non-existent-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/non-existent-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/non-existent-3/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8608591Z ------------------------------------------
2019-09-09T16:46:19.8608625Z 
2019-09-09T16:46:19.8609022Z ------------------------------------------
2019-09-09T16:46:19.8609106Z stderr:
2019-09-09T16:46:19.8609106Z stderr:
2019-09-09T16:46:19.8609364Z ------------------------------------------
2019-09-09T16:46:19.8609395Z 
2019-09-09T16:46:19.8609764Z ------------------------------------------
2019-09-09T16:46:19.8609788Z 
2019-09-09T16:46:19.8609824Z 
2019-09-09T16:46:19.8610021Z ---- [ui] ui/rfc-2126-extern-absolute-paths/not-whitelisted.rs stdout ----
2019-09-09T16:46:19.8610051Z 
2019-09-09T16:46:19.8610108Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T16:46:19.8610146Z status: signal: 11
2019-09-09T16:46:19.8610796Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8611077Z ------------------------------------------
2019-09-09T16:46:19.8611119Z 
2019-09-09T16:46:19.8611291Z ------------------------------------------
2019-09-09T16:46:19.8611326Z stderr:
---
2019-09-09T16:46:19.8613810Z 
2019-09-09T16:46:19.8613832Z 
2019-09-09T16:46:19.8614002Z ---- [ui] ui/use/use-keyword.rs stdout ----
2019-09-09T16:46:19.8614034Z 
2019-09-09T16:46:19.8614071Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T16:46:19.8614123Z status: signal: 11
2019-09-09T16:46:19.8614675Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-keyword" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-keyword/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8614939Z ------------------------------------------
2019-09-09T16:46:19.8614966Z 
2019-09-09T16:46:19.8615151Z ------------------------------------------
2019-09-09T16:46:19.8615186Z stderr:
2019-09-09T16:46:19.8615186Z stderr:
2019-09-09T16:46:19.8615426Z ------------------------------------------
2019-09-09T16:46:19.8615498Z error[E0429]: `self` imports are only allowed within a { } list
2019-09-09T16:46:19.8615751Z    |
2019-09-09T16:46:19.8615801Z LL |         use self as A;
2019-09-09T16:46:19.8615838Z    |             ^^^^^^^^^
2019-09-09T16:46:19.8615860Z 
2019-09-09T16:46:19.8615860Z 
2019-09-09T16:46:19.8615880Z 
2019-09-09T16:46:19.8616069Z ------------------------------------------
2019-09-09T16:46:19.8616094Z 
2019-09-09T16:46:19.8616114Z 
2019-09-09T16:46:19.8616633Z ---- [ui] ui/use/use-mod/use-mod-2.rs stdout ----
2019-09-09T16:46:19.8616679Z 
2019-09-09T16:46:19.8616760Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T16:46:19.8616806Z status: signal: 11
2019-09-09T16:46:19.8624293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-mod/use-mod-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-mod/use-mod-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-mod/use-mod-2/auxiliary" "-A" "unused"
2019-09-09T16:46:19.8624625Z ------------------------------------------
2019-09-09T16:46:19.8624854Z 
2019-09-09T16:46:19.8625038Z ------------------------------------------
2019-09-09T16:46:19.8625255Z stderr:
---
2019-09-09T16:46:19.8631464Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T16:46:19.8631691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T16:46:19.8631736Z 
2019-09-09T16:46:19.8631758Z 
2019-09-09T16:46:19.8633732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T16:46:19.8634013Z 
2019-09-09T16:46:19.8634042Z 
2019-09-09T16:46:19.8634096Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T16:46:19.8634162Z Build completed unsuccessfully in 1:01:56
2019-09-09T16:46:19.8634162Z Build completed unsuccessfully in 1:01:56
2019-09-09T16:46:19.8707718Z == clock drift check ==
2019-09-09T16:46:19.8723125Z   local time: Mon Sep  9 16:46:19 UTC 2019
2019-09-09T16:46:20.0307115Z   network time: Mon, 09 Sep 2019 16:46:20 GMT
2019-09-09T16:46:20.0313063Z == end clock drift check ==
2019-09-09T16:46:20.9281572Z ##[error]Bash exited with code '1'.
2019-09-09T16:46:20.9342221Z ##[section]Starting: Checkout
2019-09-09T16:46:20.9343897Z ==============================================================================
2019-09-09T16:46:20.9344113Z Task         : Get sources
2019-09-09T16:46:20.9344173Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
