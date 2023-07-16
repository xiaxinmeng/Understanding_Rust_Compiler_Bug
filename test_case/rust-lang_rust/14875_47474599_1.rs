 llvm
define internal void @_ZN4main20hd8096a747d8d732eIaa4v0.0E() unnamed_addr #0 {
entry-block:
  %b = alloca %"struct.Bar<[]>"
  %f = alloca %"struct.Foo<[]>"
  %0 = getelementptr inbounds %"struct.Bar<[]>"* %b, i32 0, i32 0
  store i1 true, i1* %0
  %1 = getelementptr inbounds %"struct.Foo<[]>"* %f, i32 0, i32 0
  store i1 true, i1* %1
  call void @_ZN3Foo14glue_drop.114317h8e9a26d4f7ff5149E(%"struct.Foo<[]>"* %f)
  call void @_ZN3Bar14glue_drop.114517hb6d883694ad7fcd5E(%"struct.Bar<[]>"* %b)
  ret void
}
