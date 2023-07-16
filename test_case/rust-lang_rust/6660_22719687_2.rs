

; Function Attrs: fixedstacksegment noinline uwtable
define void @"_ZN4main16_cd92990fc38403c7_0$x2e0E"({ i32, %tydesc*, i8*, i8*, i8 }*) #1 {
"function top level":
  %a = alloca %"struct.A::TwoU32s"
  %b = alloca %"struct.B::TwoU32s"
  call void @rust_dbg_extern_return_TwoU32s(%"struct.A::TwoU32s"* %a)
  call void bitcast (void (%"struct.A::TwoU32s"*)* @rust_dbg_extern_return_TwoU32s to void (%"struct.B::TwoU32s"*)*)(%"struct.B::TwoU32s"* %b)
  ret void
}
