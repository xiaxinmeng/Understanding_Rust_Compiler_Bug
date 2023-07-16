plain
failures:

---- [codegen] tests/codegen/vec-shrink-panik.rs#old stdout ----

error in revision `old`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll" "/checkout/tests/codegen/vec-shrink-panik.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,old" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/vec-shrink-panik.rs:29:15: error: old-NEXT: is not on the line after the previous match
 // old-NEXT: ; call core::panicking::panic_cannot_unwind
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:146:1: note: 'next' match was here
; call core::panicking::panic_cannot_unwind
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:95:32: note: previous match ended here
define { ptr, i64 } @issue71861(ptr noalias nocapture noundef dereferenceable(24) %vec) unnamed_addr #0 personality ptr @rust_eh_personality {
                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:96:1: note: non-matching line after previous match is here
^
^
/checkout/tests/codegen/vec-shrink-panik.rs:44:15: error: old-NEXT: is not on the line after the previous match
 // old-NEXT: ; call core::panicking::panic_cannot_unwind
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:245:1: note: 'next' match was here
; call core::panicking::panic_cannot_unwind
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:168:32: note: previous match ended here
define { ptr, i64 } @issue75636(ptr noalias noundef nonnull readonly align 8 %iter.0, i64 noundef %iter.1) unnamed_addr #0 personality ptr @rust_eh_personality {
                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll:169:1: note: non-matching line after previous match is here
^


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.old/vec-shrink-panik.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
        46:  ret void 
        47: } 
        48:  
        49: ; Function Attrs: nonlazybind uwtable 
        50: define void @shrink_to_fit(ptr noalias nocapture noundef align 8 dereferenceable(24) %vec) unnamed_addr #0 { 
        51: start: 
        52:  tail call void @llvm.experimental.noalias.scope.decl(metadata !19) 
        53:  %0 = getelementptr inbounds { ptr, i64 }, ptr %vec, i64 0, i32 1 
        54:  %1 = load i64, ptr %0, align 8, !alias.scope !19, !noundef !10 
        55:  %2 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %vec, i64 0, i32 1 
        56:  %_5.i = load i64, ptr %2, align 8, !alias.scope !19, !noundef !10 
        57:  %_2.i = icmp ugt i64 %1, %_5.i 
        58:  br i1 %_2.i, label %bb4.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17hf84ba881fcb8f5d5E.exit" 
        59:  
        60: bb4.i.i: ; preds = %start 
        61:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22) 
        62:  %3 = shl nuw i64 %1, 2 
        63:  %self3.i.i.i = load ptr, ptr %vec, align 8, !alias.scope !25, !noalias !28, !nonnull !10, !noundef !10 
        64:  %4 = icmp eq i64 %_5.i, 0 
        65:  br i1 %4, label %bb1.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i" 
        66:  
        67: bb1.i.i.i.i: ; preds = %bb4.i.i 
        68:  tail call void @__rust_dealloc(ptr noundef nonnull %self3.i.i.i, i64 noundef %3, i64 noundef 4) #10, !noalias !30 
        69:  br label %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i 
        70:  
        71: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i": ; preds = %bb4.i.i 
        72:  %5 = shl nuw i64 %_5.i, 2 
        73:  %_20.i.i.i = icmp ule i64 %5, %3 
        74:  tail call void @llvm.assume(i1 %_20.i.i.i) 
        75:  %raw_ptr.i.i.i = tail call noundef align 4 ptr @__rust_realloc(ptr noundef nonnull %self3.i.i.i, i64 noundef %3, i64 noundef 4, i64 noundef %5) #10, !noalias !30 
        76:  %6 = icmp eq ptr %raw_ptr.i.i.i, null 
        77:  br i1 %6, label %bb6.i.i, label %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i 
        78:  
        79: bb6.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i" 
        80: ; call alloc::alloc::handle_alloc_error 
        81:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h9f3c64ea7bb5ffe5E(i64 noundef 4, i64 noundef %5) #11, !noalias !19 
        82:  unreachable 
        83:  
        84: _ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i", %bb1.i.i.i.i 
        85:  %.sroa.0.0.i22.i.i = phi ptr [ inttoptr (i64 4 to ptr), %bb1.i.i.i.i ], [ %raw_ptr.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i" ] 
        86:  store ptr %.sroa.0.0.i22.i.i, ptr %vec, align 8, !alias.scope !31 
        87:  store i64 %_5.i, ptr %0, align 8, !alias.scope !31 
        88:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17hf84ba881fcb8f5d5E.exit" 
        89:  
        90: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17hf84ba881fcb8f5d5E.exit": ; preds = %start, %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i 
        91:  ret void 
        92: } 
        93:  
        94: ; Function Attrs: nonlazybind uwtable 
        95: define { ptr, i64 } @issue71861(ptr noalias nocapture noundef dereferenceable(24) %vec) unnamed_addr #0 personality ptr @rust_eh_personality { 
        96: start: 
        97:  tail call void @llvm.experimental.noalias.scope.decl(metadata !34) 
        98:  tail call void @llvm.experimental.noalias.scope.decl(metadata !37) 
        99:  %0 = getelementptr inbounds { ptr, i64 }, ptr %vec, i64 0, i32 1 
       100:  %1 = load i64, ptr %0, align 8, !alias.scope !40, !noundef !10 
       101:  %2 = getelementptr inbounds %"alloc::vec::Vec<u32>", ptr %vec, i64 0, i32 1 
       102:  %_5.i.i = load i64, ptr %2, align 8, !alias.scope !40, !noundef !10 
       103:  %_2.i.i = icmp ugt i64 %1, %_5.i.i 
       104:  br i1 %_2.i.i, label %bb4.i.i.i, label %start.bb3_crit_edge.i 
       105:  
       106: start.bb3_crit_edge.i: ; preds = %start 
       107:  %value.sroa.0.0.copyload.pre.i = load ptr, ptr %vec, align 8, !alias.scope !34 
       108:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h44b6d0d7cf885f5fE.exit" 
       109:  
       110: bb4.i.i.i: ; preds = %start 
       111:  tail call void @llvm.experimental.noalias.scope.decl(metadata !41) 
       112:  %3 = shl nuw i64 %1, 2 
       113:  %self3.i.i.i.i = load ptr, ptr %vec, align 8, !alias.scope !44, !noalias !47, !nonnull !10, !noundef !10 
       114:  %4 = icmp eq i64 %_5.i.i, 0 
       115:  br i1 %4, label %bb1.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i" 
       116:  
       117: bb1.i.i.i.i.i: ; preds = %bb4.i.i.i 
       118:  tail call void @__rust_dealloc(ptr noundef nonnull %self3.i.i.i.i, i64 noundef %3, i64 noundef 4) #10, !noalias !49 
       119:  br label %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i.i 
       120:  
       121: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i": ; preds = %bb4.i.i.i 
       122:  %5 = shl nuw i64 %_5.i.i, 2 
Build completed unsuccessfully in 0:05:53
       123:  %_20.i.i.i.i = icmp ule i64 %5, %3 
       124:  tail call void @llvm.assume(i1 %_20.i.i.i.i) 
       125:  %raw_ptr.i.i.i.i = tail call noundef align 4 ptr @__rust_realloc(ptr noundef nonnull %self3.i.i.i.i, i64 noundef %3, i64 noundef 4, i64 noundef %5) #10, !noalias !49 
       126:  %6 = icmp eq ptr %raw_ptr.i.i.i.i, null 
       127:  br i1 %6, label %bb6.i.i.i, label %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i.i 
       128:  
       129: bb6.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i" 
       130: ; invoke alloc::alloc::handle_alloc_error 
       131:  invoke void @_ZN5alloc5alloc18handle_alloc_error17h9f3c64ea7bb5ffe5E(i64 noundef 4, i64 noundef %5) #11 
       132:  to label %.noexc.i unwind label %bb7.i, !noalias !34 
       133:  
       134: .noexc.i: ; preds = %bb6.i.i.i 
       135:  unreachable 
       136:  
       137: _ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i", %bb1.i.i.i.i.i 
       138:  %.sroa.0.0.i22.i.i.i = phi ptr [ inttoptr (i64 4 to ptr), %bb1.i.i.i.i.i ], [ %raw_ptr.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i" ] 
       139:  store ptr %.sroa.0.0.i22.i.i.i, ptr %vec, align 8, !alias.scope !50 
       140:  store i64 %_5.i.i, ptr %0, align 8, !alias.scope !50 
       141:  br label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h44b6d0d7cf885f5fE.exit" 
       142:  
       143: terminate.i: ; preds = %bb7.i 
       144:  %7 = landingpad { ptr, i32 } 
       145:  filter [0 x ptr] zeroinitializer 
       146: ; call core::panicking::panic_cannot_unwind 
next:29     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  error: match on wrong line
       147:  tail call void @_ZN4core9panicking19panic_cannot_unwind17hfb9e551d2dae76ffE() #12, !noalias !34 
       148:  unreachable 
       149:  
       150: bb4.i: ; preds = %bb7.i 
       151:  resume { ptr, i32 } %8 
       152:  
       153: bb7.i: ; preds = %bb6.i.i.i 
       154:  %8 = landingpad { ptr, i32 } 
       155:  cleanup 
       156: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<u32>> 
       157:  invoke fastcc void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u32$GT$$GT$17h07178c3c1924233aE"(ptr noundef nonnull %vec) #13 
       158:  to label %bb4.i unwind label %terminate.i 
       159:  
       160: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$16into_boxed_slice17h44b6d0d7cf885f5fE.exit": ; preds = %start.bb3_crit_edge.i, %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i.i 
       161:  %value.sroa.0.0.copyload.i = phi ptr [ %value.sroa.0.0.copyload.pre.i, %start.bb3_crit_edge.i ], [ %.sroa.0.0.i22.i.i.i, %_ZN5alloc7raw_vec14handle_reserve17h20e6b331666304eaE.exit.i.i ] 
       162:  %9 = insertvalue { ptr, i64 } poison, ptr %value.sroa.0.0.copyload.i, 0 
       163:  %10 = insertvalue { ptr, i64 } %9, i64 %_5.i.i, 1 
       164:  ret { ptr, i64 } %10 
       165: } 
       166:  
       167: ; Function Attrs: nonlazybind uwtable 
       168: define { ptr, i64 } @issue75636(ptr noalias noundef nonnull readonly align 8 %iter.0, i64 noundef %iter.1) unnamed_addr #0 personality ptr @rust_eh_personality { 
       169: start: 
       170:  %_2.i = alloca %"alloc::vec::Vec<&str>", align 8 
       171:  %0 = getelementptr inbounds { ptr, i64 }, ptr %iter.0, i64 %iter.1 
       172:  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i) 
       173:  tail call void @llvm.experimental.noalias.scope.decl(metadata !53) 
       174:  tail call void @llvm.experimental.noalias.scope.decl(metadata !56) 
       175:  tail call void @llvm.experimental.noalias.scope.decl(metadata !59) 
       176:  tail call void @llvm.experimental.noalias.scope.decl(metadata !62) 
       177:  %.idx = shl nuw nsw i64 %iter.1, 4 
       178:  %_5.i.i.i.i.i.i = icmp eq i64 %iter.1, 0 
       179:  br i1 %_5.i.i.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h35d09aaea7ada65aE.exit", label %bb5.i.i.i.i.i.i 
       180:  
       181: bb5.i.i.i.i.i.i: ; preds = %start 
       182:  %_5.i.i.i.i.i.i.i = icmp ugt i64 %iter.1, 576460752303423487 
       183:  %.sroa.0.0.i.i.i.i.i.i.i = select i1 %_5.i.i.i.i.i.i.i, i64 0, i64 8 
       184:  br i1 %_5.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i" 
       185:  
       186: bb6.i.i.i.i.i.i: ; preds = %bb5.i.i.i.i.i.i 
       187: ; call alloc::raw_vec::capacity_overflow 
       188:  tail call void @_ZN5alloc7raw_vec17capacity_overflow17h4bcf5e113aebd33aE() #11, !noalias !65 
       189:  unreachable 
       190:  
       191: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i": ; preds = %bb5.i.i.i.i.i.i 
       192:  %1 = tail call noundef ptr @__rust_alloc(i64 noundef %.idx, i64 noundef %.sroa.0.0.i.i.i.i.i.i.i) #10, !noalias !65 
       193:  %2 = icmp eq ptr %1, null 
       194:  br i1 %2, label %bb16.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i 
       195:  
       196: bb16.i.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i" 
       197: ; call alloc::alloc::handle_alloc_error 
       198:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h9f3c64ea7bb5ffe5E(i64 noundef %.sroa.0.0.i.i.i.i.i.i.i, i64 noundef %.idx) #11, !noalias !65 
       199:  unreachable 
       200:  
       201: bb3.i.i.i.i.i.i.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i", %bb3.i.i.i.i.i.i.i.i.i.i 
       202:  %3 = phi i64 [ %6, %bb3.i.i.i.i.i.i.i.i.i.i ], [ 0, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i" ] 
       203:  %self.sroa.0.07.i.i.i.i.i.i.i.i.i.i = phi ptr [ %ptr8.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i ], [ %iter.0, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h58b70d21812b71b6E.exit.i.i.i.i.i.i" ] 
       204:  %ptr8.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds { ptr, i64 }, ptr %self.sroa.0.07.i.i.i.i.i.i.i.i.i.i, i64 1 
       205:  tail call void @llvm.experimental.noalias.scope.decl(metadata !66) 
       206:  %elt.0.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %self.sroa.0.07.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !66, !noalias !69, !nonnull !10, !align !81, !noundef !10 
       207:  %4 = getelementptr inbounds { ptr, i64 }, ptr %self.sroa.0.07.i.i.i.i.i.i.i.i.i.i, i64 0, i32 1 
       208:  %elt.1.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %4, align 8, !alias.scope !66, !noalias !69, !noundef !10 
       209:  %dst.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds { ptr, i64 }, ptr %1, i64 %3 
       210:  store ptr %elt.0.i.i.i.i.i.i.i.i.i.i.i, ptr %dst.i.i.i.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !82 
       211:  %5 = getelementptr inbounds { ptr, i64 }, ptr %1, i64 %3, i32 1 
       212:  store i64 %elt.1.i.i.i.i.i.i.i.i.i.i.i, ptr %5, align 8, !noalias !89 
       213:  %6 = add nuw nsw i64 %3, 1 
       214:  %_6.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %ptr8.i.i.i.i.i.i.i.i.i.i.i, %0 
       215:  br i1 %_6.i.i.i.i.i.i.i.i.i.i.i, label %_ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i, label %bb3.i.i.i.i.i.i.i.i.i.i 
       216:  
       217: _ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i 
       218:  store ptr %1, ptr %_2.i, align 8, !alias.scope !65 
       219:  %vector.sroa.5.0..sroa_idx.i.i.i.i.i = getelementptr inbounds i8, ptr %_2.i, i64 8 
       220:  store i64 %iter.1, ptr %vector.sroa.5.0..sroa_idx.i.i.i.i.i, align 8, !alias.scope !65 
       221:  %vector.sroa.6.0..sroa_idx.i.i.i.i.i = getelementptr inbounds i8, ptr %_2.i, i64 16 
       222:  store i64 %6, ptr %vector.sroa.6.0..sroa_idx.i.i.i.i.i, align 8, !alias.scope !65 
       223:  %_2.i.i.i = icmp ult i64 %6, %iter.1 
       224:  br i1 %_2.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i", label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h35d09aaea7ada65aE.exit" 
       225:  
       226: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i": ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i 
       227:  %7 = shl nuw i64 %6, 4 
       228:  %_20.i.i.i.i.i = icmp ule i64 %7, %.idx 
       229:  tail call void @llvm.assume(i1 %_20.i.i.i.i.i) 
       230:  %raw_ptr.i.i.i.i.i = tail call noundef align 8 ptr @__rust_realloc(ptr noundef nonnull %1, i64 noundef %.idx, i64 noundef 8, i64 noundef %7) #10, !noalias !90 
       231:  %8 = icmp eq ptr %raw_ptr.i.i.i.i.i, null 
       232:  br i1 %8, label %bb6.i.i.i.i, label %"_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h35d09aaea7ada65aE.exit" 
       233:  
       234: bb6.i.i.i.i: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i" 
       235: ; invoke alloc::alloc::handle_alloc_error 
       236:  invoke void @_ZN5alloc5alloc18handle_alloc_error17h9f3c64ea7bb5ffe5E(i64 noundef 8, i64 noundef %7) #11 
       237:  to label %.noexc.i.i unwind label %bb7.i.i, !noalias !97 
       238:  
       239: .noexc.i.i: ; preds = %bb6.i.i.i.i 
       240:  unreachable 
       241:  
       242: terminate.i.i: ; preds = %bb7.i.i 
       243:  %9 = landingpad { ptr, i32 } 
       244:  filter [0 x ptr] zeroinitializer 
       245: ; call core::panicking::panic_cannot_unwind 
next:44     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~  error: match on wrong line
       246:  tail call void @_ZN4core9panicking19panic_cannot_unwind17hfb9e551d2dae76ffE() #12, !noalias !97 
       247:  unreachable 
       248:  
       249: bb4.i.i: ; preds = %bb7.i.i 
       250:  resume { ptr, i32 } %10 
       251:  
       252: bb7.i.i: ; preds = %bb6.i.i.i.i 
       253:  %10 = landingpad { ptr, i32 } 
       254:  cleanup 
       255: ; invoke core::ptr::drop_in_place<alloc::vec::Vec<&str>> 
       256:  invoke fastcc void @"_ZN4core3ptr51drop_in_place$LT$alloc..vec..Vec$LT$$RF$str$GT$$GT$17hf584a5ce0dc36ceeE"(ptr noundef nonnull %_2.i) #13 
       257:  to label %bb4.i.i unwind label %terminate.i.i 
       258:  
       259: "_ZN107_$LT$alloc..boxed..Box$LT$$u5b$I$u5d$$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$I$GT$$GT$9from_iter17h35d09aaea7ada65aE.exit": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i", %start, %_ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i 
       260:  %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i8.i = phi i64 [ %6, %_ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i ], [ 0, %start ], [ %6, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i" ] 
       261:  %value.sroa.0.0.copyload.i.i = phi ptr [ %1, %_ZN4core4iter6traits8iterator8Iterator7collect17h35edd01800334ae9E.exit.i ], [ inttoptr (i64 8 to ptr), %start ], [ %raw_ptr.i.i.i.i.i, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$6shrink17hc1cd009cdbbf4e76E.exit.i.i.i.i" ] 
       262:  %11 = insertvalue { ptr, i64 } poison, ptr %value.sroa.0.0.copyload.i.i, 0 
       263:  %12 = insertvalue { ptr, i64 } %11, i64 %_2.i.i.i.i.i.i.i.i.i.i.i.i.i.i8.i, 1 
       264:  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i) 
       265:  ret { ptr, i64 } %12 
       266: } 
       267:  
       268: ; Function Attrs: nonlazybind uwtable 
       269: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0 
       271: ; core::panicking::panic_cannot_unwind 
       271: ; core::panicking::panic_cannot_unwind 
       272: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
       273: declare void @_ZN4core9panicking19panic_cannot_unwind17hfb9e551d2dae76ffE() unnamed_addr #1 
       274:  
       275: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) 
       276: declare void @llvm.assume(i1 noundef) #2 
       277:  
       278: ; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable 
       279: declare noalias noundef ptr @__rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #3 
       280:  
       281: ; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable 
       282: declare noalias noundef ptr @__rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #4 
       284: ; alloc::alloc::handle_alloc_error 
       284: ; alloc::alloc::handle_alloc_error 
       285: ; Function Attrs: cold noreturn nonlazybind uwtable 
       286: declare void @_ZN5alloc5alloc18handle_alloc_error17h9f3c64ea7bb5ffe5E(i64 noundef, i64 noundef) unnamed_addr #5 
       288: ; alloc::raw_vec::capacity_overflow 
       288: ; alloc::raw_vec::capacity_overflow 
       289: ; Function Attrs: noreturn nonlazybind uwtable 
       290: declare void @_ZN5alloc7raw_vec17capacity_overflow17h4bcf5e113aebd33aE() unnamed_addr #6 
       291:  
       292: ; Function Attrs: nounwind nonlazybind allockind("free") uwtable 
       293: declare void @__rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #7 
       294:  
       295: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
       296: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #8 
       297:  
       298: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
       299: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #8 
       300:  
       301: ; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) 
       302: declare void @llvm.experimental.noalias.scope.decl(metadata) #9 
       303:  
       304: attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       305: attributes #1 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       306: attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) } 
       307: attributes #3 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       308: attributes #4 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       309: attributes #5 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       310: attributes #6 = { noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       311: attributes #7 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
       312: attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) } 
       313: attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) } 
       314: attributes #10 = { nounwind } 
       315: attributes #11 = { noreturn } 
       316: attributes #12 = { noinline noreturn nounwind } 
       317: attributes #13 = { noinline } 
       318:  
       319: !llvm.module.flags = !{!0, !1} 
       320:  
       321: !0 = !{i32 8, !"PIC Level", i32 2} 
       322: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
       323: !2 = !{!3} 
       324: !3 = distinct !{!3, !4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd1c74e309f0b794eE: %self"} 
       325: !4 = distinct !{!4, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hd1c74e309f0b794eE"} 
       326: !5 = !{!6, !3} 
       327: !6 = distinct !{!6, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1d49a90720123729E: %self"} 
       328: !7 = distinct !{!7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1d49a90720123729E"} 
       329: !8 = !{!9} 
       330: !9 = distinct !{!9, !7, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1d49a90720123729E: argument 0"} 
       331: !10 = !{} 
       332: !11 = !{!12} 
       333: !12 = distinct !{!12, !13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h998dee209ae45d30E: %self"} 
       334: !13 = distinct !{!13, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h998dee209ae45d30E"} 
       335: !14 = !{!15, !12} 
       336: !15 = distinct !{!15, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hbf64544f204a76a3E: %self"} 
       337: !16 = distinct !{!16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hbf64544f204a76a3E"} 
       338: !17 = !{!18} 
       339: !18 = distinct !{!18, !16, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hbf64544f204a76a3E: argument 0"} 
       340: !19 = !{!20} 
       341: !20 = distinct !{!20, !21, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17hf84ba881fcb8f5d5E: %self"} 
       342: !21 = distinct !{!21, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13shrink_to_fit17hf84ba881fcb8f5d5E"} 
       343: !22 = !{!23} 
       344: !23 = distinct !{!23, !24, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6shrink17hefec308083ac2543E: %self"} 
       345: !24 = distinct !{!24, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6shrink17hefec308083ac2543E"} 
         .
         .
>>>>>>
------------------------------------------
