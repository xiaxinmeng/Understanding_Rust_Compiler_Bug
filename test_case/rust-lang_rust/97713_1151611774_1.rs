
$f = comdat any

$sancov.module_ctor_8bit_counters = comdat any

@__sancov_gen_ = private global [1 x i8] zeroinitializer, section "__sancov_cntrs", comdat($f), align 1
@llvm.global_ctors = appending global [3 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 2, void ()* @sancov.module_ctor_8bit_counters, i8* bitcast (void ()* @sancov.module_ctor_8bit_counters to i8*) },\
 { i32, void ()*, i8* } { i32 2, void ()* @f, i8* bitcast (void ()* @f to i8*) }, { i32, void ()*, i8* } { i32 2, void ()* @sancov.module_ctor_8bit_counters.1, i8* bitcast (void ()* @sancov.module_ctor_8bit_cou\
nters.1 to i8*) }]
@llvm.used = appending global [1 x i8*] [i8* bitcast (void ()* @sancov.module_ctor_8bit_counters.1 to i8*)], section "llvm.metadata"
@llvm.compiler.used = appending global [1 x i8*] [i8* getelementptr inbounds ([1 x i8], [1 x i8]* @__sancov_gen_, i32 0, i32 0)], section "llvm.metadata"

define internal void @sancov.module_ctor_8bit_counters() {
  ret void
}

; Function Attrs: nounwind
define internal void @sancov.module_ctor_8bit_counters.1() #0 comdat($sancov.module_ctor_8bit_counters) {
  call void @__sanitizer_cov_8bit_counters_init(i8* @__start___sancov_cntrs, i8* @__stop___sancov_cntrs)
  ret void
}

attributes #0 = { nounwind }
