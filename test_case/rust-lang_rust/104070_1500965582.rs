plain
failures:

---- [codegen] tests/codegen/vec-shrink-panik.rs#old stdout ----

error in revision `old`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll" "/checkout/tests/codegen/vec-shrink-panik.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,old" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/vec-shrink-panik.rs:24:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:149:14: note: found here
; call core::panicking::panic_cannot_unwind
/checkout/tests/codegen/vec-shrink-panik.rs:39:16: error: CHECK-NOT: excluded string found in input
/checkout/tests/codegen/vec-shrink-panik.rs:39:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:255:14: note: found here
; call core::panicking::panic_cannot_unwind


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       49: ; Function Attrs: nonlazybind uwtable 
       50: define void @shrink_to_fit(ptr noalias nocapture noundef align 4 dereferenceable(12) %vec) unnamed_addr #0 { 
       51: start: 
       52:  tail call void @llvm.experimental.noalias.scope.decl(metadata !19) 
       53:  %0 = load i32, ptr %vec, align 4, !alias.scope !19, !noundef !10 
       54:  %1 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %vec, i32 0, i32 1 
       55:  %_5.i = load i32, ptr %1, align 4, !alias.scope !19, !noundef !10 
       56:  %_2.i = icmp ugt i32 %0, %_5.i 
       57:  br i1 %_2.i, label %bb5.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h67e1ca6677de7b42E.exit" 
       58:  
       59: bb5.i.i.i: ; preds = %start 
       60:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22) 
       61:  tail call void @llvm.experimental.noalias.scope.decl(metadata !25) 
       62:  %2 = shl nuw i32 %0, 2 
       63:  %3 = getelementptr inbounds { i32, ptr }, ptr %vec, i32 0, i32 1 
       64:  %self2.i.i.i.i = load ptr, ptr %3, align 4, !alias.scope !28, !noalias !31, !nonnull !10, !noundef !10 
       65:  %4 = icmp eq i32 %_5.i, 0 
       66:  br i1 %4, label %bb1.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i" 
       67:  
       68: bb1.i.i.i.i.i: ; preds = %bb5.i.i.i 
       69:  tail call void @__rust_dealloc(ptr noundef nonnull %self2.i.i.i.i, i32 noundef %2, i32 noundef 4) #10, !noalias !33 
       70:  br label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i" 
       71:  
       72: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i": ; preds = %bb5.i.i.i 
       73:  %5 = shl nuw i32 %_5.i, 2 
       74:  %_19.i.i.i.i = icmp ule i32 %5, %2 
       75:  tail call void @llvm.assume(i1 %_19.i.i.i.i) 
       76:  %raw_ptr.i.i.i.i = tail call noundef align 4 ptr @__rust_realloc(ptr noundef nonnull %self2.i.i.i.i, i32 noundef %2, i32 noundef 4, i32 noundef %5) #10, !noalias !33 
       77:  %6 = icmp eq ptr %raw_ptr.i.i.i.i, null 
       78:  br i1 %6, label %bb6.i.i.i, label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i" 
       79:  
       80: bb6.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i" 
       81: ; call alloc::alloc::handle_alloc_error 
       82:  tail call void @_ZN5alloc5alloc18handle_alloc_error17hbd9ecf4128969175E(i32 noundef %5, i32 noundef 4) #11, !noalias !34 
       83:  unreachable 
       84:  
       85: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i", %bb1.i.i.i.i.i 
       86:  %.sroa.0.0.i22.i.i.i = phi ptr [ inttoptr (i32 4 to ptr), %bb1.i.i.i.i.i ], [ %raw_ptr.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i" ] 
       87:  store ptr %.sroa.0.0.i22.i.i.i, ptr %3, align 4, !alias.scope !35 
       88:  store i32 %_5.i, ptr %vec, align 4, !alias.scope !35 
       89:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h67e1ca6677de7b42E.exit" 
       90:  
       91: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h67e1ca6677de7b42E.exit": ; preds = %start, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i" 
       92:  ret void 
       93: } 
       94:  
       95: ; Function Attrs: nonlazybind uwtable 
       96: define { ptr, i32 } @issue71861(ptr noalias nocapture noundef dereferenceable(12) %vec) unnamed_addr #0 personality ptr @rust_eh_personality { 
       97: start: 
       98:  tail call void @llvm.experimental.noalias.scope.decl(metadata !38) 
       99:  tail call void @llvm.experimental.noalias.scope.decl(metadata !41) 
      100:  %0 = load i32, ptr %vec, align 4, !alias.scope !44, !noundef !10 
      101:  %1 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %vec, i32 0, i32 1 
      102:  %_5.i.i = load i32, ptr %1, align 4, !alias.scope !44, !noundef !10 
      103:  %_2.i.i = icmp ugt i32 %0, %_5.i.i 
      104:  br i1 %_2.i.i, label %bb5.i.i.i.i, label %start.bb3_crit_edge.i 
      105:  
      106: start.bb3_crit_edge.i: ; preds = %start 
      107:  %value.sroa.4.0.self.sroa_idx.phi.trans.insert.i = getelementptr inbounds i8, ptr %vec, i32 4 
      108:  %value.sroa.4.0.copyload.pre.i = load ptr, ptr %value.sroa.4.0.self.sroa_idx.phi.trans.insert.i, align 4, !alias.scope !38 
      109:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h2b99c5b973599af2E.exit" 
      110:  
      111: bb5.i.i.i.i: ; preds = %start 
      112:  tail call void @llvm.experimental.noalias.scope.decl(metadata !45) 
      113:  tail call void @llvm.experimental.noalias.scope.decl(metadata !48) 
      114:  %2 = shl nuw i32 %0, 2 
      115:  %3 = getelementptr inbounds { i32, ptr }, ptr %vec, i32 0, i32 1 
      116:  %self2.i.i.i.i.i = load ptr, ptr %3, align 4, !alias.scope !51, !noalias !54, !nonnull !10, !noundef !10 
      117:  %4 = icmp eq i32 %_5.i.i, 0 
      118:  br i1 %4, label %bb1.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i" 
      119:  
      120: bb1.i.i.i.i.i.i: ; preds = %bb5.i.i.i.i 
      121:  tail call void @__rust_dealloc(ptr noundef nonnull %self2.i.i.i.i.i, i32 noundef %2, i32 noundef 4) #10, !noalias !56 
      122:  br label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i.i" 
      123:  
      124: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i": ; preds = %bb5.i.i.i.i 
      125:  %5 = shl nuw i32 %_5.i.i, 2 
      126:  %_19.i.i.i.i.i = icmp ule i32 %5, %2 
      127:  tail call void @llvm.assume(i1 %_19.i.i.i.i.i) 
      128:  %raw_ptr.i.i.i.i.i = tail call noundef align 4 ptr @__rust_realloc(ptr noundef nonnull %self2.i.i.i.i.i, i32 noundef %2, i32 noundef 4, i32 noundef %5) #10, !noalias !56 
      129:  %6 = icmp eq ptr %raw_ptr.i.i.i.i.i, null 
      130:  br i1 %6, label %bb6.i.i.i.i, label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i.i" 
      131:  
      132: bb6.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i" 
      133: ; invoke alloc::alloc::handle_alloc_error 
      134:  invoke void @_ZN5alloc5alloc18handle_alloc_error17hbd9ecf4128969175E(i32 noundef %5, i32 noundef 4) #11 
      135:  to label %.noexc.i unwind label %bb7.i, !noalias !38 
      136:  
      137: .noexc.i: ; preds = %bb6.i.i.i.i 
      138:  unreachable 
      139:  
      140: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i", %bb1.i.i.i.i.i.i 
      141:  %.sroa.0.0.i22.i.i.i.i = phi ptr [ inttoptr (i32 4 to ptr), %bb1.i.i.i.i.i.i ], [ %raw_ptr.i.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i" ] 
      142:  store ptr %.sroa.0.0.i22.i.i.i.i, ptr %3, align 4, !alias.scope !57 
      143:  store i32 %_5.i.i, ptr %vec, align 4, !alias.scope !57 
      144:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h2b99c5b973599af2E.exit" 
      145:  
      146: terminate.i: ; preds = %bb7.i 
      147:  %7 = landingpad { ptr, i32 } 
      148:  filter [0 x ptr] zeroinitializer 
      149: ; call core::panicking::panic_cannot_unwind 
not:24                  !~~~~                           error: no match expected
      150:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h429a64d56bea4f5fE() #12, !noalias !38 
      151:  unreachable 
      152:  
      153: bb4.i: ; preds = %bb7.i 
      154:  resume { ptr, i32 } %8 
      155:  
      156: bb7.i: ; preds = %bb6.i.i.i.i 
      157:  %8 = landingpad { ptr, i32 } 
      158:  cleanup 
      159: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<u32>> 
      160:  invoke fastcc void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u32$GT$$GT$17h5f1e5dab66777dfdE"(ptr noundef nonnull %vec) #13 
      161:  to label %bb4.i unwind label %terminate.i 
      162:  
      163: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h2b99c5b973599af2E.exit": ; preds = %start.bb3_crit_edge.i, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i.i" 
      164:  %value.sroa.4.0.copyload.i = phi ptr [ %value.sroa.4.0.copyload.pre.i, %start.bb3_crit_edge.i ], [ %.sroa.0.0.i22.i.i.i.i, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E.exit.i.i" ] 
      165:  %9 = insertvalue { ptr, i32 } poison, ptr %value.sroa.4.0.copyload.i, 0 
      166:  %10 = insertvalue { ptr, i32 } %9, i32 %_5.i.i, 1 
      167:  ret { ptr, i32 } %10 
      168: } 
      169:  
      170: ; Function Attrs: nonlazybind uwtable 
      171: define { ptr, i32 } @issue75636(ptr noalias noundef nonnull readonly align 4 %iter.0, i32 noundef %iter.1) unnamed_addr #0 personality ptr @rust_eh_personality { 
      172: start: 
      173:  %_2.i = alloca %"alloc::vec::Vec<&str>", align 4 
      174:  %0 = getelementptr inbounds { ptr, i32 }, ptr %iter.0, i32 %iter.1 
      175:  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_2.i) 
      176:  tail call void @llvm.experimental.noalias.scope.decl(metadata !60) 
      177:  tail call void @llvm.experimental.noalias.scope.decl(metadata !63) 
      178:  tail call void @llvm.experimental.noalias.scope.decl(metadata !66) 
      179:  tail call void @llvm.experimental.noalias.scope.decl(metadata !69) 
      180:  %.idx = shl nuw nsw i32 %iter.1, 3 
      181:  %_5.i.i.i.i.i.i = icmp eq i32 %iter.1, 0 
      182:  br i1 %_5.i.i.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h0e72820030fea05aE.exit", label %bb6.i.i.i.i.i.i 
      183:  
      184: bb6.i.i.i.i.i.i: ; preds = %start 
      185:  %_5.i.i.i.i.i.i.i = icmp sgt i32 %iter.1, -1 
      186:  %1 = lshr i32 %iter.1, 26 
      187:  %2 = and i32 %1, 4 
      188:  %.sroa.2.0.i.i.i.i.i.i.i = xor i32 %2, 4 
      189:  br i1 %_5.i.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5c250dce01b93de2E.exit.i.i.i.i.i.i", label %bb7.i.i.i.i.i.i 
      190:  
      191: bb7.i.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i 
      192: ; call alloc::raw_vec::capacity_overflow 
      193:  tail call void @_ZN5alloc7raw_vec17capacity_overflow17h3b8b3503cdbb56aaE() #11, !noalias !72 
      194:  unreachable 
      195:  
      196: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5c250dce01b93de2E.exit.i.i.i.i.i.i": ; preds = %bb6.i.i.i.i.i.i 
      197:  %3 = tail call noundef ptr @__rust_alloc(i32 noundef %.idx, i32 noundef %.sroa.2.0.i.i.i.i.i.i.i) #10, !noalias !72 
      198:  %4 = icmp eq ptr %3, null 
      199:  br i1 %4, label %bb17.i.i.i.i.i.i, label %bb3.i.i.i.i.i.preheader.i.i.i.i.i 
      200:  
      201: bb17.i.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5c250dce01b93de2E.exit.i.i.i.i.i.i" 
      202: ; call alloc::alloc::handle_alloc_error 
      203:  tail call void @_ZN5alloc5alloc18handle_alloc_error17hbd9ecf4128969175E(i32 noundef %.idx, i32 noundef %.sroa.2.0.i.i.i.i.i.i.i) #11, !noalias !72 
      204:  unreachable 
      205:  
      206: bb3.i.i.i.i.i.preheader.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h5c250dce01b93de2E.exit.i.i.i.i.i.i" 
      207:  store i32 %iter.1, ptr %_2.i, align 4, !alias.scope !72 
      208:  %5 = getelementptr inbounds { i32, ptr }, ptr %_2.i, i32 0, i32 1 
      209:  store ptr %3, ptr %5, align 4, !alias.scope !72 
      210:  tail call void @llvm.experimental.noalias.scope.decl(metadata !73) 
      211:  tail call void @llvm.experimental.noalias.scope.decl(metadata !76) 
      212:  br label %bb3.i.i.i.i.i.i.i.i.i.i 
      213:  
      214: bb3.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.preheader.i.i.i.i.i 
      215:  %6 = phi i32 [ %10, %bb3.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb3.i.i.i.i.i.preheader.i.i.i.i.i ] 
      216:  %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i = phi ptr [ %7, %bb3.i.i.i.i.i.i.i.i.i.i ], [ %iter.0, %bb3.i.i.i.i.i.preheader.i.i.i.i.i ] 
      217:  %7 = getelementptr inbounds { ptr, i32 }, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, i32 1 
      218:  tail call void @llvm.experimental.noalias.scope.decl(metadata !79) 
      219:  %elt.0.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, align 4, !alias.scope !79, !noalias !82, !nonnull !10, !align !90, !noundef !10 
      220:  %8 = getelementptr inbounds { ptr, i32 }, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, i32 0, i32 1 
      221:  %elt.1.i.i.i.i.i.i.i.i.i.i.i = load i32, ptr %8, align 4, !alias.scope !79, !noalias !82, !noundef !10 
      222:  %9 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 %6 
      223:  store ptr %elt.0.i.i.i.i.i.i.i.i.i.i.i, ptr %9, align 4, !noalias !91 
      224:  %src.sroa.4.0._10.sroa_idx.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %9, i32 4 
      225:  store i32 %elt.1.i.i.i.i.i.i.i.i.i.i.i, ptr %src.sroa.4.0._10.sroa_idx.i.i.i.i.i.i.i.i.i.i.i.i.i, align 4, !noalias !98 
      226:  %10 = add nuw nsw i32 %6, 1 
      227:  %_10.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %7, %0 
      228:  br i1 %_10.i.i.i.i.i.i.i.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i, label %bb3.i.i.i.i.i.i.i.i.i.i 
      229:  
      230: _ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i 
      231:  %11 = getelementptr inbounds %"alloc::vec::Vec<&str>", ptr %_2.i, i32 0, i32 1 
      232:  store i32 %10, ptr %11, align 4, !alias.scope !99, !noalias !100 
      233:  %_2.i.i.i = icmp ult i32 %10, %iter.1 
      234:  br i1 %_2.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i", label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h0e72820030fea05aE.exit" 
      235:  
      236: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i": ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i 
      237:  %12 = shl nuw i32 %10, 3 
      238:  %_19.i.i.i.i.i.i = icmp ule i32 %12, %.idx 
      239:  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i) 
      240:  %raw_ptr.i.i.i.i.i.i = tail call noundef align 4 ptr @__rust_realloc(ptr noundef nonnull %3, i32 noundef %.idx, i32 noundef 4, i32 noundef %12) #10, !noalias !103 
      241:  %13 = icmp eq ptr %raw_ptr.i.i.i.i.i.i, null 
      242:  br i1 %13, label %bb6.i.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h0e72820030fea05aE.exit" 
      243:  
      244: bb6.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i" 
      245: ; invoke alloc::alloc::handle_alloc_error 
      246:  invoke void @_ZN5alloc5alloc18handle_alloc_error17hbd9ecf4128969175E(i32 noundef %12, i32 noundef 4) #11 
      247:  to label %.noexc.i.i unwind label %bb7.i.i, !noalias !112 
      248:  
      249: .noexc.i.i: ; preds = %bb6.i.i.i.i.i 
      250:  unreachable 
      251:  
      252: terminate.i.i: ; preds = %bb7.i.i 
      253:  %14 = landingpad { ptr, i32 } 
      254:  filter [0 x ptr] zeroinitializer 
      255: ; call core::panicking::panic_cannot_unwind 
not:39                  !~~~~                           error: no match expected
      256:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h429a64d56bea4f5fE() #12, !noalias !112 
      257:  unreachable 
      258:  
      259: bb4.i.i: ; preds = %bb7.i.i 
      260:  resume { ptr, i32 } %15 
      261:  
      262: bb7.i.i: ; preds = %bb6.i.i.i.i.i 
      263:  %15 = landingpad { ptr, i32 } 
      264:  cleanup 
      265: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<&str>> 
      266:  invoke fastcc void @"_ZN4core3ptr51drop_in_place$LT$alloc..vec..Vec$LT$$RF$str$GT$$GT$17h72d0d4d8bd86a291E"(ptr noundef nonnull %_2.i) #13 
      267:  to label %bb4.i.i unwind label %terminate.i.i 
      268:  
      269: "_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h0e72820030fea05aE.exit": ; preds = %start, %_ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i" 
      270:  %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i4.i = phi i32 [ %10, %_ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i ], [ %10, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i" ], [ 0, %start ] 
      271:  %value.sroa.4.0.copyload.i.i = phi ptr [ %3, %_ZN4core4iter6traits8iterator8Iterator7collect17hb2a0c9a9ae9f0a20E.exit.i ], [ %raw_ptr.i.i.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hda1e8e83482f2de5E.exit.i.i.i.i.i" ], [ inttoptr (i32 4 to ptr), %start ] 
      272:  %16 = insertvalue { ptr, i32 } poison, ptr %value.sroa.4.0.copyload.i.i, 0 
      273:  %17 = insertvalue { ptr, i32 } %16, i32 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i4.i, 1 
      274:  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_2.i) 
      275:  ret { ptr, i32 } %17 
      276: } 
      277:  
      278: ; Function Attrs: nonlazybind uwtable 
      279: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0 
      281: ; core::panicking::panic_cannot_unwind 
      281: ; core::panicking::panic_cannot_unwind 
      282: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
      283: declare void @_ZN4core9panicking19panic_cannot_unwind17h429a64d56bea4f5fE() unnamed_addr #1 
      284:  
      285: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) 
      286: declare void @llvm.assume(i1 noundef) #2 
      287:  
      288: ; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable 
      289: declare noalias noundef ptr @__rust_alloc(i32 noundef, i32 allocalign noundef) unnamed_addr #3 
      290:  
      291: ; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable 
      292: declare noalias noundef ptr @__rust_realloc(ptr allocptr noundef, i32 noundef, i32 allocalign noundef, i32 noundef) unnamed_addr #4 
      294: ; alloc::alloc::handle_alloc_error 
      294: ; alloc::alloc::handle_alloc_error 
      295: ; Function Attrs: cold noreturn nonlazybind uwtable 
      296: declare void @_ZN5alloc5alloc18handle_alloc_error17hbd9ecf4128969175E(i32 noundef, i32 noundef) unnamed_addr #5 
      298: ; alloc::raw_vec::capacity_overflow 
      298: ; alloc::raw_vec::capacity_overflow 
      299: ; Function Attrs: noreturn nonlazybind uwtable 
      300: declare void @_ZN5alloc7raw_vec17capacity_overflow17h3b8b3503cdbb56aaE() unnamed_addr #6 
      301:  
      302: ; Function Attrs: nounwind nonlazybind allockind("free") uwtable 
      303: declare void @__rust_dealloc(ptr allocptr noundef, i32 noundef, i32 noundef) unnamed_addr #7 
      304:  
      305: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
      306: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #8 
      307:  
      308: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
      309: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #8 
      310:  
      311: ; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) 
      312: declare void @llvm.experimental.noalias.scope.decl(metadata) #9 
      313:  
      314: attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      315: attributes #1 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      316: attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) } 
      317: attributes #3 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      318: attributes #4 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      319: attributes #5 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      320: attributes #6 = { noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      321: attributes #7 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="pentium" } 
      322: attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) } 
      323: attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) } 
      324: attributes #10 = { nounwind } 
      325: attributes #11 = { noreturn } 
      326: attributes #12 = { noinline noreturn nounwind } 
      327: attributes #13 = { noinline } 
      328:  
      329: !llvm.module.flags = !{!0, !1} 
      330:  
      331: !0 = !{i32 8, !"PIC Level", i32 2} 
      332: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
      333: !2 = !{!3} 
      334: !3 = distinct !{!3, !4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8669caae85c35ecfE: %self"} 
      335: !4 = distinct !{!4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8669caae85c35ecfE"} 
      336: !5 = !{!6, !3} 
      337: !6 = distinct !{!6, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb7af4f0b618dcef1E: %self"} 
      338: !7 = distinct !{!7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb7af4f0b618dcef1E"} 
      339: !8 = !{!9} 
      340: !9 = distinct !{!9, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb7af4f0b618dcef1E: argument 0"} 
      341: !10 = !{} 
      342: !11 = !{!12} 
      343: !12 = distinct !{!12, !13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcb6e6539168d9e34E: %self"} 
      344: !13 = distinct !{!13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcb6e6539168d9e34E"} 
      345: !14 = !{!15, !12} 
      346: !15 = distinct !{!15, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf747c7b3c16603c4E: %self"} 
      347: !16 = distinct !{!16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf747c7b3c16603c4E"} 
      348: !17 = !{!18} 
      349: !18 = distinct !{!18, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf747c7b3c16603c4E: argument 0"} 
      350: !19 = !{!20} 
      351: !20 = distinct !{!20, !21, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h67e1ca6677de7b42E: %self"} 
      352: !21 = distinct !{!21, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h67e1ca6677de7b42E"} 
      353: !22 = !{!23} 
      354: !23 = distinct !{!23, !24, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E: %self"} 
      355: !24 = distinct !{!24, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17hc18e3324fcbb32c5E"} 
        .
        .
>>>>>>
------------------------------------------
