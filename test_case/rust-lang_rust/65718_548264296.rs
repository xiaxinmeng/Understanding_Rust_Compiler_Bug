plain
2019-10-31T08:24:36.5183810Z failures:
2019-10-31T08:24:36.5189828Z 
2019-10-31T08:24:36.5246330Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs stdout ----
2019-10-31T08:24:36.5246451Z 
2019-10-31T08:24:36.5246695Z error: verification with 'FileCheck' failed
2019-10-31T08:24:36.5246780Z status: exit code: 1
2019-10-31T08:24:36.5247281Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs"
2019-10-31T08:24:36.5247880Z ------------------------------------------
2019-10-31T08:24:36.5247927Z 
2019-10-31T08:24:36.5248151Z ------------------------------------------
2019-10-31T08:24:36.5248215Z stderr:
2019-10-31T08:24:36.5248215Z stderr:
2019-10-31T08:24:36.5248438Z ------------------------------------------
2019-10-31T08:24:36.5248752Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:32:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5249135Z  // CHECK: [[A:%[0-9]+]] = lshr <2 x i32> %_2, <i32 31, i32 31>
2019-10-31T08:24:36.5249214Z            ^
2019-10-31T08:24:36.5249595Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:7:23: note: scanning from here
2019-10-31T08:24:36.5249731Z define i8 @bitmask_int(<2 x i32> %x) unnamed_addr #0 {
2019-10-31T08:24:36.5249797Z                       ^
2019-10-31T08:24:36.5250172Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:11:2: note: possible intended match here
2019-10-31T08:24:36.5250278Z  %1 = lshr <2 x i32> %x, <i32 31, i32 31>
2019-10-31T08:24:36.5250674Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:42:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5250674Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:42:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5250948Z  // CHECK: [[A:%[0-9]+]] = lshr <2 x i32> %_2, <i32 31, i32 31>
2019-10-31T08:24:36.5251042Z            ^
2019-10-31T08:24:36.5251569Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:25:24: note: scanning from here
2019-10-31T08:24:36.5251861Z define i8 @bitmask_uint(<2 x i32> %x) unnamed_addr #0 {
2019-10-31T08:24:36.5251935Z                        ^
2019-10-31T08:24:36.5252311Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:29:2: note: possible intended match here
2019-10-31T08:24:36.5252433Z  %1 = lshr <2 x i32> %x, <i32 31, i32 31>
2019-10-31T08:24:36.5252810Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:52:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5252810Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:52:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5253170Z  // CHECK: [[A:%[0-9]+]] = lshr <16 x i8> %_2, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>
2019-10-31T08:24:36.5253284Z            ^
2019-10-31T08:24:36.5253644Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:43:26: note: scanning from here
2019-10-31T08:24:36.5253753Z define i16 @bitmask_int16(<16 x i8> %x) unnamed_addr #0 {
2019-10-31T08:24:36.5253845Z                          ^
2019-10-31T08:24:36.5254205Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:48:2: note: possible intended match here
2019-10-31T08:24:36.5254350Z  %2 = lshr <16 x i8> %x, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>
2019-10-31T08:24:36.5254478Z 
2019-10-31T08:24:36.5254689Z ------------------------------------------
2019-10-31T08:24:36.5254747Z 
2019-10-31T08:24:36.5254777Z 
2019-10-31T08:24:36.5254777Z 
2019-10-31T08:24:36.5255064Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs stdout ----
2019-10-31T08:24:36.5255127Z 
2019-10-31T08:24:36.5255339Z error: verification with 'FileCheck' failed
2019-10-31T08:24:36.5255422Z status: exit code: 1
2019-10-31T08:24:36.5255969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs"
2019-10-31T08:24:36.5256450Z ------------------------------------------
2019-10-31T08:24:36.5256495Z 
2019-10-31T08:24:36.5256724Z ------------------------------------------
2019-10-31T08:24:36.5256790Z stderr:
2019-10-31T08:24:36.5256790Z stderr:
2019-10-31T08:24:36.5257011Z ------------------------------------------
2019-10-31T08:24:36.5257409Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:121:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5260774Z  // CHECK: %{{[0-9]+}} = call <2 x i8> @llvm.sadd.sat.v2i8(<2 x i8> %_3, <2 x i8> %_4)
2019-10-31T08:24:36.5261066Z            ^
2019-10-31T08:24:36.5262027Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:7:27: note: scanning from here
2019-10-31T08:24:36.5262185Z define <2 x i8> @sadd_i8x2(<2 x i8> %x, <2 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5262275Z                           ^
2019-10-31T08:24:36.5262666Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:12:2: note: possible intended match here
2019-10-31T08:24:36.5262804Z  %2 = call <2 x i8> @llvm.sadd.sat.v2i8(<2 x i8> %x, <2 x i8> %y)
2019-10-31T08:24:36.5263395Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:128:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5263395Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:128:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5263731Z  // CHECK: %{{[0-9]+}} = call <4 x i8> @llvm.sadd.sat.v4i8(<4 x i8> %_3, <4 x i8> %_4)
2019-10-31T08:24:36.5263813Z            ^
2019-10-31T08:24:36.5264224Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:24:27: note: scanning from here
2019-10-31T08:24:36.5264364Z define <4 x i8> @sadd_i8x4(<4 x i8> %x, <4 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5264440Z                           ^
2019-10-31T08:24:36.5265016Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:29:2: note: possible intended match here
2019-10-31T08:24:36.5265138Z  %2 = call <4 x i8> @llvm.sadd.sat.v4i8(<4 x i8> %x, <4 x i8> %y)
2019-10-31T08:24:36.5265564Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:135:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5265564Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:135:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5266052Z  // CHECK: %{{[0-9]+}} = call <8 x i8> @llvm.sadd.sat.v8i8(<8 x i8> %_3, <8 x i8> %_4)
2019-10-31T08:24:36.5266158Z            ^
2019-10-31T08:24:36.5266702Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:41:27: note: scanning from here
2019-10-31T08:24:36.5267058Z define <8 x i8> @sadd_i8x8(<8 x i8> %x, <8 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5267387Z                           ^
2019-10-31T08:24:36.5267775Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:46:2: note: possible intended match here
2019-10-31T08:24:36.5267912Z  %2 = call <8 x i8> @llvm.sadd.sat.v8i8(<8 x i8> %x, <8 x i8> %y)
2019-10-31T08:24:36.5269091Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:142:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5269091Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:142:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5269427Z  // CHECK: %{{[0-9]+}} = call <16 x i8> @llvm.sadd.sat.v16i8(<16 x i8> %_3, <16 x i8> %_4)
2019-10-31T08:24:36.5269842Z            ^
2019-10-31T08:24:36.5270484Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:58:29: note: scanning from here
2019-10-31T08:24:36.5270622Z define <16 x i8> @sadd_i8x16(<16 x i8> %x, <16 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5270724Z                             ^
2019-10-31T08:24:36.5271505Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:63:2: note: possible intended match here
2019-10-31T08:24:36.5271734Z  %2 = call <16 x i8> @llvm.sadd.sat.v16i8(<16 x i8> %x, <16 x i8> %y)
2019-10-31T08:24:36.5272352Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:149:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5272352Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:149:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5272717Z  // CHECK: %{{[0-9]+}} = call <32 x i8> @llvm.sadd.sat.v32i8(<32 x i8> %_3, <32 x i8> %_4)
2019-10-31T08:24:36.5272808Z            ^
2019-10-31T08:24:36.5273818Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:75:29: note: scanning from here
2019-10-31T08:24:36.5273959Z define <32 x i8> @sadd_i8x32(<32 x i8> %x, <32 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5274033Z                             ^
2019-10-31T08:24:36.5274454Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:80:2: note: possible intended match here
2019-10-31T08:24:36.5274581Z  %2 = call <32 x i8> @llvm.sadd.sat.v32i8(<32 x i8> %x, <32 x i8> %y)
2019-10-31T08:24:36.5275011Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:156:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5275011Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:156:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5275334Z  // CHECK: %{{[0-9]+}} = call <64 x i8> @llvm.sadd.sat.v64i8(<64 x i8> %_3, <64 x i8> %_4)
2019-10-31T08:24:36.5275434Z            ^
2019-10-31T08:24:36.5275813Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:92:29: note: scanning from here
2019-10-31T08:24:36.5275957Z define <64 x i8> @sadd_i8x64(<64 x i8> %x, <64 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5276050Z                             ^
2019-10-31T08:24:36.5276451Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:97:2: note: possible intended match here
2019-10-31T08:24:36.5276595Z  %2 = call <64 x i8> @llvm.sadd.sat.v64i8(<64 x i8> %x, <64 x i8> %y)
2019-10-31T08:24:36.5277016Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:163:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5277016Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:163:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5277342Z  // CHECK: %{{[0-9]+}} = call <2 x i16> @llvm.sadd.sat.v2i16(<2 x i16> %_3, <2 x i16> %_4)
2019-10-31T08:24:36.5277428Z            ^
2019-10-31T08:24:36.5277858Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:109:29: note: scanning from here
2019-10-31T08:24:36.5277992Z define <2 x i16> @sadd_i16x2(<2 x i16> %x, <2 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5278085Z                             ^
2019-10-31T08:24:36.5278525Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:114:2: note: possible intended match here
2019-10-31T08:24:36.5278658Z  %2 = call <2 x i16> @llvm.sadd.sat.v2i16(<2 x i16> %x, <2 x i16> %y)
2019-10-31T08:24:36.5279194Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:170:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5279194Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:170:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5279547Z  // CHECK: %{{[0-9]+}} = call <4 x i16> @llvm.sadd.sat.v4i16(<4 x i16> %_3, <4 x i16> %_4)
2019-10-31T08:24:36.5279649Z            ^
2019-10-31T08:24:36.5282935Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:126:29: note: scanning from here
2019-10-31T08:24:36.5283127Z define <4 x i16> @sadd_i16x4(<4 x i16> %x, <4 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5283216Z                             ^
2019-10-31T08:24:36.5287170Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:131:2: note: possible intended match here
2019-10-31T08:24:36.5287385Z  %2 = call <4 x i16> @llvm.sadd.sat.v4i16(<4 x i16> %x, <4 x i16> %y)
2019-10-31T08:24:36.5287906Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:177:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5287906Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:177:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5288239Z  // CHECK: %{{[0-9]+}} = call <8 x i16> @llvm.sadd.sat.v8i16(<8 x i16> %_3, <8 x i16> %_4)
2019-10-31T08:24:36.5288344Z            ^
2019-10-31T08:24:36.5288762Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:143:29: note: scanning from here
2019-10-31T08:24:36.5288902Z define <8 x i16> @sadd_i16x8(<8 x i16> %x, <8 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5288997Z                             ^
2019-10-31T08:24:36.5289418Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:148:2: note: possible intended match here
2019-10-31T08:24:36.5289576Z  %2 = call <8 x i16> @llvm.sadd.sat.v8i16(<8 x i16> %x, <8 x i16> %y)
2019-10-31T08:24:36.5290015Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:184:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5290015Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:184:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5290370Z  // CHECK: %{{[0-9]+}} = call <16 x i16> @llvm.sadd.sat.v16i16(<16 x i16> %_3, <16 x i16> %_4)
2019-10-31T08:24:36.5290458Z            ^
2019-10-31T08:24:36.5291057Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:160:31: note: scanning from here
2019-10-31T08:24:36.5291214Z define <16 x i16> @sadd_i16x16(<16 x i16> %x, <16 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5291294Z                               ^
2019-10-31T08:24:36.5291881Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:165:2: note: possible intended match here
2019-10-31T08:24:36.5292019Z  %2 = call <16 x i16> @llvm.sadd.sat.v16i16(<16 x i16> %x, <16 x i16> %y)
2019-10-31T08:24:36.5292437Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:191:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5292437Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:191:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5292773Z  // CHECK: %{{[0-9]+}} = call <32 x i16> @llvm.sadd.sat.v32i16(<32 x i16> %_3, <32 x i16> %_4)
2019-10-31T08:24:36.5292873Z            ^
2019-10-31T08:24:36.5293257Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:177:31: note: scanning from here
2019-10-31T08:24:36.5293398Z define <32 x i16> @sadd_i16x32(<32 x i16> %x, <32 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5293474Z                               ^
2019-10-31T08:24:36.5293995Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:182:2: note: possible intended match here
2019-10-31T08:24:36.5294145Z  %2 = call <32 x i16> @llvm.sadd.sat.v32i16(<32 x i16> %x, <32 x i16> %y)
2019-10-31T08:24:36.5310741Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:198:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5310741Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:198:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5311287Z  // CHECK: %{{[0-9]+}} = call <2 x i32> @llvm.sadd.sat.v2i32(<2 x i32> %_3, <2 x i32> %_4)
2019-10-31T08:24:36.5311407Z            ^
2019-10-31T08:24:36.5312004Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:194:29: note: scanning from here
2019-10-31T08:24:36.5312147Z define <2 x i32> @sadd_i32x2(<2 x i32> %x, <2 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5312253Z                             ^
2019-10-31T08:24:36.5313109Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:199:2: note: possible intended match here
2019-10-31T08:24:36.5313419Z  %2 = call <2 x i32> @llvm.sadd.sat.v2i32(<2 x i32> %x, <2 x i32> %y)
2019-10-31T08:24:36.5313828Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:205:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5313828Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:205:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5314150Z  // CHECK: %{{[0-9]+}} = call <4 x i32> @llvm.sadd.sat.v4i32(<4 x i32> %_3, <4 x i32> %_4)
2019-10-31T08:24:36.5314233Z            ^
2019-10-31T08:24:36.5314647Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:211:29: note: scanning from here
2019-10-31T08:24:36.5314798Z define <4 x i32> @sadd_i32x4(<4 x i32> %x, <4 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5315050Z                             ^
2019-10-31T08:24:36.5315471Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:216:2: note: possible intended match here
2019-10-31T08:24:36.5315599Z  %2 = call <4 x i32> @llvm.sadd.sat.v4i32(<4 x i32> %x, <4 x i32> %y)
2019-10-31T08:24:36.5316030Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:212:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5316030Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:212:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5316356Z  // CHECK: %{{[0-9]+}} = call <8 x i32> @llvm.sadd.sat.v8i32(<8 x i32> %_3, <8 x i32> %_4)
2019-10-31T08:24:36.5316459Z            ^
2019-10-31T08:24:36.5316861Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:228:29: note: scanning from here
2019-10-31T08:24:36.5317017Z define <8 x i32> @sadd_i32x8(<8 x i32> %x, <8 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5317113Z                             ^
2019-10-31T08:24:36.5317533Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:233:2: note: possible intended match here
2019-10-31T08:24:36.5317684Z  %2 = call <8 x i32> @llvm.sadd.sat.v8i32(<8 x i32> %x, <8 x i32> %y)
2019-10-31T08:24:36.5318116Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:219:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5318116Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:219:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5318475Z  // CHECK: %{{[0-9]+}} = call <16 x i32> @llvm.sadd.sat.v16i32(<16 x i32> %_3, <16 x i32> %_4)
2019-10-31T08:24:36.5318566Z            ^
2019-10-31T08:24:36.5318991Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:245:31: note: scanning from here
2019-10-31T08:24:36.5319218Z define <16 x i32> @sadd_i32x16(<16 x i32> %x, <16 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5319315Z                               ^
2019-10-31T08:24:36.5319766Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:250:2: note: possible intended match here
2019-10-31T08:24:36.5319903Z  %2 = call <16 x i32> @llvm.sadd.sat.v16i32(<16 x i32> %x, <16 x i32> %y)
2019-10-31T08:24:36.5320598Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:226:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5320598Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:226:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5321119Z  // CHECK: %{{[0-9]+}} = call <2 x i64> @llvm.sadd.sat.v2i64(<2 x i64> %_3, <2 x i64> %_4)
2019-10-31T08:24:36.5321223Z            ^
2019-10-31T08:24:36.5321634Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:262:29: note: scanning from here
2019-10-31T08:24:36.5321779Z define <2 x i64> @sadd_i64x2(<2 x i64> %x, <2 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5321910Z                             ^
2019-10-31T08:24:36.5322361Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:267:2: note: possible intended match here
2019-10-31T08:24:36.5322514Z  %2 = call <2 x i64> @llvm.sadd.sat.v2i64(<2 x i64> %x, <2 x i64> %y)
2019-10-31T08:24:36.5322961Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:233:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5322961Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:233:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5323298Z  // CHECK: %{{[0-9]+}} = call <4 x i64> @llvm.sadd.sat.v4i64(<4 x i64> %_3, <4 x i64> %_4)
2019-10-31T08:24:36.5323410Z            ^
2019-10-31T08:24:36.5323994Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:279:29: note: scanning from here
2019-10-31T08:24:36.5324118Z define <4 x i64> @sadd_i64x4(<4 x i64> %x, <4 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5324208Z                             ^
2019-10-31T08:24:36.5324600Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:284:2: note: possible intended match here
2019-10-31T08:24:36.5324750Z  %2 = call <4 x i64> @llvm.sadd.sat.v4i64(<4 x i64> %x, <4 x i64> %y)
2019-10-31T08:24:36.5325154Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:240:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5325154Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:240:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5325483Z  // CHECK: %{{[0-9]+}} = call <8 x i64> @llvm.sadd.sat.v8i64(<8 x i64> %_3, <8 x i64> %_4)
2019-10-31T08:24:36.5325567Z            ^
2019-10-31T08:24:36.5325970Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:296:29: note: scanning from here
2019-10-31T08:24:36.5326112Z define <8 x i64> @sadd_i64x8(<8 x i64> %x, <8 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5326189Z                             ^
2019-10-31T08:24:36.5326609Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:301:2: note: possible intended match here
2019-10-31T08:24:36.5326749Z  %2 = call <8 x i64> @llvm.sadd.sat.v8i64(<8 x i64> %x, <8 x i64> %y)
2019-10-31T08:24:36.5327172Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:247:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5327172Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:247:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5327616Z  // CHECK: %{{[0-9]+}} = call <2 x i128> @llvm.sadd.sat.v2i128(<2 x i128> %_3, <2 x i128> %_4)
2019-10-31T08:24:36.5327718Z            ^
2019-10-31T08:24:36.5328099Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:313:31: note: scanning from here
2019-10-31T08:24:36.5328243Z define <2 x i128> @sadd_i128x2(<2 x i128> %x, <2 x i128> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5328318Z                               ^
2019-10-31T08:24:36.5328807Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:318:2: note: possible intended match here
2019-10-31T08:24:36.5328960Z  %2 = call <2 x i128> @llvm.sadd.sat.v2i128(<2 x i128> %x, <2 x i128> %y)
2019-10-31T08:24:36.5329569Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:254:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5329569Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:254:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5329900Z  // CHECK: %{{[0-9]+}} = call <4 x i128> @llvm.sadd.sat.v4i128(<4 x i128> %_3, <4 x i128> %_4)
2019-10-31T08:24:36.5330168Z            ^
2019-10-31T08:24:36.5330913Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:330:31: note: scanning from here
2019-10-31T08:24:36.5331038Z define <4 x i128> @sadd_i128x4(<4 x i128> %x, <4 x i128> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5331130Z                               ^
2019-10-31T08:24:36.5331521Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:335:2: note: possible intended match here
2019-10-31T08:24:36.5331665Z  %2 = call <4 x i128> @llvm.sadd.sat.v4i128(<4 x i128> %x, <4 x i128> %y)
2019-10-31T08:24:36.5332099Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:263:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5332099Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:263:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5332421Z  // CHECK: %{{[0-9]+}} = call <2 x i8> @llvm.uadd.sat.v2i8(<2 x i8> %_3, <2 x i8> %_4)
2019-10-31T08:24:36.5332527Z            ^
2019-10-31T08:24:36.5332915Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:347:27: note: scanning from here
2019-10-31T08:24:36.5333056Z define <2 x i8> @uadd_u8x2(<2 x i8> %x, <2 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5333145Z                           ^
2019-10-31T08:24:36.5333557Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:352:2: note: possible intended match here
2019-10-31T08:24:36.5333706Z  %2 = call <2 x i8> @llvm.uadd.sat.v2i8(<2 x i8> %x, <2 x i8> %y)
2019-10-31T08:24:36.5334133Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:270:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5334133Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:270:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5334465Z  // CHECK: %{{[0-9]+}} = call <4 x i8> @llvm.uadd.sat.v4i8(<4 x i8> %_3, <4 x i8> %_4)
2019-10-31T08:24:36.5334550Z            ^
2019-10-31T08:24:36.5334953Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:364:27: note: scanning from here
2019-10-31T08:24:36.5335078Z define <4 x i8> @uadd_u8x4(<4 x i8> %x, <4 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5335176Z                           ^
2019-10-31T08:24:36.5335596Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:369:2: note: possible intended match here
2019-10-31T08:24:36.5335798Z  %2 = call <4 x i8> @llvm.uadd.sat.v4i8(<4 x i8> %x, <4 x i8> %y)
2019-10-31T08:24:36.5336234Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:277:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5336234Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:277:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5336566Z  // CHECK: %{{[0-9]+}} = call <8 x i8> @llvm.uadd.sat.v8i8(<8 x i8> %_3, <8 x i8> %_4)
2019-10-31T08:24:36.5336664Z            ^
2019-10-31T08:24:36.5337052Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:381:27: note: scanning from here
2019-10-31T08:24:36.5337249Z define <8 x i8> @uadd_u8x8(<8 x i8> %x, <8 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5337329Z                           ^
2019-10-31T08:24:36.5339939Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:386:2: note: possible intended match here
2019-10-31T08:24:36.5340313Z  %2 = call <8 x i8> @llvm.uadd.sat.v8i8(<8 x i8> %x, <8 x i8> %y)
2019-10-31T08:24:36.5341375Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:284:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5341375Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:284:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5342208Z  // CHECK: %{{[0-9]+}} = call <16 x i8> @llvm.uadd.sat.v16i8(<16 x i8> %_3, <16 x i8> %_4)
2019-10-31T08:24:36.5342499Z            ^
2019-10-31T08:24:36.5343271Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:398:29: note: scanning from here
2019-10-31T08:24:36.5343608Z define <16 x i8> @uadd_u8x16(<16 x i8> %x, <16 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5343702Z                             ^
2019-10-31T08:24:36.5344268Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:403:2: note: possible intended match here
2019-10-31T08:24:36.5344422Z  %2 = call <16 x i8> @llvm.uadd.sat.v16i8(<16 x i8> %x, <16 x i8> %y)
2019-10-31T08:24:36.5344836Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:291:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5344836Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:291:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5345163Z  // CHECK: %{{[0-9]+}} = call <32 x i8> @llvm.uadd.sat.v32i8(<32 x i8> %_3, <32 x i8> %_4)
2019-10-31T08:24:36.5345249Z            ^
2019-10-31T08:24:36.5345816Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:415:29: note: scanning from here
2019-10-31T08:24:36.5345955Z define <32 x i8> @uadd_u8x32(<32 x i8> %x, <32 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5346026Z                             ^
2019-10-31T08:24:36.5346442Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:420:2: note: possible intended match here
2019-10-31T08:24:36.5346570Z  %2 = call <32 x i8> @llvm.uadd.sat.v32i8(<32 x i8> %x, <32 x i8> %y)
2019-10-31T08:24:36.5346991Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:298:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5346991Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:298:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5347328Z  // CHECK: %{{[0-9]+}} = call <64 x i8> @llvm.uadd.sat.v64i8(<64 x i8> %_3, <64 x i8> %_4)
2019-10-31T08:24:36.5347429Z            ^
2019-10-31T08:24:36.5347822Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:432:29: note: scanning from here
2019-10-31T08:24:36.5347964Z define <64 x i8> @uadd_u8x64(<64 x i8> %x, <64 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5348170Z                             ^
2019-10-31T08:24:36.5348857Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:437:2: note: possible intended match here
2019-10-31T08:24:36.5349234Z  %2 = call <64 x i8> @llvm.uadd.sat.v64i8(<64 x i8> %x, <64 x i8> %y)
2019-10-31T08:24:36.5349664Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:305:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5349664Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:305:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5349985Z  // CHECK: %{{[0-9]+}} = call <2 x i16> @llvm.uadd.sat.v2i16(<2 x i16> %_3, <2 x i16> %_4)
2019-10-31T08:24:36.5350269Z            ^
2019-10-31T08:24:36.5351111Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:449:29: note: scanning from here
2019-10-31T08:24:36.5351263Z define <2 x i16> @uadd_u16x2(<2 x i16> %x, <2 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5351359Z                             ^
2019-10-31T08:24:36.5351800Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:454:2: note: possible intended match here
2019-10-31T08:24:36.5351963Z  %2 = call <2 x i16> @llvm.uadd.sat.v2i16(<2 x i16> %x, <2 x i16> %y)
2019-10-31T08:24:36.5352553Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:312:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5352553Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:312:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5353125Z  // CHECK: %{{[0-9]+}} = call <4 x i16> @llvm.uadd.sat.v4i16(<4 x i16> %_3, <4 x i16> %_4)
2019-10-31T08:24:36.5353213Z            ^
2019-10-31T08:24:36.5375263Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:466:29: note: scanning from here
2019-10-31T08:24:36.5375447Z define <4 x i16> @uadd_u16x4(<4 x i16> %x, <4 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5375540Z                             ^
2019-10-31T08:24:36.5376128Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:471:2: note: possible intended match here
2019-10-31T08:24:36.5376277Z  %2 = call <4 x i16> @llvm.uadd.sat.v4i16(<4 x i16> %x, <4 x i16> %y)
2019-10-31T08:24:36.5376729Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:319:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5376729Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:319:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5377439Z  // CHECK: %{{[0-9]+}} = call <8 x i16> @llvm.uadd.sat.v8i16(<8 x i16> %_3, <8 x i16> %_4)
2019-10-31T08:24:36.5377533Z            ^
2019-10-31T08:24:36.5378513Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:483:29: note: scanning from here
2019-10-31T08:24:36.5379131Z define <8 x i16> @uadd_u16x8(<8 x i16> %x, <8 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5379411Z                             ^
2019-10-31T08:24:36.5380137Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:488:2: note: possible intended match here
2019-10-31T08:24:36.5380447Z  %2 = call <8 x i16> @llvm.uadd.sat.v8i16(<8 x i16> %x, <8 x i16> %y)
2019-10-31T08:24:36.5380897Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:326:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5380897Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:326:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5381511Z  // CHECK: %{{[0-9]+}} = call <16 x i16> @llvm.uadd.sat.v16i16(<16 x i16> %_3, <16 x i16> %_4)
2019-10-31T08:24:36.5381617Z            ^
2019-10-31T08:24:36.5382304Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:500:31: note: scanning from here
2019-10-31T08:24:36.5382457Z define <16 x i16> @uadd_u16x16(<16 x i16> %x, <16 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5382548Z                               ^
2019-10-31T08:24:36.5382994Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:505:2: note: possible intended match here
2019-10-31T08:24:36.5383145Z  %2 = call <16 x i16> @llvm.uadd.sat.v16i16(<16 x i16> %x, <16 x i16> %y)
2019-10-31T08:24:36.5383681Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:333:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5383681Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:333:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5384078Z  // CHECK: %{{[0-9]+}} = call <32 x i16> @llvm.uadd.sat.v32i16(<32 x i16> %_3, <32 x i16> %_4)
2019-10-31T08:24:36.5384178Z            ^
2019-10-31T08:24:36.5384606Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:517:31: note: scanning from here
2019-10-31T08:24:36.5384743Z define <32 x i16> @uadd_u16x32(<32 x i16> %x, <32 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5384832Z                               ^
2019-10-31T08:24:36.5385413Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:522:2: note: possible intended match here
2019-10-31T08:24:36.5385553Z  %2 = call <32 x i16> @llvm.uadd.sat.v32i16(<32 x i16> %x, <32 x i16> %y)
2019-10-31T08:24:36.5385991Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:340:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5385991Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:340:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5386345Z  // CHECK: %{{[0-9]+}} = call <2 x i32> @llvm.uadd.sat.v2i32(<2 x i32> %_3, <2 x i32> %_4)
2019-10-31T08:24:36.5386436Z            ^
2019-10-31T08:24:36.5386842Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:534:29: note: scanning from here
2019-10-31T08:24:36.5386980Z define <2 x i32> @uadd_u32x2(<2 x i32> %x, <2 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5387057Z                             ^
2019-10-31T08:24:36.5387486Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:539:2: note: possible intended match here
2019-10-31T08:24:36.5387640Z  %2 = call <2 x i32> @llvm.uadd.sat.v2i32(<2 x i32> %x, <2 x i32> %y)
2019-10-31T08:24:36.5388073Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:347:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5388073Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:347:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5388426Z  // CHECK: %{{[0-9]+}} = call <4 x i32> @llvm.uadd.sat.v4i32(<4 x i32> %_3, <4 x i32> %_4)
2019-10-31T08:24:36.5388527Z            ^
2019-10-31T08:24:36.5389482Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:551:29: note: scanning from here
2019-10-31T08:24:36.5389812Z define <4 x i32> @uadd_u32x4(<4 x i32> %x, <4 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5389909Z                             ^
2019-10-31T08:24:36.5390747Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:556:2: note: possible intended match here
2019-10-31T08:24:36.5390897Z  %2 = call <4 x i32> @llvm.uadd.sat.v4i32(<4 x i32> %x, <4 x i32> %y)
2019-10-31T08:24:36.5391510Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:354:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5391510Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:354:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5392300Z  // CHECK: %{{[0-9]+}} = call <8 x i32> @llvm.uadd.sat.v8i32(<8 x i32> %_3, <8 x i32> %_4)
2019-10-31T08:24:36.5392386Z            ^
2019-10-31T08:24:36.5392794Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:568:29: note: scanning from here
2019-10-31T08:24:36.5392924Z define <8 x i32> @uadd_u32x8(<8 x i32> %x, <8 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5393009Z                             ^
2019-10-31T08:24:36.5393504Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:573:2: note: possible intended match here
2019-10-31T08:24:36.5393641Z  %2 = call <8 x i32> @llvm.uadd.sat.v8i32(<8 x i32> %x, <8 x i32> %y)
2019-10-31T08:24:36.5394101Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:361:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5394101Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:361:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5394624Z  // CHECK: %{{[0-9]+}} = call <16 x i32> @llvm.uadd.sat.v16i32(<16 x i32> %_3, <16 x i32> %_4)
2019-10-31T08:24:36.5394718Z            ^
2019-10-31T08:24:36.5395120Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:585:31: note: scanning from here
2019-10-31T08:24:36.5395266Z define <16 x i32> @uadd_u32x16(<16 x i32> %x, <16 x i32> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5395344Z                               ^
2019-10-31T08:24:36.5395777Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:590:2: note: possible intended match here
2019-10-31T08:24:36.5395929Z  %2 = call <16 x i32> @llvm.uadd.sat.v16i32(<16 x i32> %x, <16 x i32> %y)
2019-10-31T08:24:36.5396376Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:368:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5396376Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:368:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5396722Z  // CHECK: %{{[0-9]+}} = call <2 x i64> @llvm.uadd.sat.v2i64(<2 x i64> %_3, <2 x i64> %_4)
2019-10-31T08:24:36.5396819Z            ^
2019-10-31T08:24:36.5397244Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:602:29: note: scanning from here
2019-10-31T08:24:36.5397388Z define <2 x i64> @uadd_u64x2(<2 x i64> %x, <2 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5397476Z                             ^
2019-10-31T08:24:36.5397910Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:607:2: note: possible intended match here
2019-10-31T08:24:36.5398062Z  %2 = call <2 x i64> @llvm.uadd.sat.v2i64(<2 x i64> %x, <2 x i64> %y)
2019-10-31T08:24:36.5398676Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:375:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5398676Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:375:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5399223Z  // CHECK: %{{[0-9]+}} = call <4 x i64> @llvm.uadd.sat.v4i64(<4 x i64> %_3, <4 x i64> %_4)
2019-10-31T08:24:36.5399319Z            ^
2019-10-31T08:24:36.5400163Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:619:29: note: scanning from here
2019-10-31T08:24:36.5400484Z define <4 x i64> @uadd_u64x4(<4 x i64> %x, <4 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5400568Z                             ^
2019-10-31T08:24:36.5401349Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:624:2: note: possible intended match here
2019-10-31T08:24:36.5401566Z  %2 = call <4 x i64> @llvm.uadd.sat.v4i64(<4 x i64> %x, <4 x i64> %y)
2019-10-31T08:24:36.5402312Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:382:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5402312Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:382:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5402640Z  // CHECK: %{{[0-9]+}} = call <8 x i64> @llvm.uadd.sat.v8i64(<8 x i64> %_3, <8 x i64> %_4)
2019-10-31T08:24:36.5402740Z            ^
2019-10-31T08:24:36.5403215Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:636:29: note: scanning from here
2019-10-31T08:24:36.5403364Z define <8 x i64> @uadd_u64x8(<8 x i64> %x, <8 x i64> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5403452Z                             ^
2019-10-31T08:24:36.5403907Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:641:2: note: possible intended match here
2019-10-31T08:24:36.5404049Z  %2 = call <8 x i64> @llvm.uadd.sat.v8i64(<8 x i64> %x, <8 x i64> %y)
2019-10-31T08:24:36.5404492Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:389:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5404492Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:389:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5404854Z  // CHECK: %{{[0-9]+}} = call <2 x i128> @llvm.uadd.sat.v2i128(<2 x i128> %_3, <2 x i128> %_4)
2019-10-31T08:24:36.5404945Z            ^
2019-10-31T08:24:36.5405375Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:653:31: note: scanning from here
2019-10-31T08:24:36.5405511Z define <2 x i128> @uadd_u128x2(<2 x i128> %x, <2 x i128> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5405608Z                               ^
2019-10-31T08:24:36.5406052Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:658:2: note: possible intended match here
2019-10-31T08:24:36.5406188Z  %2 = call <2 x i128> @llvm.uadd.sat.v2i128(<2 x i128> %x, <2 x i128> %y)
2019-10-31T08:24:36.5406641Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:396:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5406641Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:396:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5407003Z  // CHECK: %{{[0-9]+}} = call <4 x i128> @llvm.uadd.sat.v4i128(<4 x i128> %_3, <4 x i128> %_4)
2019-10-31T08:24:36.5407105Z            ^
2019-10-31T08:24:36.5407529Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:670:31: note: scanning from here
2019-10-31T08:24:36.5407679Z define <4 x i128> @uadd_u128x4(<4 x i128> %x, <4 x i128> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5407760Z                               ^
2019-10-31T08:24:36.5408211Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:675:2: note: possible intended match here
2019-10-31T08:24:36.5408356Z  %2 = call <4 x i128> @llvm.uadd.sat.v4i128(<4 x i128> %x, <4 x i128> %y)
2019-10-31T08:24:36.5408802Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:407:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5408802Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:407:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5409150Z  // CHECK: %{{[0-9]+}} = call <2 x i8> @llvm.ssub.sat.v2i8(<2 x i8> %_3, <2 x i8> %_4)
2019-10-31T08:24:36.5409246Z            ^
2019-10-31T08:24:36.5409919Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:687:27: note: scanning from here
2019-10-31T08:24:36.5410288Z define <2 x i8> @ssub_i8x2(<2 x i8> %x, <2 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5410370Z                           ^
2019-10-31T08:24:36.5410804Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:692:2: note: possible intended match here
2019-10-31T08:24:36.5410940Z  %2 = call <2 x i8> @llvm.ssub.sat.v2i8(<2 x i8> %x, <2 x i8> %y)
2019-10-31T08:24:36.5411369Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:414:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5411369Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:414:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5411767Z  // CHECK: %{{[0-9]+}} = call <4 x i8> @llvm.ssub.sat.v4i8(<4 x i8> %_3, <4 x i8> %_4)
2019-10-31T08:24:36.5411857Z            ^
2019-10-31T08:24:36.5412275Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:704:27: note: scanning from here
2019-10-31T08:24:36.5412413Z define <4 x i8> @ssub_i8x4(<4 x i8> %x, <4 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5412507Z                           ^
2019-10-31T08:24:36.5412929Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:709:2: note: possible intended match here
2019-10-31T08:24:36.5413060Z  %2 = call <4 x i8> @llvm.ssub.sat.v4i8(<4 x i8> %x, <4 x i8> %y)
2019-10-31T08:24:36.5413488Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:421:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5413488Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:421:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5413814Z  // CHECK: %{{[0-9]+}} = call <8 x i8> @llvm.ssub.sat.v8i8(<8 x i8> %_3, <8 x i8> %_4)
2019-10-31T08:24:36.5413902Z            ^
2019-10-31T08:24:36.5414299Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:721:27: note: scanning from here
2019-10-31T08:24:36.5414441Z define <8 x i8> @ssub_i8x8(<8 x i8> %x, <8 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5414514Z                           ^
2019-10-31T08:24:36.5414934Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:726:2: note: possible intended match here
2019-10-31T08:24:36.5415071Z  %2 = call <8 x i8> @llvm.ssub.sat.v8i8(<8 x i8> %x, <8 x i8> %y)
2019-10-31T08:24:36.5415498Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:428:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5415498Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:428:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5415826Z  // CHECK: %{{[0-9]+}} = call <16 x i8> @llvm.ssub.sat.v16i8(<16 x i8> %_3, <16 x i8> %_4)
2019-10-31T08:24:36.5415927Z            ^
2019-10-31T08:24:36.5416366Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:738:29: note: scanning from here
2019-10-31T08:24:36.5416499Z define <16 x i8> @ssub_i8x16(<16 x i8> %x, <16 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5416585Z                             ^
2019-10-31T08:24:36.5417016Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:743:2: note: possible intended match here
2019-10-31T08:24:36.5417165Z  %2 = call <16 x i8> @llvm.ssub.sat.v16i8(<16 x i8> %x, <16 x i8> %y)
2019-10-31T08:24:36.5417892Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:435:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5417892Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:435:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5418333Z  // CHECK: %{{[0-9]+}} = call <32 x i8> @llvm.ssub.sat.v32i8(<32 x i8> %_3, <32 x i8> %_4)
2019-10-31T08:24:36.5418537Z            ^
2019-10-31T08:24:36.5418982Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:755:29: note: scanning from here
2019-10-31T08:24:36.5419123Z define <32 x i8> @ssub_i8x32(<32 x i8> %x, <32 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5419202Z                             ^
2019-10-31T08:24:36.5419642Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:760:2: note: possible intended match here
2019-10-31T08:24:36.5419853Z  %2 = call <32 x i8> @llvm.ssub.sat.v32i8(<32 x i8> %x, <32 x i8> %y)
2019-10-31T08:24:36.5420314Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:442:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5420314Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:442:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5420679Z  // CHECK: %{{[0-9]+}} = call <64 x i8> @llvm.ssub.sat.v64i8(<64 x i8> %_3, <64 x i8> %_4)
2019-10-31T08:24:36.5420775Z            ^
2019-10-31T08:24:36.5421250Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:772:29: note: scanning from here
2019-10-31T08:24:36.5421398Z define <64 x i8> @ssub_i8x64(<64 x i8> %x, <64 x i8> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5421479Z                             ^
2019-10-31T08:24:36.5421923Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:777:2: note: possible intended match here
2019-10-31T08:24:36.5422077Z  %2 = call <64 x i8> @llvm.ssub.sat.v64i8(<64 x i8> %x, <64 x i8> %y)
2019-10-31T08:24:36.5422531Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:449:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5422531Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:449:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5422886Z  // CHECK: %{{[0-9]+}} = call <2 x i16> @llvm.ssub.sat.v2i16(<2 x i16> %_3, <2 x i16> %_4)
2019-10-31T08:24:36.5422983Z            ^
2019-10-31T08:24:36.5423411Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:789:29: note: scanning from here
2019-10-31T08:24:36.5423545Z define <2 x i16> @ssub_i16x2(<2 x i16> %x, <2 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5423632Z                             ^
2019-10-31T08:24:36.5424070Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:794:2: note: possible intended match here
2019-10-31T08:24:36.5424211Z  %2 = call <2 x i16> @llvm.ssub.sat.v2i16(<2 x i16> %x, <2 x i16> %y)
2019-10-31T08:24:36.5424654Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:456:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5424654Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:456:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5425021Z  // CHECK: %{{[0-9]+}} = call <4 x i16> @llvm.ssub.sat.v4i16(<4 x i16> %_3, <4 x i16> %_4)
2019-10-31T08:24:36.5425112Z            ^
2019-10-31T08:24:36.5425533Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:806:29: note: scanning from here
2019-10-31T08:24:36.5425677Z define <4 x i16> @ssub_i16x4(<4 x i16> %x, <4 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5425756Z                             ^
2019-10-31T08:24:36.5426436Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:811:2: note: possible intended match here
2019-10-31T08:24:36.5426567Z  %2 = call <4 x i16> @llvm.ssub.sat.v4i16(<4 x i16> %x, <4 x i16> %y)
2019-10-31T08:24:36.5427312Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:463:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5427312Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:463:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5427638Z  // CHECK: %{{[0-9]+}} = call <8 x i16> @llvm.ssub.sat.v8i16(<8 x i16> %_3, <8 x i16> %_4)
2019-10-31T08:24:36.5427737Z            ^
2019-10-31T08:24:36.5428328Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:823:29: note: scanning from here
2019-10-31T08:24:36.5428475Z define <8 x i16> @ssub_i16x8(<8 x i16> %x, <8 x i16> %y) unnamed_addr #0 {
2019-10-31T08:24:36.5428562Z                             ^
2019-10-31T08:24:36.5429216Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:828:2: note: possible intended match here
2019-10-31T08:24:36.5429373Z  %2 = call <8 x i16> @llvm.ssub.sat.v8i16(<8 x i16> %x, <8 x i16> %y)
2019-10-31T08:24:36.5430018Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:470:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5430018Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:470:12: error: CHECK: expected string not found in input
2019-10-31T08:24:36.5430347Z  // CHECK: %{{[0-9]+}} = call <16 x i16> @llvm.ssub.sat.v16i16(<16 x i16> %_3, <16 x i16> %_4)
2019-10-31T08:24:36.5430429Z            ^
2019-10-31T08:24:36.5430818Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.ll:840:31: note: scanning from here
---
2019-10-31T08:24:36.5839978Z test result: FAILED. 125 passed; 2 failed; 31 ignored; 0 measured; 0 filtered out
2019-10-31T08:24:36.5840065Z 
2019-10-31T08:24:36.5840098Z 
2019-10-31T08:24:36.5840130Z 
2019-10-31T08:24:36.5841908Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-31T08:24:36.5842623Z 
2019-10-31T08:24:36.5842660Z 
2019-10-31T08:24:36.5843036Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-31T08:24:36.5843218Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-31T08:24:36.5843218Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-31T08:24:36.5843709Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2019-10-31T08:24:36.5843876Z Build completed unsuccessfully in 2:09:48
2019-10-31T08:24:36.5843960Z == clock drift check ==
2019-10-31T08:24:36.5844028Z   local time: Thu Oct 31 08:24:36 UTC 2019
2019-10-31T08:24:36.7062456Z   network time: Thu, 31 Oct 2019 08:24:36 GMT
2019-10-31T08:24:36.7063175Z == end clock drift check ==
2019-10-31T08:24:40.1027073Z 
2019-10-31T08:24:40.1132133Z ##[error]Bash exited with code '1'.
2019-10-31T08:24:40.1191966Z ##[section]Starting: Checkout
2019-10-31T08:24:40.1194713Z ==============================================================================
2019-10-31T08:24:40.1194795Z Task         : Get sources
2019-10-31T08:24:40.1194894Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
