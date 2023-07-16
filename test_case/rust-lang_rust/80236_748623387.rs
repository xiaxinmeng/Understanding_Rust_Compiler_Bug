plain
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `*mut ()`
     |
     |
2417 |             SeqCst => intrinsics::atomic_xchg(dst, val),


error[E0511]: invalid monomorphization of `atomic_xchg_acqrel` intrinsic: expected basic integer type, found `*mut ()`
     |
     |
2415 |             AcqRel => intrinsics::atomic_xchg_acqrel(dst, val),


error[E0511]: invalid monomorphization of `atomic_xchg_acq` intrinsic: expected basic integer type, found `*mut ()`
     |
     |
2413 |             Acquire => intrinsics::atomic_xchg_acq(dst, val),


error[E0511]: invalid monomorphization of `atomic_xchg_rel` intrinsic: expected basic integer type, found `*mut ()`
     |
     |
2414 |             Release => intrinsics::atomic_xchg_rel(dst, val),


error[E0511]: invalid monomorphization of `atomic_xchg_relaxed` intrinsic: expected basic integer type, found `*mut ()`
     |
     |
2416 |             Relaxed => intrinsics::atomic_xchg_relaxed(dst, val),

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0511`.
