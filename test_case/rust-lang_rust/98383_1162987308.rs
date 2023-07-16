plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0425]: cannot find function `atomic_cxchg_relaxed_acquire` in module `intrinsics`
     |
     |
2652 |             (Relaxed, Acquire) => intrinsics::atomic_cxchg_relaxed_acquire(dst, old, new),
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_acqrel_acquire`
    ::: library/core/src/intrinsics.rs:79:5
     |
     |
79   |     pub fn atomic_cxchg_acqrel<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ------------------------------------------------------------------------------ similarly named function `atomic_cxchg_acqrel_acquire` defined here

error[E0425]: cannot find function `atomic_cxchg_relaxed_seqcst` in module `intrinsics`
     |
     |
2653 |             (Relaxed, SeqCst) => intrinsics::atomic_cxchg_relaxed_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_relaxed_relaxed`
    ::: library/core/src/intrinsics.rs:80:5
     |
     |
80   |     pub fn atomic_cxchg_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ------------------------------------------------------------------------------- similarly named function `atomic_cxchg_relaxed_relaxed` defined here

error[E0425]: cannot find function `atomic_cxchg_acquire_seqcst` in module `intrinsics`
     |
     |
2656 |             (Acquire, SeqCst) => intrinsics::atomic_cxchg_acquire_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_acquire_acquire`
    ::: library/core/src/intrinsics.rs:77:5
     |
     |
77   |     pub fn atomic_cxchg_acq<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     --------------------------------------------------------------------------- similarly named function `atomic_cxchg_acquire_acquire` defined here

error[E0425]: cannot find function `atomic_cxchg_release_acquire` in module `intrinsics`
     |
     |
2658 |             (Release, Acquire) => intrinsics::atomic_cxchg_release_acquire(dst, old, new),
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_seqcst_acquire`
    ::: library/core/src/intrinsics.rs:82:5
     |
     |
82   |     pub fn atomic_cxchg_failacq<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ------------------------------------------------------------------------------- similarly named function `atomic_cxchg_seqcst_acquire` defined here

error[E0425]: cannot find function `atomic_cxchg_release_seqcst` in module `intrinsics`
     |
     |
2659 |             (Release, SeqCst) => intrinsics::atomic_cxchg_release_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_seqcst_seqcst`
    ::: library/core/src/intrinsics.rs:76:5
     |
     |
76   |     pub fn atomic_cxchg<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ----------------------------------------------------------------------- similarly named function `atomic_cxchg_seqcst_seqcst` defined here

error[E0425]: cannot find function `atomic_cxchg_acqrel_seqcst` in module `intrinsics`
     |
     |
2662 |             (AcqRel, SeqCst) => intrinsics::atomic_cxchg_acqrel_seqcst(dst, old, new),
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_seqcst_seqcst`
    ::: library/core/src/intrinsics.rs:76:5
     |
     |
76   |     pub fn atomic_cxchg<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ----------------------------------------------------------------------- similarly named function `atomic_cxchg_seqcst_seqcst` defined here

error[E0425]: cannot find function `atomic_cxchgweak_relaxed_acquire` in module `intrinsics`
     |
     |
2686 |             (Relaxed, Acquire) => intrinsics::atomic_cxchgweak_relaxed_acquire(dst, old, new),
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_acqrel_acquire`
    ::: library/core/src/intrinsics.rs:88:5
     |
     |
88   |     pub fn atomic_cxchgweak_acqrel<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ---------------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_acqrel_acquire` defined here

error[E0425]: cannot find function `atomic_cxchgweak_relaxed_seqcst` in module `intrinsics`
     |
     |
2687 |             (Relaxed, SeqCst) => intrinsics::atomic_cxchgweak_relaxed_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_relaxed_relaxed`
    ::: library/core/src/intrinsics.rs:89:5
     |
     |
89   |     pub fn atomic_cxchgweak_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ----------------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_relaxed_relaxed` defined here

error[E0425]: cannot find function `atomic_cxchgweak_acquire_seqcst` in module `intrinsics`
     |
     |
2690 |             (Acquire, SeqCst) => intrinsics::atomic_cxchgweak_acquire_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_acquire_acquire`
    ::: library/core/src/intrinsics.rs:86:5
     |
     |
86   |     pub fn atomic_cxchgweak_acq<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ------------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_acquire_acquire` defined here

error[E0425]: cannot find function `atomic_cxchgweak_release_acquire` in module `intrinsics`
     |
     |
2692 |             (Release, Acquire) => intrinsics::atomic_cxchgweak_release_acquire(dst, old, new),
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_seqcst_acquire`
    ::: library/core/src/intrinsics.rs:91:5
     |
     |
91   |     pub fn atomic_cxchgweak_failacq<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     ----------------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_seqcst_acquire` defined here

error[E0425]: cannot find function `atomic_cxchgweak_release_seqcst` in module `intrinsics`
     |
     |
2693 |             (Release, SeqCst) => intrinsics::atomic_cxchgweak_release_seqcst(dst, old, new),
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_seqcst_seqcst`
    ::: library/core/src/intrinsics.rs:85:5
     |
     |
85   |     pub fn atomic_cxchgweak<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     --------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_seqcst_seqcst` defined here

error[E0425]: cannot find function `atomic_cxchgweak_acqrel_seqcst` in module `intrinsics`
     |
     |
2696 |             (AcqRel, SeqCst) => intrinsics::atomic_cxchgweak_acqrel_seqcst(dst, old, new),
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchgweak_seqcst_seqcst`
    ::: library/core/src/intrinsics.rs:85:5
     |
     |
85   |     pub fn atomic_cxchgweak<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
     |     --------------------------------------------------------------------------- similarly named function `atomic_cxchgweak_seqcst_seqcst` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to 12 previous errors
Build completed unsuccessfully in 0:02:03
