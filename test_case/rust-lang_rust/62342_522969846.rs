
LLVM ERROR: Cannot select: 0x3ff947b5610: i32 = SystemZISD::GET_CCMASK 0x3ff943a9258, Constant:i32<15>, Constant:i32<3>
  0x3ff943a9258: i32 = srl 0x3ff947b5af0, Constant:i32<31>
    0x3ff947b5af0: i32 = add 0x3ff943a9870, Constant:i32<-536870912>
      0x3ff943a9870: i32 = SystemZISD::IPM 0x3ff943a96d0:1
        0x3ff943a96d0: i64,i32 = SystemZISD::USUBO 0x3ff943a9120, Constant:i64<1>
          0x3ff943a9120: i64,ch = CopyFromReg 0x3ff947c35e8, Register:i64 %1
            0x3ff948ee890: i64 = Register %1
          0x3ff94d3f7d0: i64 = Constant<1>
      0x3ff943a9738: i32 = Constant<-536870912>
    0x3ff947b5200: i32 = Constant<31>
  0x3ff943a8e48: i32 = Constant<15>
  0x3ff947b5880: i32 = Constant<3>
In function: _ZN15primitive_types4U12814saturating_sub17hbfa9e41384c02873E
