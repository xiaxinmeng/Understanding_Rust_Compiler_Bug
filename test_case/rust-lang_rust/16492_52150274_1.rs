 llvm
define internal void @_ZN11DropMe_Impl14glue_drop.146417h9f98bddab7694295E(%"struct.DropMe_Impl<[]>"*) unnamed_addr #3 {
entry-block:
  %1 = alloca { i8*, i32 }
  %2 = getelementptr inbounds %"struct.DropMe_Impl<[]>"* %0, i32 0, i32 3
  %3 = load i8* %2, !range !0
  %4 = trunc i8 %3 to i1
  br i1 %4, label %cond, label %next

next:                                             ; preds = %normal-return, %entry-block
  ret void

cond:                                             ; preds = %entry-block
  %5 = getelementptr inbounds %"struct.DropMe_Impl<[]>"* %0, i32 0, i32 0
  %6 = getelementptr inbounds %"struct.DropMe_Impl<[]>"* %0, i32 0, i32 1
  %7 = getelementptr inbounds %"struct.DropMe_Impl<[]>"* %0, i32 0, i32 2
  %8 = getelementptr inbounds %"struct.DropMe_Impl<[]>"* %0, i32 0, i32 3
  invoke void @_ZN16DropMe_Impl.Drop4drop20hf5d4e9dbec49bd1eCaaE(%"struct.DropMe_Impl<[]>"* noalias nocapture dereferenceable(80) %0)
          to label %normal-return unwind label %unwind_custom_

normal-return:                                    ; preds = %cond
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %7)
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %6)
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %5)
  br label %next

unwind_custom_:                                   ; preds = %cond
  %9 = landingpad { i8*, i32 } personality i32 (i32, i32, i64, %"struct.rustrt::libunwind::_Unwind_Exception<[]>[#9]"*, %"enum.rustrt::libunwind::_Unwind_Context<[]>[#9]"*)* @rust_eh_personality
          cleanup
  store { i8*, i32 } %9, { i8*, i32 }* %1
  br label %clean_custom_

resume:                                           ; preds = %clean_custom_
  %10 = load { i8*, i32 }* %1
  resume { i8*, i32 } %10

clean_custom_:                                    ; preds = %unwind_custom_
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %7)
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %6)
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %5)
  br label %resume
}
