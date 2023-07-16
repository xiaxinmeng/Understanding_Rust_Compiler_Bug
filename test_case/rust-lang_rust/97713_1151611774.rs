
$f = comdat any

@llvm.global_ctors = appending global [2 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 2, void ()* @sancov.module_ctor_8bit_counters, i8* bitcast (void ()* @sancov.module_ctor_8bit_counters to i8*) },{ i32, void ()*, i8* } { i32 2, void ()* @f, i8* bitcast (void ()* @f to i8*) }]                                                                                                                                   

define internal void @f() comdat {
  ret void
}

define internal void @sancov.module_ctor_8bit_counters() {
  ret void
}
