
; issue_65844::get_pair
; Function Attrs: nonlazybind uwtable
define internal { double, double } @_ZN11issue_658448get_pair17h78e1eb5d988d8ff6E() unnamed_addr #0 {
start:
  %0 = alloca { double, double }, align 8
  %1 = bitcast { double, double }* %0 to double*
  store double 0.000000e+00, double* %1, align 8
  %2 = getelementptr inbounds { double, double }, { double, double }* %0, i32 0, i32 1
  store double 0.000000e+00, double* %2, align 8
  %3 = getelementptr inbounds { double, double }, { double, double }* %0, i32 0, i32 0
  %4 = load double, double* %3, align 8
  %5 = getelementptr inbounds { double, double }, { double, double }* %0, i32 0, i32 1
  %6 = load double, double* %5, align 8
  %7 = insertvalue { double, double } undef, double %4, 0
  %8 = insertvalue { double, double } %7, double %6, 1
  ret { double, double } %8
}
