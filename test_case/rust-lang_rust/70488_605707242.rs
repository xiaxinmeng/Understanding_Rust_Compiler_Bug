llvm
; stable
%"std::future::GenFuture<ready_bench::{{closure}}::{{closure}}>" = type { [0 x i32], %"ready_bench::{{closure}}::{{closure}}", [0 x i32] }
%"ready_bench::{{closure}}::{{closure}}" = type { [0 x i32], i32, [2 x i32] }

; nightly
%"core::future::from_generator::GenFuture<ready_bench::{{closure}}::{{closure}}>" = type { [0 x i32], %"ready_bench::{{closure}}::{{closure}}", [0 x i32] }
%"ready_bench::{{closure}}::{{closure}}" = type { [8 x i8], i8, [3 x i8] }
