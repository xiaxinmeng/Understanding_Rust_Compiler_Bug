plain
failures:

---- [codegen] src/test/codegen/vec-in-place.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll" "/checkout/src/test/codegen/vec-in-place.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-in-place.rs:31:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:488:40: note: found here
 br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i
                                       ^~~~
/checkout/src/test/codegen/vec-in-place.rs:32:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:440:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !156)
      ^~~~
/checkout/src/test/codegen/vec-in-place.rs:39:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:632:40: note: found here
 br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i
                                       ^~~~
/checkout/src/test/codegen/vec-in-place.rs:40:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:584:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !190)
      ^~~~
/checkout/src/test/codegen/vec-in-place.rs:48:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:728:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !224)
      ^~~~
/checkout/src/test/codegen/vec-in-place.rs:55:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:903:40: note: found here
 br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i
                                       ^~~~
/checkout/src/test/codegen/vec-in-place.rs:56:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:863:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !267)
      ^~~~
/checkout/src/test/codegen/vec-in-place.rs:63:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:1019:40: note: found here
 br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i
                                       ^~~~
/checkout/src/test/codegen/vec-in-place.rs:64:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:979:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !292)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll
Check file: /checkout/src/test/codegen/vec-in-place.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
      435:  %_4.sroa.4.0..sroa_idx23 = getelementptr inbounds %"alloc::vec::Vec<i8>", %"alloc::vec::Vec<i8>"* %vec, i64 0, i32 0, i32 1 
      436:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx23, align 8 
      437:  %_4.sroa.5.0..sroa_idx25 = getelementptr inbounds %"alloc::vec::Vec<i8>", %"alloc::vec::Vec<i8>"* %vec, i64 0, i32 1 
      438:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx25, align 8 
      439:  %1 = getelementptr inbounds i8, i8* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload 
      440:  tail call void @llvm.experimental.noalias.scope.decl(metadata !156) 
not:32           !~~~                                                            error: no match expected
      441:  tail call void @llvm.experimental.noalias.scope.decl(metadata !159) 
      442:  %2 = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<i8>, [closure@/checkout/src/test/codegen/vec-in-place.rs:33:25: 33:36]>"* %_2.i.i to i8* 
      443:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !noalias !162 
      444:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<i8>, [closure@/checkout/src/test/codegen/vec-in-place.rs:33:25: 33:36]>"* %_2.i.i to i8** 
      445:  store i8* %_4.sroa.0.0.copyload, i8** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast, align 8, !noalias !156 
        .
        .
        .
      483:  resume { i8*, i32 } %eh.lpad-body.i.i.i 
      484:  
      485: bb8.i.i.i.i: ; preds = %bb14.i.i.i.i 
      486:  %9 = getelementptr inbounds i8, i8* %_4.sroa.0.0.copyload, i64 %18 
      487:  %_22.not.i.i.i.i = icmp sgt i64 %18, %_4.sroa.5.0.copyload 
      488:  br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i 
not:31                                            !~~~                                 error: no match expected
      489:  
      490: bb10.i.loopexit.i.i.i: ; preds = %bb8.i.i.i.i 
      491:  %10 = getelementptr inbounds i8, i8* %17, i64 1 
      492:  br label %bb10.i.i.i.i 
        .
        .
        .
        .
      579:  %_4.sroa.4.0..sroa_idx23 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vec, i64 0, i32 0, i32 1 
      580:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx23, align 8 
      581:  %_4.sroa.5.0..sroa_idx25 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vec, i64 0, i32 1 
      582:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx25, align 8 
      583:  %1 = getelementptr inbounds i8, i8* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload 
      584:  tail call void @llvm.experimental.noalias.scope.decl(metadata !190) 
not:40           !~~~                                                            error: no match expected
      585:  tail call void @llvm.experimental.noalias.scope.decl(metadata !193) 
      586:  %2 = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<u8>, [closure@/checkout/src/test/codegen/vec-in-place.rs:41:25: 41:39]>"* %_2.i.i to i8* 
      587:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !noalias !196 
      588:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<u8>, [closure@/checkout/src/test/codegen/vec-in-place.rs:41:25: 41:39]>"* %_2.i.i to i8** 
      589:  store i8* %_4.sroa.0.0.copyload, i8** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast, align 8, !noalias !190 
        .
        .
        .
      627:  resume { i8*, i32 } %eh.lpad-body.i.i.i 
      628:  
      629: bb8.i.i.i.i: ; preds = %bb14.i.i.i.i 
      630:  %9 = getelementptr inbounds i8, i8* %_4.sroa.0.0.copyload, i64 %18 
      631:  %_22.not.i.i.i.i = icmp sgt i64 %18, %_4.sroa.5.0.copyload 
      632:  br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i 
not:39                                            !~~~                                 error: no match expected
      633:  
      634: bb10.i.loopexit.i.i.i: ; preds = %bb8.i.i.i.i 
      635:  %10 = getelementptr inbounds i8, i8* %17, i64 1 
      636:  br label %bb10.i.i.i.i 
        .
        .
        .
        .
      723:  %_4.sroa.4.0..sroa_idx23 = getelementptr inbounds %"alloc::vec::Vec<Wrapper<u8>>", %"alloc::vec::Vec<Wrapper<u8>>"* %vec, i64 0, i32 0, i32 1 
      724:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx23, align 8 
      725:  %_4.sroa.5.0..sroa_idx25 = getelementptr inbounds %"alloc::vec::Vec<Wrapper<u8>>", %"alloc::vec::Vec<Wrapper<u8>>"* %vec, i64 0, i32 1 
      726:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx25, align 8 
      727:  %1 = getelementptr inbounds i8, i8* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload 
      728:  tail call void @llvm.experimental.noalias.scope.decl(metadata !224) 
not:48           !~~~                                                            error: no match expected
      729:  tail call void @llvm.experimental.noalias.scope.decl(metadata !227) 
      730:  %2 = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<Wrapper<u8>>, [closure@/checkout/src/test/codegen/vec-in-place.rs:49:25: 49:32]>"* %_2.i.i to i8* 
      731:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !noalias !230 
      732:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<Wrapper<u8>>, [closure@/checkout/src/test/codegen/vec-in-place.rs:49:25: 49:32]>"* %_2.i.i to i8** 
      733:  store i8* %_4.sroa.0.0.copyload, i8** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast, align 8, !noalias !224 
        .
        .
        .
      858:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx24, align 8 
      859:  %_4.sroa.5.0..sroa_idx26 = getelementptr inbounds %"alloc::vec::Vec<[u64; 4]>", %"alloc::vec::Vec<[u64; 4]>"* %vec, i64 0, i32 1 
      860:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx26, align 8 
      861:  %1 = getelementptr inbounds [4 x i64], [4 x i64]* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload 
      862:  %2 = getelementptr [4 x i64], [4 x i64]* %_4.sroa.0.0.copyload, i64 0, i64 0 
      863:  tail call void @llvm.experimental.noalias.scope.decl(metadata !267) 
not:56           !~~~                                                            error: no match expected
      864:  tail call void @llvm.experimental.noalias.scope.decl(metadata !270) 
      865:  %3 = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<[u64; 4]>, [closure@/checkout/src/test/codegen/vec-in-place.rs:57:25: 57:62]>"* %_2.i.i to i8* 
      866:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %3), !noalias !273 
      867:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<[u64; 4]>, [closure@/checkout/src/test/codegen/vec-in-place.rs:57:25: 57:62]>"* %_2.i.i to i64** 
      868:  store i64* %2, i64** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast, align 8, !noalias !267 
        .
        .
      898:  
      898:  
      899: bb8.i.i.i.i: ; preds = %bb14.i.i.i.i 
      900:  %10 = getelementptr inbounds [4 x i64], [4 x i64]* %_4.sroa.0.0.copyload, i64 %20 
      901:  %11 = bitcast [4 x i64]* %10 to %Foo* 
      902:  %_22.not.i.i.i.i = icmp sgt i64 %20, %_4.sroa.5.0.copyload 
      903:  br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i 
not:55                                            !~~~                                 error: no match expected
      904:  
      905: bb10.i.loopexit.i.i.i: ; preds = %bb8.i.i.i.i 
      906:  %12 = getelementptr inbounds %Foo, %Foo* %19, i64 1 
      907:  br label %bb10.i.i.i.i 
        .
        .
        .
        .
      974:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx24, align 8 
      975:  %_4.sroa.5.0..sroa_idx26 = getelementptr inbounds %"alloc::vec::Vec<Bar>", %"alloc::vec::Vec<Bar>"* %vec, i64 0, i32 1 
      976:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx26, align 8 
      977:  %1 = getelementptr inbounds %Bar, %Bar* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload 
      978:  %2 = getelementptr %Bar, %Bar* %_4.sroa.0.0.copyload, i64 0, i32 0 
      979:  tail call void @llvm.experimental.noalias.scope.decl(metadata !292) 
not:64           !~~~                                                            error: no match expected
      980:  tail call void @llvm.experimental.noalias.scope.decl(metadata !295) 
      981:  %3 = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<Bar>, [closure@/checkout/src/test/codegen/vec-in-place.rs:70:25: 70:62]>"* %_2.i.i to i8* 
      982:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %3), !noalias !298 
      983:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast = bitcast %"core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<Bar>, [closure@/checkout/src/test/codegen/vec-in-place.rs:70:25: 70:62]>"* %_2.i.i to i64** 
      984:  store i64* %2, i64** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_cast4.sroa_cast, align 8, !noalias !292 
        .
        .
     1014:  
     1014:  
     1015: bb8.i.i.i.i: ; preds = %bb14.i.i.i.i 
     1016:  %10 = getelementptr inbounds %Bar, %Bar* %_4.sroa.0.0.copyload, i64 %20 
     1017:  %11 = bitcast %Bar* %10 to [4 x i64]* 
     1018:  %_22.not.i.i.i.i = icmp sgt i64 %20, %_4.sroa.5.0.copyload 
     1019:  br i1 %_22.not.i.i.i.i, label %bb10.i.loopexit.i.i.i, label %bb14.i.i.i.i 
not:63                                            !~~~                                 error: no match expected
     1020:  
     1021: bb10.i.loopexit.i.i.i: ; preds = %bb8.i.i.i.i 
     1022:  %12 = getelementptr inbounds [4 x i64], [4 x i64]* %19, i64 1 
     1023:  br label %bb10.i.i.i.i 
        .
        .
        .
>>>>>>
