 rust
> nm --defined-only ./x*/stage1/lib/r*/x*/l*/libstd-*.so | grep -v 'expr_fn\|\.\.\|GCC' | c++filt | sort -R | head
0000000000109ac0 T fmt::rt::Piece::Piece::Argument::hqMrQ_xr.r5s$::v0.9.pre
0000000000121a14 R libc::consts::os::posix88::EKEYREJECTED::h.vIV0_KaexA$aE::v0.9.pre
00000000000863f0 T i32::Add$i32::add::h8W3PoE86stU$43aR::v0.9.pre
0000000000078520 T rt::global_heap::exchange_free_::hyPa8N3dH0UY$aZ::v0.9.pre
000000000008dba0 T u32::Default$u32::default::hITfgkj1OV_c$kOar::v0.9.pre
00000000000b0e10 T num::FromPrimitive$uint::from_uint::hZzVzAOkD6_w$JVa0::v0.9.pre
0000000000092390 T f32::Real$f32::ln_2::hcJ17WXwyr0E$BJaX::v0.9.pre
000000000008c230 T u16::CheckedDiv$u16::checked_div::htJ2QjTyy09I$jYaa::v0.9.pre
00000000000b0e90 T num::FromPrimitive$uint::from_u16::hKTglBKguS2A$JVa2::v0.9.pre
0000000000078600 t u32::glue_take::hRNP0EbNP9Rk$az
