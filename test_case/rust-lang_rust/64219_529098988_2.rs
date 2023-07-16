llvm
@FOO = external global void ()*

define void @foo() unnamed_addr #0 {
start:
  %0 = load void ()*, void ()** @FOO, align 8, !nonnull !0
  call void %0() #1
  unreachable
}
attributes #1 = { noreturn }
