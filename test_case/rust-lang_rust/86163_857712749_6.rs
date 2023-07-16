
wr_dp_push_stacking_context:    save      %sp, -0x150, %sp
wr_dp_push_stacking_context+4:  call      +0x8          <wr_dp_push_stacking_context+0xc>
wr_dp_push_stacking_context+8:  sethi     %hi(0x3b05800), %i0
wr_dp_push_stacking_context+0xc:or        %i0, 0x3a8, %i0
wr_dp_push_stacking_context+0x10:       add       %i0, %o7, %i0
wr_dp_push_stacking_context+0x14:       stx       %i0, [%fp + 0x75f]
wr_dp_push_stacking_context+0x18:       ldx       [%fp + 0x8af], %i1
wr_dp_push_stacking_context+0x1c:       st        %f1, [%fp + 0x787]
wr_dp_push_stacking_context+0x20:       st        %f3, [%fp + 0x78b]
wr_dp_push_stacking_context+0x24:       st        %f5, [%fp + 0x78f]
wr_dp_push_stacking_context+0x28:       st        %f7, [%fp + 0x793]
wr_dp_push_stacking_context+0x2c:       ld        [%fp + 0x787], %i1
wr_dp_push_stacking_context+0x30:       sllx      %i1, 0x20, %i1
wr_dp_push_stacking_context+0x34:       ld        [%fp + 0x78b], %i2
wr_dp_push_stacking_context+0x38:       or        %i1, %i2, %i1
wr_dp_push_stacking_context+0x3c:       stx       %i1, [%fp + 0x777]
wr_dp_push_stacking_context+0x40:       ld        [%fp + 0x78f], %i1
wr_dp_push_stacking_context+0x44:       sllx      %i1, 0x20, %i1
wr_dp_push_stacking_context+0x48:       ld        [%fp + 0x793], %i2
wr_dp_push_stacking_context+0x4c:       or        %i1, %i2, %i1
wr_dp_push_stacking_context+0x50:       stx       %i1, [%fp + 0x77f]
wr_dp_push_stacking_context+0x54:       stx       %i4, [%fp + 0x797]
wr_dp_push_stacking_context+0x58:       stx       %i5, [%fp + 0x7a7]
wr_dp_push_stacking_context+0x5c:       stx       %i5, [%fp + 0x79f]
wr_dp_push_stacking_context+0x60:       add       %fp, 0x797, %o0
wr_dp_push_stacking_context+0x64:       stx       %o0, [%fp + 0x7ef]
wr_dp_push_stacking_context+0x68:       stx       %o0, [%fp + 0x7f7]
wr_dp_push_stacking_context+0x6c:       sethi     %hi(0x1400), %i1
wr_dp_push_stacking_context+0x70:       add       %i1, 0x190, %i1
wr_dp_push_stacking_context+0x74:       call      +0x3c81078    <PLT:_ZN4core3fmt10ArgumentV13new17ha9bc9565de3aadf7E>
wr_dp_push_stacking_context+0x78:       ldx       [%i0 + %i1], %o1
wr_dp_push_stacking_context+0x7c:       stx       %o0, [%fp + 0x767]
wr_dp_push_stacking_context+0x80:       ba        +0x8          <wr_dp_push_stacking_context+0x88>
wr_dp_push_stacking_context+0x84:       stx       %o1, [%fp + 0x76f]
wr_dp_push_stacking_context+0x88:       ldx       [%fp + 0x75f], %i0
wr_dp_push_stacking_context+0x8c:       ldx       [%fp + 0x76f], %i1
wr_dp_push_stacking_context+0x90:       ldx       [%fp + 0x767], %i2
wr_dp_push_stacking_context+0x94:       stx       %i2, [%fp + 0x7df]
wr_dp_push_stacking_context+0x98:       stx       %i1, [%fp + 0x7e7]
wr_dp_push_stacking_context+0x9c:       sethi     %hi(0x36000), %i1
wr_dp_push_stacking_context+0xa0:       add       %i1, 0x50, %i1
wr_dp_push_stacking_context+0xa4:       ldx       [%i0 + %i1], %o1
wr_dp_push_stacking_context+0xa8:       add       %fp, 0x7af, %o0
wr_dp_push_stacking_context+0xac:       mov       0x2, %o2
wr_dp_push_stacking_context+0xb0:       add       %fp, 0x7df, %o3
wr_dp_push_stacking_context+0xb4:       call      +0xe2a0c      <_ZN4core3fmt9Arguments6new_v117h337fca81b2e584b1E>
wr_dp_push_stacking_context+0xb8:       mov       0x1, %o4
wr_dp_push_stacking_context+0xbc:       ba        +0x8          <wr_dp_push_stacking_context+0xc4>
wr_dp_push_stacking_context+0xc0:       nop
wr_dp_push_stacking_context+0xc4:       call      +0x3df5b00    <PLT:_ZN3std2io5stdio6_print17h02792fbd1097b851E>
wr_dp_push_stacking_context+0xc8:       add       %fp, 0x7af, %o0
wr_dp_push_stacking_context+0xcc:       ba        +0x8          <wr_dp_push_stacking_context+0xd4>
wr_dp_push_stacking_context+0xd0:       nop
wr_dp_push_stacking_context+0xd4:       ret
wr_dp_push_stacking_context+0xd8:       restore
