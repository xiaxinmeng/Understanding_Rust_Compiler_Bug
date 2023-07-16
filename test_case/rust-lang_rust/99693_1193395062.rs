
==945806==ERROR: AddressSanitizer: stack-overflow on address 0x7ffcb5483f80 (pc 0x55e6547ae9c2 bp 0x7ffcb5484280 sp 0x7ffcb5483f80 T0)
    #0 0x55e6547ae9c2 in core::iter::adapters::chain::and_then_or_clear::hd6800037c97dc3f5 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/iter/adapters/chain.rs:286
    #1 0x55e6547ad992 in _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h975816fb506e5749 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/iter/adapters/chain.rs:50:9
    #2 0x55e6547b1545 in _$LT$alloc..boxed..Box$LT$I$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h302e16f501c42718 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/alloc/src/boxed.rs:1869:9
    #3 0x55e6547a8b64 in core::ops::function::FnOnce::call_once::h71c5c9d1d500c07c /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/ops/function.rs:248:5
    #4 0x55e6547aed15 in core::iter::adapters::chain::and_then_or_clear::hd6800037c97dc3f5 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/iter/adapters/chain.rs:287:13
    #5 0x55e6547ad992 in _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h975816fb506e5749 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/iter/adapters/chain.rs:50:9
    #6 0x55e6547b1545 in _$LT$alloc..boxed..Box$LT$I$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h302e16f501c42718 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/alloc/src/boxed.rs:1869:9
    #7 0x55e6547a8b64 in core::ops::function::FnOnce::call_once::h71c5c9d1d500c07c /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/ops/function.rs:248:5
    #8 0x55e6547aed15 in core::iter::adapters::chain::and_then_or_clear::hd6800037c97dc3f5 /rustc/93ffde6f04d3d24327a4e17a2a2bf4f63c246235/library/core/src/iter/adapters/chain.rs:287:13
