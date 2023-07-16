
define internal void @_ZN4drop17h974e21c3220a1699E(%"3.std::vec::Vec<u8>"*) unnamed_addr personality i32 (i32, i32, i64, %"8.unwind::libunwind::_Unwind_Exception"*, %"8.unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %1 = alloca { i8*, i32 }
  invoke void @"_ZN66_$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..Drop$GT$4drop17h4c07c16183524d21E"(%"3.std::vec::Vec<u8>"* dereferenceable(24) %0)
          to label %normal-return unwind label %unwind_custom_

normal-return:                                    ; preds = %entry-block
  call void @_ZN13drop_contents17h974e21c3220a1699E(%"3.std::vec::Vec<u8>"* %0)
  ret void

** SNIP **

declare void @"_ZN66_$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..Drop$GT$4drop17h4c07c16183524d21E"(%"3.std::vec::Vec<u8>"* dereferenceable(24)) unnamed_addr
