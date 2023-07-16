
(lldb) fr sel 0
frame #0: 0x000000010162c7e8 librustc-863c36ebb34cad14.dylib`rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::h8d48a4ca8ec2db40 + 760
librustc-863c36ebb34cad14.dylib`rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::h8d48a4ca8ec2db40:
->  0x10162c7e8 <+760>: movdqa xmm0, xmmword ptr [rax]
    0x10162c7ec <+764>: movdqa xmm1, xmmword ptr [rax + 0x10]
    0x10162c7f1 <+769>: mov    rcx, -0x1
    0x10162c7f8 <+776>: movq   xmm2, rcx
