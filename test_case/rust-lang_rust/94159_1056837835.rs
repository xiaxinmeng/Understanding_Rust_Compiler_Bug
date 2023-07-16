plain
Some tests failed in compiletest suite=codegen mode=codegen host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu

---- [codegen] codegen/loads.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/loads.rs:38:11: error: CHECK: expected string not found in input
// CHECK: load %Align16*, %Align16** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_16_META:[0-9]+]], !noundef !{{[0-9]+}}
          ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:66:77: note: scanning from here
define noundef align 16 dereferenceable(16) i128* @load_ref_higher_alignment(i128** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 {
                                                                            ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:66:77: note: with "PTR_ALIGNMENT" equal to "8"
define noundef align 16 dereferenceable(16) i128* @load_ref_higher_alignment(i128** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 {
                                                                            ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:68:7: note: possible intended match here
 %0 = load i128*, i128** %x, align 8, !nonnull !2, !align !4, !noundef !2
      ^
/checkout/src/test/codegen/loads.rs:46:94: error: undefined variable: ALIGN_16_META
// CHECK: load i64*, i64** %{{.+}}, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_16_META]], !noundef !{{[0-9]+}}
                                                                                             ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:76:72: note: with "PTR_ALIGNMENT" equal to "8"
 %1 = load i32*, i32** %0, align 8, !nonnull !2, !align !3, !noundef !2
                                                                       ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:78:7: note: possible intended match here
 %3 = load i64*, i64** %2, align 8, !nonnull !2, !align !4, !noundef !2
      ^
/checkout/src/test/codegen/loads.rs:152:18: error: undefined variable: ALIGN_16_META
// CHECK-DAG: ![[ALIGN_16_META]] = !{i64 16}
                 ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll:336:31: note: possible intended match here
 call void @llvm.lifetime.start.p0i8(i64 16, i8* %5)

Input file: /checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/loads/loads.ll
Check file: /checkout/src/test/codegen/loads.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           61:  %0 = load i32*, i32** %x, align 8, !nonnull !2, !align !3, !noundef !2 
           62:  ret i32* %0 
           63: } 
           64:  
           65: ; Function Attrs: nonlazybind uwtable 
           66: define noundef align 16 dereferenceable(16) i128* @load_ref_higher_alignment(i128** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 { 
check:38'0                                                                                 X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:38'1                                                                                                                                                                    with "PTR_ALIGNMENT" equal to "8"
           67: start: 
check:38'0     ~~~~~~~
           68:  %0 = load i128*, i128** %x, align 8, !nonnull !2, !align !4, !noundef !2 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'2           ?                                                                    possible intended match
           69:  ret i128* %0 
check:38'0     ~~~~~~~~~~~~~~
           70: } 
check:38'0     ~~
           71:  
check:38'0     ~
           72: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73: define { i32*, i64* } @load_scalar_pair({ i32*, i64* }* noalias noundef readonly align 8 dereferenceable(16) %x) unnamed_addr #0 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74: start: 
           75:  %0 = getelementptr inbounds { i32*, i64* }, { i32*, i64* }* %x, i32 0, i32 0 
           76:  %1 = load i32*, i32** %0, align 8, !nonnull !2, !align !3, !noundef !2 
check:46'0                                                                            X error: match failed for invalid pattern
check:46'1                                                                              undefined variable: ALIGN_16_META
check:46'2                                                                              with "PTR_ALIGNMENT" equal to "8"
           77:  %2 = getelementptr inbounds { i32*, i64* }, { i32*, i64* }* %x, i32 0, i32 1 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78:  %3 = load i64*, i64** %2, align 8, !nonnull !2, !align !4, !noundef !2 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'3           ?                                                                  possible intended match
           79:  %4 = insertvalue { i32*, i64* } undef, i32* %1, 0 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  %5 = insertvalue { i32*, i64* } %4, i64* %3, 1 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  ret { i32*, i64* } %5 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~
           82: } 
check:46'0     ~~
           83:  
check:46'0     ~
            .
            .
            .
          240:  %7 = bitcast %Bytes* %1 to i8* 
          241:  %8 = bitcast %Bytes* %x to i8* 
          242:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 4, i1 false) 
          243:  %9 = bitcast %Bytes* %1 to i32* 
          244:  %10 = load i32, i32* %9, align 1 
          245:  ret i32 %10 
dag:152'0                  X error: match failed for invalid pattern
dag:152'1                    undefined variable: ALIGN_16_META
          246: } 
dag:152'0      ~~
          247:  
dag:152'0      ~
          248: ; alloc::alloc::dealloc 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~
          249: ; Function Attrs: inlinehint nonlazybind uwtable 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          250: define internal void @_ZN5alloc5alloc7dealloc17h6df39be2a29a2e7fE(i8* %ptr, i64 %0, i64 noundef %1) unnamed_addr #1 { 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
          331:  
          331:  
dag:152'0      ~
          332: cleanup: ; preds = %bb7, %bb6, %bb5, %bb4, %bb2, %start 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          333:  %4 = landingpad { i8*, i32 } 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          334:  cleanup 
dag:152'0      ~~~~~~~~~
          335:  %5 = bitcast { i8*, i32 }* %3 to i8* 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          336:  call void @llvm.lifetime.start.p0i8(i64 16, i8* %5) 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:152'2                                    ?                       possible intended match
          337:  %6 = extractvalue { i8*, i32 } %4, 0 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          338:  %7 = extractvalue { i8*, i32 } %4, 1 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          339:  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %3, i32 0, i32 0 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          340:  store i8* %6, i8** %8, align 8 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          341:  %9 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %3, i32 0, i32 1 
dag:152'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
