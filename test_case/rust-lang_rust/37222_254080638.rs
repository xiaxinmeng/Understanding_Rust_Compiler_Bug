 llvm
; GOOD
%Binding = type { %Action, i8 }
%Action = type { i8, [7 x i8], [2 x i64] }

@_ZN5newsf10V_BINDINGS17h3669347bdb3d8fc8E = internal constant { %Binding*, i64 } { %Binding* bitcast ({
    { { i8, [23 x i8] }, i8, [7 x i8] },
    { { i8, [23 x i8] }, i8, [7 x i8] }
}* @ref8290 to %Binding*), i64 2 }, align 8


; BAD
%Binding = type { %Action, i8 }
%Action = type { i32, [1 x i32], [2 x i64] }

@_ZN5newsf10V_BINDINGS17h490c5b52b95657e8E = internal constant { %Binding*, i64 } { %Binding* bitcast ({
    { { i32, [20 x i8] }, i8 },
    { { i32, [20 x i8] }, i8 }
}* @ref8293 to %Binding*), i64 2 }, align 8
