
-----------------------------------------------------------------------------
little-endian
-----------------------------------------------------------------------------
SipHasher128, old code
- write_u32(0xDDCCBBAA)
  - short_write([AA, BB, CC, DD])
  - needed = 8, fill = 4
  - self.tail |= u8to64_le(msg, 0, 4) << 0 --> 0xDDCCBBAA
- write_u8(0xEE)
  - short_write([EE])
  - needed = 4, fill = 1
  - self.tail |= u8to64_le(msg, 0, 1) << 4*8 --> 0xEE_CCDDBBAA
- write_u32(0xIIHHGGFF)
  - short_write([FF, GG, HH, II])
  - needed = 3, fill = 3
  - self.tail |= u8to64_le(msg, 0, 3) << 5*8 --> 0xHHGGFF_EE_CCDDBBAA
  - process
  - self.tail = u8to64_le(msg, 3, 1) --> 0xII

SipHasher128, new code
- write_u32(0xDDCCBBAA)
  - short_write(0x00000000_DDCCBBAA)
  - needed = 8, fill = 4
  - self.tail |= x << 0 --> 0xDDCCBBAA
- write_u8(0xEE)
  - short_write(0x00000000_000000EE)
  - needed = 4, fill = 1
  - self.tail |= x << 4*8 --> 0xEE_CCDDBBAA
- write_u32(0xIIHHGGFF)
  - short_write(0x00000000_IIHHGGFF)
  - needed = 3, fill = 3
  - self.tail |= x << 5*8 --> 0xHHGGFF_EE_CCDDBBAA
  - process
  - self.tail = x >> 3*8 --> 0xII

-----------------------------------------------------------------------------
big-endian
-----------------------------------------------------------------------------
SipHasher128, old code
- write_u32(0xDDCCBBAA)
  - short_write([DD, CC, BB, AA])
  - needed = 8, fill = 4
  - self.tail |= u8to64_le(msg, 0, 4) << 0 --> 0xAABBCCDD
- write_u8(0xEE)
  - short_write([EE])
  - needed = 4, fill = 1
  - self.tail |= u8to64_le(msg, 0, 1) << 4*8 --> 0xEE_AABBCCDD
- write_u32(0xIIHHGGFF)
  - short_write([II, HH, GG, FF])
  - needed = 3, fill = 3
  - self.tail |= u8to64_le(msg, 0, 3) << 5*8 --> 0xGGHHII_EE_AABBCCDD
  - process
  - self.tail = u8to64_le(msg, 3, 1) --> 0xFF

SipHasher128, new code
- write_u32(0xDDCCBBAA)
  - short_write(0x00000000_AABBCCDD)    // was byte-swapped, then zero-extended
  - needed = 8, fill = 4
  - self.tail |= x << 0 --> 0xAABBCCDD
- write_u8(0xEE)
  - short_write(0x00000000_000000EE)    // was zero-extended
  - needed = 4, fill = 1
  - self.tail |= x << 4*8 --> 0xEE_AABBCCDD
- write_u32(0xIIHHGGFF)
  - short_write(0x00000000_FFGGHHII)    // was byte-swapped, then zero-extended
  - needed = 3, fill = 3
  - self.tail |= x << 5*8 --> 0xGGHHII_EE_AABBCCDD
  - process
  - self.tail = x >> 3*8 --> 0xFF
