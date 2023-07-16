plain
failures:

---- [codegen] src/test/codegen/vec-shrink-panik.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll" "/checkout/src/test/codegen/vec-shrink-panik.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-shrink-panik.rs:24:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: ; call core::panicking::panic_no_unwind
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:157:9: note: scanning from here
        ^
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:158:1: note: possible intended match here
; call core::panicking::panic_cannot_unwind
^
/checkout/src/test/codegen/vec-shrink-panik.rs:39:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: ; call core::panicking::panic_no_unwind
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:266:9: note: scanning from here
        ^
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:267:1: note: possible intended match here
; call core::panicking::panic_cannot_unwind

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll
Check file: /checkout/src/test/codegen/vec-shrink-panik.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          57:  %0 = load i32, ptr %vec, align 4, !alias.scope !19 
          58:  %1 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %vec, i32 0, i32 1 
          59:  %_5.i = load i32, ptr %1, align 4, !alias.scope !19 
          60:  %_2.i = icmp ugt i32 %0, %_5.i 
          61:  br i1 %_2.i, label %bb5.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h97f417373fb40ae5E.exit" 
          62:  
          63: bb5.i.i.i: ; preds = %start 
          64:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22) 
          65:  tail call void @llvm.experimental.noalias.scope.decl(metadata !25) 
          66:  %_6.i.i.i.i.i = icmp ult i32 %0, 536870912 
          67:  %array_size.i.i.i.i.i = shl nuw nsw i32 %0, 2 
          68:  tail call void @llvm.assume(i1 %_6.i.i.i.i.i) 
          69:  %2 = getelementptr inbounds { i32, ptr }, ptr %vec, i32 0, i32 1 
          70:  %self1.i.i.i.i = load ptr, ptr %2, align 4, !alias.scope !28, !noalias !31, !nonnull !10, !noundef !10 
          71:  %_6.i.i.i.i = icmp ult i32 %_5.i, 536870912 
          72:  tail call void @llvm.assume(i1 %_6.i.i.i.i) 
          73:  %3 = icmp eq i32 %_5.i, 0 
          74:  br i1 %3, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i" 
          75:  
          76: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i": ; preds = %bb5.i.i.i 
          77:  tail call void @__rust_dealloc(ptr nonnull %self1.i.i.i.i, i32 %array_size.i.i.i.i.i, i32 4) #11, !noalias !33 
          78:  br label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17ha81ee10e2bf91daaE.exit.i" 
          79:  
          80: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i": ; preds = %bb5.i.i.i 
          81:  %array_size.i.i.i.i = shl nuw nsw i32 %_5.i, 2 
          82:  %_23.i.i.i.i = icmp ule i32 %array_size.i.i.i.i, %array_size.i.i.i.i.i 
          83:  tail call void @llvm.assume(i1 %_23.i.i.i.i) 
          84:  %raw_ptr.i.i.i.i = tail call align 4 ptr @__rust_realloc(ptr nonnull %self1.i.i.i.i, i32 %array_size.i.i.i.i.i, i32 4, i32 %array_size.i.i.i.i) #11, !noalias !33 
          85:  %4 = icmp eq ptr %raw_ptr.i.i.i.i, null 
          86:  br i1 %4, label %bb6.i.i.i, label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17ha81ee10e2bf91daaE.exit.i" 
          87:  
          88: bb6.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i" 
          89: ; call alloc::alloc::handle_alloc_error 
          90:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h6fe90bebe1e7d8f1E(i32 %array_size.i.i.i.i, i32 noundef 4) #12, !noalias !34 
          91:  unreachable 
          92:  
          93: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17ha81ee10e2bf91daaE.exit.i": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i", %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i" 
          94:  %.sroa.0.0.i33.i.i.i = phi ptr [ inttoptr (i32 4 to ptr), %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i" ], [ %raw_ptr.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i" ] 
          95:  store ptr %.sroa.0.0.i33.i.i.i, ptr %2, align 4, !alias.scope !35 
          96:  store i32 %_5.i, ptr %vec, align 4, !alias.scope !35 
          97:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h97f417373fb40ae5E.exit" 
          98:  
          99: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h97f417373fb40ae5E.exit": ; preds = %start, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$13shrink_to_fit17ha81ee10e2bf91daaE.exit.i" 
         100:  ret void 
         101: } 
         102:  
         103: ; Function Attrs: nonlazybind uwtable 
         104: define { ptr, i32 } @issue71861(ptr noalias nocapture noundef readonly dereferenceable(12) %vec) unnamed_addr #0 personality ptr @rust_eh_personality { 
         105: start: 
         106:  %_2 = alloca %"alloc::vec::Vec<u32>", align 4 
         107:  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_2) 
         108:  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %_2, ptr noundef nonnull align 4 dereferenceable(12) %vec, i32 12, i1 false) 
         109:  tail call void @llvm.experimental.noalias.scope.decl(metadata !38) 
         110:  tail call void @llvm.experimental.noalias.scope.decl(metadata !41) 
         111:  %0 = load i32, ptr %_2, align 4, !alias.scope !44 
         112:  %1 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %_2, i32 0, i32 1 
         113:  %_5.i.i = load i32, ptr %1, align 4, !alias.scope !44 
         114:  %_2.i.i = icmp ugt i32 %0, %_5.i.i 
         115:  br i1 %_2.i.i, label %bb5.i.i.i.i, label %start.bb3_crit_edge.i 
         116:  
         117: start.bb3_crit_edge.i: ; preds = %start 
         118:  %value.sroa.0.sroa.4.0.self.sroa_idx.phi.trans.insert.i = getelementptr inbounds i8, ptr %_2, i32 4 
         119:  %value.sroa.0.sroa.4.0.copyload.pre.i = load ptr, ptr %value.sroa.0.sroa.4.0.self.sroa_idx.phi.trans.insert.i, align 4, !alias.scope !38 
         120:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h31568ee242583beeE.exit" 
         121:  
         122: bb5.i.i.i.i: ; preds = %start 
         123:  tail call void @llvm.experimental.noalias.scope.decl(metadata !45) 
         124:  tail call void @llvm.experimental.noalias.scope.decl(metadata !48) 
         125:  %_6.i.i.i.i.i.i = icmp ult i32 %0, 536870912 
         126:  %array_size.i.i.i.i.i.i = shl nuw nsw i32 %0, 2 
         127:  tail call void @llvm.assume(i1 %_6.i.i.i.i.i.i) 
         128:  %2 = getelementptr inbounds { i32, ptr }, ptr %_2, i32 0, i32 1 
         129:  %self1.i.i.i.i.i = load ptr, ptr %2, align 4, !alias.scope !51, !noalias !54, !nonnull !10, !noundef !10 
         130:  %_6.i.i.i.i.i = icmp ult i32 %_5.i.i, 536870912 
         131:  tail call void @llvm.assume(i1 %_6.i.i.i.i.i) 
         132:  %3 = icmp eq i32 %_5.i.i, 0 
         133:  br i1 %3, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i.i", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i" 
         134:  
         135: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i.i": ; preds = %bb5.i.i.i.i 
         136:  tail call void @__rust_dealloc(ptr nonnull %self1.i.i.i.i.i, i32 %array_size.i.i.i.i.i.i, i32 4) #11, !noalias !56 
         137:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h31568ee242583beeE.exit" 
         138:  
         139: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i": ; preds = %bb5.i.i.i.i 
         140:  %array_size.i.i.i.i.i = shl nuw nsw i32 %_5.i.i, 2 
         141:  %_23.i.i.i.i.i = icmp ule i32 %array_size.i.i.i.i.i, %array_size.i.i.i.i.i.i 
         142:  tail call void @llvm.assume(i1 %_23.i.i.i.i.i) 
         143:  %raw_ptr.i.i.i.i.i = tail call align 4 ptr @__rust_realloc(ptr nonnull %self1.i.i.i.i.i, i32 %array_size.i.i.i.i.i.i, i32 4, i32 %array_size.i.i.i.i.i) #11, !noalias !56 
         144:  %4 = icmp eq ptr %raw_ptr.i.i.i.i.i, null 
         145:  br i1 %4, label %bb6.i.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h31568ee242583beeE.exit" 
         146:  
         147: bb6.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i" 
         148: ; invoke alloc::alloc::handle_alloc_error 
         149:  invoke void @_ZN5alloc5alloc18handle_alloc_error17h6fe90bebe1e7d8f1E(i32 %array_size.i.i.i.i.i, i32 noundef 4) #12 
         150:  to label %.noexc.i unwind label %bb7.i, !noalias !38 
         151:  
         152: .noexc.i: ; preds = %bb6.i.i.i.i 
         153:  unreachable 
         154:  
         155: abort.i: ; preds = %bb7.i 
         156:  %5 = landingpad { ptr, i32 } 
         157:  cleanup 
next:24'0             X error: no match found
         158: ; call core::panicking::panic_cannot_unwind 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:24'1     ?                                            possible intended match
         159:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h9eae876adce0cb19E() #13, !noalias !38 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         160:  unreachable 
next:24'0     ~~~~~~~~~~~~~
         161:  
next:24'0     ~
         162: bb4.i: ; preds = %bb7.i 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~
         163:  resume { ptr, i32 } %6 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~
         164:  
next:24'0     ~
         165: bb7.i: ; preds = %bb6.i.i.i.i 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         166:  %6 = landingpad { ptr, i32 } 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         167:  cleanup 
next:24'0     ~~~~~~~~~
         168: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<u32>> 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         169:  invoke fastcc void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u32$GT$$GT$17hb4a2df33123779c7E"(ptr nonnull %_2) #14 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         170:  to label %bb4.i unwind label %abort.i 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         171:  
next:24'0     ~
         172: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h31568ee242583beeE.exit": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i.i", %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i", %start.bb3_crit_edge.i 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         173:  %value.sroa.0.sroa.4.0.copyload.i = phi ptr [ %value.sroa.0.sroa.4.0.copyload.pre.i, %start.bb3_crit_edge.i ], [ inttoptr (i32 4 to ptr), %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.thread.i.i.i.i" ], [ %raw_ptr.i.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i" ] 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         174:  %7 = insertvalue { ptr, i32 } undef, ptr %value.sroa.0.sroa.4.0.copyload.i, 0 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         175:  %8 = insertvalue { ptr, i32 } %7, i32 %_5.i.i, 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         176:  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_2) 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         177:  ret { ptr, i32 } %8 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~
         178: } 
next:24'0     ~~
         179:  
next:24'0     ~
         180: ; Function Attrs: nonlazybind uwtable 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         181: define { ptr, i32 } @issue75636(ptr noalias noundef nonnull readonly align 4 %iter.0, i32 %iter.1) unnamed_addr #0 personality ptr @rust_eh_personality { 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         182: start: 
         183:  %_2.i = alloca %"alloc::vec::Vec<&str>", align 4 
         184:  %0 = getelementptr inbounds { ptr, i32 }, ptr %iter.0, i32 %iter.1 
         185:  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_2.i) 
         186:  tail call void @llvm.experimental.noalias.scope.decl(metadata !57) 
         187:  tail call void @llvm.experimental.noalias.scope.decl(metadata !60) 
         188:  tail call void @llvm.experimental.noalias.scope.decl(metadata !63) 
         189:  tail call void @llvm.experimental.noalias.scope.decl(metadata !66) 
         190:  %.idx = shl nuw nsw i32 %iter.1, 3 
         191:  %_5.i.i.i.i.i.i = icmp eq i32 %iter.1, 0 
         192:  br i1 %_5.i.i.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h9e295fb6a9a55082E.exit", label %bb6.i.i.i.i.i.i 
         193:  
         194: bb6.i.i.i.i.i.i: ; preds = %start 
         195:  %_6.i.i.i.i.i.i.i = icmp ugt i32 %iter.1, 268435455 
         196:  %.sroa.2.0.i.i.i.i.i.i.i = select i1 %_6.i.i.i.i.i.i.i, i32 0, i32 4 
         197:  br i1 %_6.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h23de59c4ef26b0d6E.exit.i.i.i.i.i.i" 
         198:  
         199: bb7.i.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i 
         200: ; call alloc::raw_vec::capacity_overflow 
         201:  tail call void @_ZN5alloc7raw_vec17capacity_overflow17h79624c32a837d574E() #12, !noalias !69 
         202:  unreachable 
         203:  
         204: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h23de59c4ef26b0d6E.exit.i.i.i.i.i.i": ; preds = %bb6.i.i.i.i.i.i 
         205:  %1 = tail call ptr @__rust_alloc(i32 %.idx, i32 %.sroa.2.0.i.i.i.i.i.i.i) #11, !noalias !69 
         206:  %2 = icmp eq ptr %1, null 
         207:  br i1 %2, label %bb19.i.i.i.i.i.i, label %bb3.i.i.i.i.i.preheader.i.i.i.i.i 
         208:  
         209: bb19.i.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h23de59c4ef26b0d6E.exit.i.i.i.i.i.i" 
         210: ; call alloc::alloc::handle_alloc_error 
         211:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h6fe90bebe1e7d8f1E(i32 %.idx, i32 noundef %.sroa.2.0.i.i.i.i.i.i.i) #12, !noalias !69 
         212:  unreachable 
         213:  
         214: bb3.i.i.i.i.i.preheader.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h23de59c4ef26b0d6E.exit.i.i.i.i.i.i" 
         215:  store i32 %iter.1, ptr %_2.i, align 4, !alias.scope !69 
         216:  %3 = getelementptr inbounds { i32, ptr }, ptr %_2.i, i32 0, i32 1 
         217:  store ptr %1, ptr %3, align 4, !alias.scope !69 
         218:  tail call void @llvm.experimental.noalias.scope.decl(metadata !70) 
         219:  tail call void @llvm.experimental.noalias.scope.decl(metadata !73) 
         220:  br label %bb3.i.i.i.i.i.i.i.i.i.i 
         221:  
         222: bb3.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.preheader.i.i.i.i.i 
         223:  %4 = phi i32 [ %8, %bb3.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb3.i.i.i.i.i.preheader.i.i.i.i.i ] 
         224:  %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i = phi ptr [ %5, %bb3.i.i.i.i.i.i.i.i.i.i ], [ %iter.0, %bb3.i.i.i.i.i.preheader.i.i.i.i.i ] 
         225:  %5 = getelementptr inbounds { ptr, i32 }, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, i32 1 
         226:  tail call void @llvm.experimental.noalias.scope.decl(metadata !76) 
         227:  %elt.0.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, align 4, !alias.scope !76, !noalias !79, !nonnull !10, !align !87, !noundef !10 
         228:  %6 = getelementptr inbounds { ptr, i32 }, ptr %self.sroa.2.07.i.i.i.i.i.i.i.i.i.i, i32 0, i32 1 
         229:  %elt.1.i.i.i.i.i.i.i.i.i.i.i = load i32, ptr %6, align 4, !alias.scope !76, !noalias !79 
         230:  %7 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 %4 
         231:  store ptr %elt.0.i.i.i.i.i.i.i.i.i.i.i, ptr %7, align 4, !noalias !88 
         232:  %src.sroa.4.0._13.sroa_idx.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %7, i32 4 
         233:  store i32 %elt.1.i.i.i.i.i.i.i.i.i.i.i, ptr %src.sroa.4.0._13.sroa_idx.i.i.i.i.i.i.i.i.i.i.i.i.i, align 4, !noalias !95 
         234:  %8 = add nuw nsw i32 %4, 1 
         235:  %_10.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %5, %0 
         236:  br i1 %_10.i.i.i.i.i.i.i.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i, label %bb3.i.i.i.i.i.i.i.i.i.i 
         237:  
         238: _ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i 
         239:  %9 = getelementptr inbounds %"alloc::vec::Vec<&str>", ptr %_2.i, i32 0, i32 1 
         240:  store i32 %8, ptr %9, align 4, !alias.scope !96, !noalias !97 
         241:  %_2.i.i.i = icmp ult i32 %8, %iter.1 
         242:  br i1 %_2.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i", label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h9e295fb6a9a55082E.exit" 
         243:  
         244: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i": ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i 
         245:  %_6.i.i.i.i.i.i1.i = icmp sgt i32 %iter.1, -1 
         246:  tail call void @llvm.assume(i1 %_6.i.i.i.i.i.i1.i) 
         247:  %_6.i.i.i.i.i.i = icmp ult i32 %4, 268435455 
         248:  tail call void @llvm.assume(i1 %_6.i.i.i.i.i.i) 
         249:  %array_size.i.i.i.i.i.i = shl nuw nsw i32 %8, 3 
         250:  %_23.i.i.i.i.i.i = icmp ule i32 %array_size.i.i.i.i.i.i, %.idx 
         251:  tail call void @llvm.assume(i1 %_23.i.i.i.i.i.i) 
         252:  %raw_ptr.i.i.i.i.i.i = tail call align 4 ptr @__rust_realloc(ptr nonnull %1, i32 %.idx, i32 4, i32 %array_size.i.i.i.i.i.i) #11, !noalias !100 
         253:  %10 = icmp eq ptr %raw_ptr.i.i.i.i.i.i, null 
         254:  br i1 %10, label %bb6.i.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h9e295fb6a9a55082E.exit" 
         255:  
         256: bb6.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i" 
         257: ; invoke alloc::alloc::handle_alloc_error 
         258:  invoke void @_ZN5alloc5alloc18handle_alloc_error17h6fe90bebe1e7d8f1E(i32 %array_size.i.i.i.i.i.i, i32 noundef 4) #12 
         259:  to label %.noexc.i.i unwind label %bb7.i.i, !noalias !109 
         260:  
         261: .noexc.i.i: ; preds = %bb6.i.i.i.i.i 
         262:  unreachable 
         263:  
         264: abort.i.i: ; preds = %bb7.i.i 
         265:  %11 = landingpad { ptr, i32 } 
         266:  cleanup 
next:39'0             X error: no match found
         267: ; call core::panicking::panic_cannot_unwind 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:39'1     ?                                            possible intended match
         268:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h9eae876adce0cb19E() #13, !noalias !109 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         269:  unreachable 
next:39'0     ~~~~~~~~~~~~~
         270:  
next:39'0     ~
         271: bb4.i.i: ; preds = %bb7.i.i 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         272:  resume { ptr, i32 } %12 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
         273:  
next:39'0     ~
         274: bb7.i.i: ; preds = %bb6.i.i.i.i.i 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         275:  %12 = landingpad { ptr, i32 } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         276:  cleanup 
next:39'0     ~~~~~~~~~
         277: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<&str>> 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         278:  invoke fastcc void @"_ZN4core3ptr51drop_in_place$LT$alloc..vec..Vec$LT$$RF$str$GT$$GT$17h70a4de6937d771c4E"(ptr nonnull %_2.i) #14 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         279:  to label %bb4.i.i unwind label %abort.i.i 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         280:  
next:39'0     ~
         281: "_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h9e295fb6a9a55082E.exit": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i", %start, %_ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         282:  %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i8.i = phi i32 [ %8, %_ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i ], [ 0, %start ], [ %8, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i" ] 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         283:  %value.sroa.0.sroa.4.0.copyload.i.i = phi ptr [ %1, %_ZN4core4iter6traits8iterator8Iterator7collect17h7e1f0bcb1779d511E.exit.i ], [ inttoptr (i32 4 to ptr), %start ], [ %raw_ptr.i.i.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17h1a0523d7a65eb876E.exit.i.i.i.i.i" ] 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         284:  %13 = insertvalue { ptr, i32 } undef, ptr %value.sroa.0.sroa.4.0.copyload.i.i, 0 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         285:  %14 = insertvalue { ptr, i32 } %13, i32 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i8.i, 1 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         286:  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_2.i) 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         287:  ret { ptr, i32 } %14 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~
         288: } 
next:39'0     ~~
         289:  
next:39'0     ~
         290: ; Function Attrs: nonlazybind uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         291: declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, ptr, ptr) unnamed_addr #0 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         292:  
next:39'0     ~
         293: ; Function Attrs: argmemonly mustprogress nocallback nofree nounwind willreturn 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         294: declare void @llvm.memcpy.p0.p0.i32(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i32, i1 immarg) #1 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         295:  
next:39'0     ~
         296: ; core::panicking::panic_cannot_unwind 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         297: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         298: declare void @_ZN4core9panicking19panic_cannot_unwind17h9eae876adce0cb19E() unnamed_addr #2 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         299:  
next:39'0     ~
         300: ; Function Attrs: inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         301: declare void @llvm.assume(i1 noundef) #3 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         302:  
next:39'0     ~
         303: ; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         304: declare noalias ptr @__rust_alloc(i32, i32 allocalign) unnamed_addr #4 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         305:  
next:39'0     ~
         306: ; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         307: declare noalias ptr @__rust_realloc(ptr allocptr, i32, i32 allocalign, i32) unnamed_addr #5 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         308:  
next:39'0     ~
         309: ; alloc::alloc::handle_alloc_error 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         310: ; Function Attrs: cold noreturn nonlazybind uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         311: declare void @_ZN5alloc5alloc18handle_alloc_error17h6fe90bebe1e7d8f1E(i32, i32 noundef) unnamed_addr #6 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         312:  
next:39'0     ~
         313: ; alloc::raw_vec::capacity_overflow 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         314: ; Function Attrs: noreturn nonlazybind uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         315: declare void @_ZN5alloc7raw_vec17capacity_overflow17h79624c32a837d574E() unnamed_addr #7 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         316:  
next:39'0     ~
         317: ; Function Attrs: nounwind nonlazybind allockind("free") uwtable 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         318: declare void @__rust_dealloc(ptr allocptr, i32, i32) unnamed_addr #8 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         319:  
next:39'0     ~
         320: ; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         321: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #9 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         322:  
next:39'0     ~
         323: ; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         324: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #9 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         325:  
next:39'0     ~
         326: ; Function Attrs: inaccessiblememonly nocallback nofree nosync nounwind willreturn 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         327: declare void @llvm.experimental.noalias.scope.decl(metadata) #10 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         328:  
next:39'0     ~
         329: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         330: attributes #1 = { argmemonly mustprogress nocallback nofree nounwind willreturn } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         331: attributes #2 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         332: attributes #3 = { inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         333: attributes #4 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         334: attributes #5 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         335: attributes #6 = { cold noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         336: attributes #7 = { noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         337: attributes #8 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         338: attributes #9 = { argmemonly mustprogress nocallback nofree nosync nounwind willreturn } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         339: attributes #10 = { inaccessiblememonly nocallback nofree nosync nounwind willreturn } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         340: attributes #11 = { nounwind } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         341: attributes #12 = { noreturn } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         342: attributes #13 = { noinline noreturn nounwind } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         343: attributes #14 = { noinline } 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         344:  
next:39'0     ~
         345: !llvm.module.flags = !{!0, !1} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         346:  
next:39'0     ~
         347: !0 = !{i32 7, !"PIC Level", i32 2} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         348: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         349: !2 = !{!3} 
next:39'0     ~~~~~~~~~~~
         350: !3 = distinct !{!3, !4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h674456140b43a433E: %self"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         351: !4 = distinct !{!4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h674456140b43a433E"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         352: !5 = !{!6, !3} 
next:39'0     ~~~~~~~~~~~~~~~
         353: !6 = distinct !{!6, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h4f78b7baa0804bb0E: %self"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         354: !7 = distinct !{!7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h4f78b7baa0804bb0E"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         355: !8 = !{!9} 
next:39'0     ~~~~~~~~~~~
         356: !9 = distinct !{!9, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h4f78b7baa0804bb0E: argument 0"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         357: !10 = !{} 
next:39'0     ~~~~~~~~~~
         358: !11 = !{!12} 
next:39'0     ~~~~~~~~~~~~~
         359: !12 = distinct !{!12, !13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc9044d509a153d35E: %self"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         360: !13 = distinct !{!13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc9044d509a153d35E"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         361: !14 = !{!15, !12} 
next:39'0     ~~~~~~~~~~~~~~~~~~
         362: !15 = distinct !{!15, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb6fe5ff8b6d897c6E: %self"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         363: !16 = distinct !{!16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb6fe5ff8b6d897c6E"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         364: !17 = !{!18} 
next:39'0     ~~~~~~~~~~~~~
         365: !18 = distinct !{!18, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hb6fe5ff8b6d897c6E: argument 0"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         366: !19 = !{!20} 
next:39'0     ~~~~~~~~~~~~~
         367: !20 = distinct !{!20, !21, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17h97f417373fb40ae5E: %self"} 
next:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
