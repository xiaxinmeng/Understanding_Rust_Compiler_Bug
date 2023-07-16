llvm
attributes #0 = { "target-cpu"="x86-64" "target-features"="-sse2" }

define { double, double } @get_pair() unnamed_addr #0 {
    %1 = alloca { double, double }, align 8
    %2 = bitcast { double, double }* %1 to double*
    store double 0.000000e+00, double* %2, align 8
    %3 = getelementptr inbounds { double, double }, { double, double }* %1, i32 0, i32 1
    store double 0.000000e+00, double* %3, align 8
    %4 = getelementptr inbounds { double, double }, { double, double }* %1, i32 0, i32 0
    %5 = load double, double* %4, align 8
    %6 = getelementptr inbounds { double, double }, { double, double }* %1, i32 0, i32 1
    %7 = load double, double* %6, align 8
    %8 = insertvalue { double, double } undef, double %5, 0
    %9 = insertvalue { double, double } %8, double %7, 1
    ret { double, double } %9
}
