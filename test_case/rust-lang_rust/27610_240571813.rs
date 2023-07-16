 c
DISPATCH_INLINE DISPATCH_ALWAYS_INLINE DISPATCH_NONNULL_ALL DISPATCH_NOTHROW
void
_dispatch_once(dispatch_once_t *predicate,
        DISPATCH_NOESCAPE dispatch_block_t block)
{
    if (DISPATCH_EXPECT(*predicate, ~0l) != ~0l) {
        dispatch_once(predicate, block);
    } else {
        dispatch_compiler_barrier();
    }
    DISPATCH_COMPILER_CAN_ASSUME(*predicate == ~0l);
}
