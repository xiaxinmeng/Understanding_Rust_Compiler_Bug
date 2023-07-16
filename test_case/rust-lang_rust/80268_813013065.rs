llvm-ir
define float @foo(double %x) {
start:
  %0 = fptrunc double %x to float
  ret float %0
}
