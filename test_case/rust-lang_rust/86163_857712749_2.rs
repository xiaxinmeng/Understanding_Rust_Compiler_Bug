
0:           save      %sp, -0x180, %sp
+4:          sethi     %hi(0xe374c00), %l7
+8:          add       %l7, 0x3bc, %l7
+0xc:        call      -0x2b0f6dc    <0x10c78ee8>
+0x10:       nop
+0x14:       mov       %l7, %i5
+0x18:       stx       %i0, [%fp + 0x797]
+0x1c:       stx       %i1, [%fp + 0x78f]
+0x20:       stx       %i2, [%fp + 0x787]
+0x24:       stx       %i3, [%fp + 0x77f]
+0x28:       stx       %i4, [%fp + 0x777]
+0x2c:       sethi     %hi(0x0), %g1
+0x30:       xor       %g1, 0x40, %g1
+0x34:       ldx       [%i5 + %g1], %g1
+0x38:       ldx       [%g1], %g2
+0x3c:       stx       %g2, [%fp + 0x7f7]
+0x40:       clr       %g2
+0x44:       ldx       [%fp + 0x78f], %g1
+0x48:       add       %g1, 0x50, %g1
+0x4c:       mov       %g1, %o0
+0x50:       call      +0xe02c       <_ZNK7mozilla5MaybeINS_2wr4RectIfNS1_11LayoutPixelEEEE9isNothingEv>
+0x54:       nop
+0x58:       mov       %o0, %g1
+0x5c:       xor       %g1, 0x1, %g1
+0x60:       and       %g1, 0xff, %g1
+0x64:       mov       %g1, %g2
+0x68:       clr       %g1
+0x6c:       movrne    %g2, 0x1, %g1
+0x70:       and       %g1, 0xff, %g1
+0x74:       cmp       %g1, 0x0
+0x78:       be,pt     %icc, +0x60   <_ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0xd8>
+0x7c:       nop
+0x80:       mov       0x3f7, %o2
+0x84:       sethi     %hi(0x128ecc00), %g1
+0x88:       xor       %g1, -0x398, %g1
+0x8c:       add       %i5, %g1, %g1
+0x90:       mov       %g1, %o1
+0x94:       sethi     %hi(0x128ecc00), %g1
+0x98:       xor       %g1, -0xc0, %g1
+0x9c:       add       %i5, %g1, %g1
+0xa0:       mov       %g1, %o0
+0xa4:       call      -0x5408       <MOZ_ReportAssertionFailure>
+0xa8:       nop
+0xac:       sethi     %hi(0x128ecc00), %g1
+0xb0:       xor       %g1, -0x60, %g1
+0xb4:       add       %i5, %g1, %g1
+0xb8:       mov       %g1, %o0
+0xbc:       call      -0x545c       <_ZL22AnnotateMozCrashReasonPKc>
+0xc0:       nop
+0xc4:       clr       %g2
+0xc8:       mov       0x3f7, %g1 _ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0xcc:       st        %g1, [%g2]
+0xd0:       call      +0xe440198    <PLT:abort>
+0xd4:       nop
+0xd8:       ldx       [%fp + 0x787], %g1
+0xdc:       ldx       [%g1 + 0x68], %g1
+0xe0:       stx       %g1, [%fp + 0x7a7]
+0xe4:       ldx       [%fp + 0x7a7], %g1
+0xe8:       brz,pt    %g1, +0x3c    <_ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0x124>
+0xec:       nop
+0xf0:       add       %fp, 0x72f, %g1
+0xf4:       ldx       [%fp + 0x7a7], %o1
+0xf8:       mov       %g1, %o0
+0xfc:       call      +0x5608       <_ZN7mozilla2wrL17ToLayoutTransformINS_3gfx12UnknownUnitsES3_EENS0_11Transform3DIfNS0_11LayoutPixelES5_EERKNS2_14Matrix4x4TypedIT_T0_fEE>
+0x100:      nop
+0x104:      add       %fp, 0x7b7, %g1
+0x108:      add       %fp, 0x72f, %g2
+0x10c:      mov       0x40, %g3
+0x110:      mov       %g3, %o2
+0x114:      mov       %g2, %o1
+0x118:      mov       %g1, %o0
+0x11c:      call      +0xe440d0c    <PLT:memcpy>
+0x120:      nop
+0x124:      ldx       [%fp + 0x7a7], %g1
+0x128:      brz,pt    %g1, +0x14    <_ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0x13c>
+0x12c:      nop
+0x130:      add       %fp, 0x7b7, %g1
+0x134:      ba,pt     %xcc, +0xc    <_ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0x140>
+0x138:      nop
+0x13c:      clr       %g1
+0x140:      stx       %g1, [%fp + 0x7af]
+0x144:      ldx       [%fp + 0x787], %g1
+0x148:      add       %g1, 0x40, %g1
+0x14c:      mov       %g1, %o0
+0x150:      call      -0x2ac7a84    <_ZNK13nsTArray_baseI27nsTArrayInfallibleAllocator30nsTArray_RelocateUsingMemutilsE6LengthEv>
+0x154:      nop
+0x158:      mov       %o0, %i4
+0x15c:      ldx       [%fp + 0x787], %g1
+0x160:      add       %g1, 0x48, %g1
+0x164:      mov       %g1, %o0
+0x168:      call      -0x2ac7a9c    <_ZNK13nsTArray_baseI27nsTArrayInfallibleAllocator30nsTArray_RelocateUsingMemutilsE6LengthEv>
+0x16c:      nop
+0x170:      mov       %o0, %g1
+0x174:      mov       %g1, %o2
+0x178:      mov       %i4, %o1
+0x17c:      sethi     %hi(0x128ec800), %g1
+0x180:      xor       %g1, -0x3f8, %g1
+0x184:      add       %i5, %g1, %g1
+0x188:      mov       %g1, %o0
+0x18c:      call      +0xe445dfc    <PLT:printf>
+0x190:      nop
+0x194:      ldx       [%fp + 0x787], %g3
+0x198:      ldx       [%fp + 0x777], %g1
+0x19c:      ld        [%g1], %g2
+0x1a0:      srl       %g2, 0x0, %g2
+0x1a4:      sllx      %g2, 0x20, %g2
+0x1a8:      ld        [%g1 + 0x4], %g1
+0x1ac:      srl       %g1, 0x0, %g1
+0x1b0:      or        %g1, %g2, %g2
+0x1b4:      ldx       [%fp + 0x77f], %g1
+0x1b8:      ld        [%g1], %f11
+0x1bc:      ld        [%g1 + 0x4], %f10
+0x1c0:      ld        [%g1 + 0x8], %f9
+0x1c4:      ld        [%g1 + 0xc], %f8
+0x1c8:      mov       %g3, %o4
+0x1cc:      mov       %g2, %o3
+0x1d0:      sethi     %hi(0x75bcc00), %g1
+0x1d4:      or        %g1, 0x115, %o2
+0x1d8:      fmovs     %f11, %f0
+0x1dc:      fmovs     %f10, %f1
+0x1e0:      fmovs     %f9, %f2
+0x1e4:      fmovs     %f8, %f3
+0x1e8:      call      +0xa86f234    <wr_dp_push_stacking_context>
+0x1ec:      nop
+0x1f0:      mov       %l0, %g1
+0x1f4:      and       %g1, 0xff, %g1
+0x1f8:      sllx      %g1, 0x38, %g1
+0x1fc:      mov       %g1, %o1
+0x200:      ldx       [%fp + 0x797], %o0
+0x204:      call      +0xdec4       <_ZN7mozilla5MaybeINS_2wr11WrSpatialIdEEC1ENS_7NothingE>
+0x208:      nop
+0x20c:      sethi     %hi(0x0), %g1
+0x210:      xor       %g1, 0x40, %g1
+0x214:      ldx       [%i5 + %g1], %g1
+0x218:      ldx       [%fp + 0x7f7], %g2
+0x21c:      ldx       [%g1], %g1
+0x220:      xor       %g2, %g1, %g2
+0x224:      clr       %g1
+0x228:      mov       %g2, %g1
+0x22c:      brz,pt    %g1, +0x10    <_ZN7mozilla2wr18DisplayListBuilder19PushStackingContextERKNS0_21StackingContextParamsERKNS0_4RectIfNS0_11LayoutPixelEEERKNS0_11RasterSpaceE+0x23c>
+0x230:      nop
+0x234:      call      +0xe440074    <PLT:__stack_chk_fail>
+0x238:      nop
+0x23c:      ldx       [%fp + 0x797], %i0
+0x240:      return    %i7 + 0x8
+0x244:      nop
