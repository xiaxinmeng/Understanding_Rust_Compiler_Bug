llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "llvm-link"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr-unknown-unknown"

%0 = type { [0 x i8], i16, [0 x i8], [40 x i32], [0 x i8] }

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i16(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i16, i1 immarg) addrspace(1) #0

declare void @_ZN4core3num6bignum8Big32x408mul_pow217h8ca9be035823d302E() unnamed_addr addrspace(1) #1

define void @_ZN4core3num7flt2dec8strategy6dragon15format_shortest17h6b1eacf2bca21fe3E(i16 %0) unnamed_addr addrspace(1) #1 {
  %2 = alloca %0, align 1
  %3 = alloca %0, align 1
  br label %4

4:                                                ; preds = %1
  %5 = load i8, i8* undef, align 1
  br label %6

6:                                                ; preds = %4
  %7 = getelementptr inbounds %0, %0* %3, i16 0, i32 3
  %8 = bitcast [40 x i32]* %7 to i8*
  br label %9

9:                                                ; preds = %6
  %10 = icmp ult i16 undef, 40
  br i1 %10, label %11, label %16

11:                                               ; preds = %9
  store i16 1, i16* undef, align 1
  br label %12

12:                                               ; preds = %11
  call addrspace(1) void @llvm.memcpy.p0i8.p0i8.i16(i8* nonnull align 1 dereferenceable(160) undef, i8* nonnull align 1 dereferenceable(160) %8, i16 160, i1 false) #2
  br i1 undef, label %16, label %13

13:                                               ; preds = %12
  %14 = icmp ugt i16 undef, 40
  br i1 %14, label %16, label %15

15:                                               ; preds = %13
  br i1 undef, label %17, label %16

16:                                               ; preds = %15, %13, %12, %9
  unreachable

17:                                               ; preds = %15
  br i1 undef, label %21, label %18

18:                                               ; preds = %17
  br i1 undef, label %20, label %19

19:                                               ; preds = %18
  unreachable

20:                                               ; preds = %18
  br i1 undef, label %21, label %22

21:                                               ; preds = %20, %17
  unreachable

22:                                               ; preds = %20
  br i1 undef, label %36, label %24

23:                                               ; preds = %24
  br i1 undef, label %36, label %35

24:                                               ; preds = %24, %22
  %25 = phi i32 [ %32, %24 ], [ 0, %22 ]
  %26 = load i32, i32* undef, align 1
  %27 = zext i32 %26 to i64
  %28 = mul nuw nsw i64 %27, 10
  %29 = zext i32 %25 to i64
  %30 = add nuw nsw i64 %28, %29
  %31 = lshr i64 %30, 32
  %32 = trunc i64 %31 to i32
  %33 = trunc i64 %30 to i32
  %34 = icmp eq i32* undef, undef
  br i1 %34, label %23, label %24

35:                                               ; preds = %23
  br i1 undef, label %36, label %36

36:                                               ; preds = %35, %35, %23, %22
  call align 1 addrspace(1) void @_ZN4core3num6bignum8Big32x408mul_pow217h8ca9be035823d302E()
  br i1 undef, label %39, label %37

37:                                               ; preds = %36
  %38 = load i16, i16* undef, align 1
  br label %40

39:                                               ; preds = %36
  unreachable

40:                                               ; preds = %128, %37
  %41 = phi i16 [ 0, %37 ], [ %80, %128 ]
  br label %42

42:                                               ; preds = %43, %40
  br i1 undef, label %44, label %43

43:                                               ; preds = %42
  br i1 false, label %42, label %44

44:                                               ; preds = %43, %42
  %45 = select i1 undef, i16 undef, i16 undef
  %46 = icmp ugt i16 %45, 40
  br i1 %46, label %47, label %48

47:                                               ; preds = %44
  unreachable

48:                                               ; preds = %44
  %49 = getelementptr inbounds %0, %0* %3, i16 0, i32 3, i16 %45
  %50 = bitcast i32* %49 to i8*
  br label %51

51:                                               ; preds = %57, %48
  %52 = phi i8* [ %50, %48 ], [ undef, %57 ]
  %53 = phi i8* [ undef, %48 ], [ %58, %57 ]
  %54 = icmp eq i8* %52, %8
  br i1 %54, label %55, label %57

55:                                               ; preds = %51
  %56 = icmp ne i8* %53, undef
  br label %60

57:                                               ; preds = %51
  %58 = getelementptr inbounds i8, i8* %53, i16 -4
  %59 = icmp eq i8 undef, 0
  br i1 %59, label %51, label %60

60:                                               ; preds = %57, %55
  br i1 undef, label %61, label %68

61:                                               ; preds = %60
  br i1 undef, label %67, label %62

62:                                               ; preds = %62, %61
  %63 = or i1 undef, undef
  store i32 0, i32* undef, align 1
  %64 = icmp ult i16 undef, %45
  br i1 %64, label %62, label %65

65:                                               ; preds = %62
  br i1 %63, label %67, label %66

66:                                               ; preds = %65
  unreachable

67:                                               ; preds = %65, %61
  br label %68

68:                                               ; preds = %67, %60
  br i1 false, label %69, label %70

69:                                               ; preds = %68
  unreachable

70:                                               ; preds = %68
  %71 = select i1 undef, i16 undef, i16 %38
  br i1 false, label %72, label %73

72:                                               ; preds = %70
  unreachable

73:                                               ; preds = %70
  br i1 undef, label %74, label %77

74:                                               ; preds = %73
  br i1 undef, label %77, label %75

75:                                               ; preds = %75, %74
  %76 = icmp ult i16 undef, %71
  br i1 %76, label %75, label %77

77:                                               ; preds = %75, %74, %73
  %78 = icmp ult i16 %41, %0
  br i1 %78, label %79, label %133

79:                                               ; preds = %77
  %80 = add nuw i16 %41, 1
  %81 = load i16, i16* undef, align 1
  %82 = icmp ugt i16 undef, 40
  br i1 %82, label %83, label %84

83:                                               ; preds = %79
  unreachable

84:                                               ; preds = %79
  br label %85

85:                                               ; preds = %87, %84
  br i1 undef, label %86, label %87

86:                                               ; preds = %85
  br label %88

87:                                               ; preds = %85
  br i1 undef, label %85, label %88

88:                                               ; preds = %87, %86
  %89 = icmp slt i8 undef, %5
  %90 = load i16, i16* undef, align 1
  %91 = select i1 undef, i16 undef, i16 %90
  %92 = icmp ugt i16 %91, 40
  br i1 %92, label %93, label %94

93:                                               ; preds = %102, %98, %88
  unreachable

94:                                               ; preds = %88
  %95 = getelementptr inbounds %0, %0* %2, i16 0, i32 3, i16 %91
  br i1 false, label %102, label %96

96:                                               ; preds = %96, %94
  br i1 undef, label %96, label %97

97:                                               ; preds = %96
  br i1 undef, label %98, label %102

98:                                               ; preds = %97
  %99 = icmp ult i16 %91, 40
  br i1 %99, label %100, label %93

100:                                              ; preds = %98
  store i32 1, i32* %95, align 1
  %101 = add nuw nsw i16 %91, 1
  br label %102

102:                                              ; preds = %100, %97, %94
  %103 = phi i16 [ %101, %100 ], [ %91, %97 ], [ 0, %94 ]
  br i1 false, label %93, label %104

104:                                              ; preds = %102
  %105 = getelementptr inbounds %0, %0* %2, i16 0, i32 3, i16 undef
  %106 = bitcast i32* %105 to i8*
  br label %107

107:                                              ; preds = %110, %104
  %108 = phi i8* [ %106, %104 ], [ %111, %110 ]
  br i1 false, label %109, label %110

109:                                              ; preds = %107
  br label %114

110:                                              ; preds = %107
  %111 = getelementptr inbounds i8, i8* %108, i16 -4
  %112 = bitcast i8* %111 to i32*
  %113 = icmp eq i8 undef, 0
  br i1 %113, label %107, label %114

114:                                              ; preds = %110, %109
  %115 = icmp slt i8 undef, %5
  %116 = or i1 %89, %115
  br i1 %116, label %129, label %117

117:                                              ; preds = %114
  %118 = icmp ugt i16 undef, 40
  br i1 %118, label %120, label %119

119:                                              ; preds = %117
  br i1 undef, label %123, label %121

120:                                              ; preds = %117
  unreachable

121:                                              ; preds = %119
  br i1 undef, label %123, label %122

122:                                              ; preds = %121
  unreachable

123:                                              ; preds = %121, %119
  %124 = icmp ugt i16 %81, 40
  br i1 %124, label %125, label %126

125:                                              ; preds = %123
  unreachable

126:                                              ; preds = %123
  %127 = icmp ugt i16 %90, 40
  br i1 %127, label %133, label %128

128:                                              ; preds = %126
  br i1 false, label %130, label %40

129:                                              ; preds = %114
  br i1 %115, label %130, label %132

130:                                              ; preds = %129, %128
  br i1 %89, label %131, label %132

131:                                              ; preds = %130
  unreachable

132:                                              ; preds = %130, %129
  ret void

133:                                              ; preds = %126, %77
  unreachable
}

attributes #0 = { argmemonly nounwind willreturn }
attributes #1 = { "target-cpu"="atmega328" }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
