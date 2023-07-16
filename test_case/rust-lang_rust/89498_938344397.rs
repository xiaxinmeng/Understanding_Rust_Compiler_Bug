
LLVM ERROR: Cannot select: 0x7fb0c9876678: i32,i32 = shl_parts 0x7fb0c98788e0, Constant:i32<0>, 0x7fb0c9876540
  0x7fb0c98788e0: i32,ch = CopyFromReg 0x7fb0c621bf58, Register:i32 %0
    0x7fb0c98769b8: i32 = Register %0
  0x7fb0c9877170: i32 = Constant<0>
  0x7fb0c9876540: i32 = add nuw nsw 0x7fb0c9878670, Constant:i32<21>
    0x7fb0c9878670: i32 = AssertZext 0x7fb0c9876af0, ValueType:ch:i8
      0x7fb0c9876af0: i32,ch = CopyFromReg 0x7fb0c621bf58, Register:i32 %2
        0x7fb0c9876a88: i32 = Register %2
    0x7fb0c9878a80: i32 = Constant<21>
In function: _ZN17compiler_builtins5float4conv11__floatsidf17hb63047be181dca88E
