plain
2020-03-08T10:46:18.1136852Z -   --> $DIR/ub-enum.rs:24:1
2020-03-08T10:46:18.1137678Z + error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2020-03-08T10:46:18.1138455Z +   --> $DIR/ub-enum.rs:91:77
2020-03-08T10:46:18.1138868Z 3    |
2020-03-08T10:46:18.1139538Z - LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
2020-03-08T10:46:18.1141152Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 1, but expected a valid enum discriminant
2020-03-08T10:46:18.1142441Z -    |
2020-03-08T10:46:18.1143753Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1146359Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1147275Z -   --> $DIR/ub-enum.rs:27:1
2020-03-08T10:46:18.1148668Z -    |
2020-03-08T10:46:18.1148668Z -    |
2020-03-08T10:46:18.1150140Z - LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
2020-03-08T10:46:18.1152361Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1156650Z -    |
2020-03-08T10:46:18.1157455Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1158683Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1159112Z -   --> $DIR/ub-enum.rs:30:1
2020-03-08T10:46:18.1159419Z -    |
2020-03-08T10:46:18.1159419Z -    |
2020-03-08T10:46:18.1161880Z - LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
2020-03-08T10:46:18.1164388Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1165822Z -    |
2020-03-08T10:46:18.1166601Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1167596Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1168003Z -   --> $DIR/ub-enum.rs:42:1
2020-03-08T10:46:18.1168318Z -    |
2020-03-08T10:46:18.1168318Z -    |
2020-03-08T10:46:18.1168735Z - LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
2020-03-08T10:46:18.1169490Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected a valid enum discriminant
2020-03-08T10:46:18.1170014Z -    |
2020-03-08T10:46:18.1170775Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1172407Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1172929Z -   --> $DIR/ub-enum.rs:44:1
2020-03-08T10:46:18.1173291Z -    |
2020-03-08T10:46:18.1173291Z -    |
2020-03-08T10:46:18.1173825Z - LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
2020-03-08T10:46:18.1175120Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1175701Z -    |
2020-03-08T10:46:18.1176473Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1177462Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1178114Z -   --> $DIR/ub-enum.rs:47:1
2020-03-08T10:46:18.1183013Z -    |
2020-03-08T10:46:18.1183013Z -    |
2020-03-08T10:46:18.1183881Z - LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
2020-03-08T10:46:18.1185233Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1185824Z -    |
2020-03-08T10:46:18.1186762Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1187781Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1188178Z -   --> $DIR/ub-enum.rs:56:1
2020-03-08T10:46:18.1188484Z -    |
2020-03-08T10:46:18.1188484Z -    |
2020-03-08T10:46:18.1190682Z - LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
2020-03-08T10:46:18.1192437Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1193205Z -    |
2020-03-08T10:46:18.1194130Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1195740Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1196333Z -   --> $DIR/ub-enum.rs:60:1
2020-03-08T10:46:18.1196626Z -    |
2020-03-08T10:46:18.1196626Z -    |
2020-03-08T10:46:18.1197088Z - LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
2020-03-08T10:46:18.1198030Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<enum-tag>, but expected initialized plain (non-pointer) bytes
2020-03-08T10:46:18.1198638Z -    |
2020-03-08T10:46:18.1199388Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1200349Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1200742Z -   --> $DIR/ub-enum.rs:77:1
2020-03-08T10:46:18.1201229Z -    |
2020-03-08T10:46:18.1201229Z -    |
2020-03-08T10:46:18.1201943Z - LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
2020-03-08T10:46:18.1202938Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(B)>.0
2020-03-08T10:46:18.1203607Z -    |
2020-03-08T10:46:18.1204837Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1205974Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1206369Z -   --> $DIR/ub-enum.rs:79:1
2020-03-08T10:46:18.1206675Z -    |
2020-03-08T10:46:18.1206675Z -    |
2020-03-08T10:46:18.1207701Z - LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
2020-03-08T10:46:18.1208651Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(D)>.0
2020-03-08T10:46:18.1209225Z -    |
2020-03-08T10:46:18.1209976Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1210942Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1212189Z -   --> $DIR/ub-enum.rs:87:1
2020-03-08T10:46:18.1212577Z -    |
2020-03-08T10:46:18.1212577Z -    |
2020-03-08T10:46:18.1213370Z - LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
2020-03-08T10:46:18.1215020Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 4294967295 at .<enum-variant(Some)>.0.1, but expected a valid unicode codepoint
2020-03-08T10:46:18.1215653Z -    |
2020-03-08T10:46:18.1216426Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1217566Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1217960Z -   --> $DIR/ub-enum.rs:91:1
2020-03-08T10:46:18.1218265Z -    |
2020-03-08T10:46:18.1218265Z -    |
2020-03-08T10:46:18.1218613Z 92 LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(1usize) };
2020-03-08T10:46:18.1219536Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(Err)>.0.1
2020-03-08T10:46:18.1220435Z 94    |
2020-03-08T10:46:18.1220435Z 94    |
2020-03-08T10:46:18.1221184Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1222122Z +    = note: source type: `usize` (32 bits)
2020-03-08T10:46:18.1222756Z +    = note: target type: `std::result::Result<(i32, Never), (i32, !)>` (64 bits)
2020-03-08T10:46:18.1223585Z - error[E0080]: it is undefined behavior to use this value
2020-03-08T10:46:18.1224083Z -   --> $DIR/ub-enum.rs:93:1
2020-03-08T10:46:18.1225157Z + error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2020-03-08T10:46:18.1225638Z +   --> $DIR/ub-enum.rs:93:77
2020-03-08T10:46:18.1225638Z +   --> $DIR/ub-enum.rs:93:77
2020-03-08T10:46:18.1225989Z 99    |
2020-03-08T10:46:18.1226313Z 100 LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(1usize) };
2020-03-08T10:46:18.1227268Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(Err)>.0.1
2020-03-08T10:46:18.1228177Z 102    |
2020-03-08T10:46:18.1228177Z 102    |
2020-03-08T10:46:18.1228937Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-08T10:46:18.1229466Z +    = note: source type: `usize` (32 bits)
2020-03-08T10:46:18.1229814Z +    = note: target type: `std::result::Result<(i32, !), (i32, Never)>` (64 bits)
2020-03-08T10:46:18.1230430Z - error: aborting due to 13 previous errors
2020-03-08T10:46:18.1230682Z + error: aborting due to 2 previous errors
2020-03-08T10:46:18.1230884Z 106 
2020-03-08T10:46:18.1231597Z - For more information about this error, try `rustc --explain E0080`.
2020-03-08T10:46:18.1231597Z - For more information about this error, try `rustc --explain E0080`.
2020-03-08T10:46:18.1232646Z + For more information about this error, try `rustc --explain E0512`.
2020-03-08T10:46:18.1232952Z 108 
2020-03-08T10:46:18.1233077Z 
2020-03-08T10:46:18.1233188Z 
2020-03-08T10:46:18.1233449Z The actual stderr differed from the expected stderr.
2020-03-08T10:46:18.1234182Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.stderr
2020-03-08T10:46:18.1235134Z To update references, rerun the tests and pass the `--bless` flag
2020-03-08T10:46:18.1237362Z To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`
2020-03-08T10:46:18.1237893Z error: 1 errors occurred comparing output.
2020-03-08T10:46:18.1240221Z status: exit code: 1
2020-03-08T10:46:18.1240221Z status: exit code: 1
2020-03-08T10:46:18.1243842Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
2020-03-08T10:46:18.1249922Z ------------------------------------------
2020-03-08T10:46:18.1250112Z 
2020-03-08T10:46:18.1251050Z ------------------------------------------
2020-03-08T10:46:18.1252205Z stderr:
2020-03-08T10:46:18.1252205Z stderr:
2020-03-08T10:46:18.1252828Z ------------------------------------------
2020-03-08T10:46:18.1253644Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2020-03-08T10:46:18.1254555Z   --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:91:77
2020-03-08T10:46:18.1254859Z    |
2020-03-08T10:46:18.1255261Z LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(1usize) };
2020-03-08T10:46:18.1256963Z    |
2020-03-08T10:46:18.1256963Z    |
2020-03-08T10:46:18.1257311Z    = note: source type: `usize` (32 bits)
2020-03-08T10:46:18.1257755Z    = note: target type: `std::result::Result<(i32, Never), (i32, !)>` (64 bits)
2020-03-08T10:46:18.1259412Z error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
2020-03-08T10:46:18.1260974Z   --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:93:77
2020-03-08T10:46:18.1261299Z    |
2020-03-08T10:46:18.1261299Z    |
2020-03-08T10:46:18.1261697Z LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(1usize) };
2020-03-08T10:46:18.1262619Z    |
2020-03-08T10:46:18.1262619Z    |
2020-03-08T10:46:18.1262890Z    = note: source type: `usize` (32 bits)
2020-03-08T10:46:18.1263317Z    = note: target type: `std::result::Result<(i32, !), (i32, Never)>` (64 bits)
2020-03-08T10:46:18.1264129Z error: aborting due to 2 previous errors
2020-03-08T10:46:18.1264330Z 
2020-03-08T10:46:18.1265290Z For more information about this error, try `rustc --explain E0512`.
2020-03-08T10:46:18.1265789Z 
---
2020-03-08T10:46:18.1271174Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-08T10:46:18.1277950Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-08T10:46:18.1286733Z 
2020-03-08T10:46:18.1286872Z 
2020-03-08T10:46:18.1291834Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-08T10:46:18.1296457Z 
2020-03-08T10:46:18.1296552Z 
2020-03-08T10:46:18.1297186Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-03-08T10:46:18.1297587Z Build completed unsuccessfully in 1:20:13
2020-03-08T10:46:18.1297587Z Build completed unsuccessfully in 1:20:13
2020-03-08T10:46:18.1297836Z == clock drift check ==
2020-03-08T10:46:18.1298087Z   local time: Sun Mar  8 10:46:18 UTC 2020
2020-03-08T10:46:18.6823103Z   network time: Sun, 08 Mar 2020 10:46:18 GMT
2020-03-08T10:46:18.6830177Z == end clock drift check ==
2020-03-08T10:46:19.3152683Z 
2020-03-08T10:46:19.3238927Z ##[error]Bash exited with code '1'.
2020-03-08T10:46:19.3314519Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-08T10:46:19.3319310Z ==============================================================================
2020-03-08T10:46:19.3319664Z Task         : Get sources
2020-03-08T10:46:19.3320040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
