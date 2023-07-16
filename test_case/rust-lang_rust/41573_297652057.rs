
[00:21:06] warning: ../compiler-rt/lib/builtins/divdc3.c:21:1: warning: conflicting types for built-in function '__divdc3'
[00:21:06] warning:  __divdc3(double __a, double __b, double __c, double __d)
[00:21:06] warning:  ^
[00:21:06] warning: ../compiler-rt/lib/builtins/divsc3.c:21:1: warning: conflicting types for built-in function '__divsc3'
[00:21:06] warning:  __divsc3(float __a, float __b, float __c, float __d)
[00:21:06] warning:  ^
[00:21:06] warning: ../compiler-rt/lib/builtins/muldc3.c:21:1: warning: conflicting types for built-in function '__muldc3'
[00:21:06] warning:  __muldc3(double __a, double __b, double __c, double __d)
[00:21:06] warning:  ^
[00:21:06] warning: ../compiler-rt/lib/builtins/mulsc3.c:21:1: warning: conflicting types for built-in function '__mulsc3'
[00:21:06] warning:  __mulsc3(float __a, float __b, float __c, float __d)
[00:21:06] warning:  ^
[00:21:13] error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:680: can't type-check body of DefId { krate: CrateNum(0), node: DefIndex(2147484335) => core/31cda99d85f5f93f25d91c8ad10a555f::intrinsics[0]::[0]::atomic_cxchg[0] }
[00:21:13]   --> /checkout/src/libcore/intrinsics.rs:67:5
[00:21:13]    |
[00:21:13] 67 |     pub fn atomic_cxchg<T>(dst: *mut T, old: T, src: T) -> (T, bool);
[00:21:13]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:21:13] 
[00:21:13] note: the compiler unexpectedly panicked. this is a bug.
[00:21:13] 
