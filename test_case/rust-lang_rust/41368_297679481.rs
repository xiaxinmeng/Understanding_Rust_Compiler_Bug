
[00:20:46] error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:680: can't type-check body of DefId { krate: CrateNum(0), node: DefIndex(2147484335) => core/31cda99d85f5f93f25d91c8ad10a555f::intrinsics[0]::[0]::atomic_cxchg[0] }

[00:20:46]   --> /checkout/src/libcore/intrinsics.rs:67:5

[00:20:46]    |

[00:20:46] 67 |     pub fn atomic_cxchg<T>(dst: *mut T, old: T, src: T) -> (T, bool);

[00:20:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
