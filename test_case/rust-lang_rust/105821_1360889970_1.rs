rs
#[repr(C)]
struct ConcatJoin<const M: usize, const N: usize> {
    left: [u8; M],
    right: [u8; N],
}

#[repr(C)]
union ConcatJoiner<const M: usize, const N: usize>
where
    [(); M + N]:,
{
    whole: ManuallyDrop<[u8; M + N]>,
    split: ManuallyDrop<ConcatJoin<M, N>>,
}

const fn concat_arrs<const M: usize, const N: usize>(a: [u8; M], b: [u8; N]) -> [u8; M + N]
where
    [(); M + N]:,
{
    unsafe {
        let joiner = ConcatJoiner {
            split: ManuallyDrop::new(ConcatJoin { left: a, right: b }),
        };
        let join = joiner.whole;
        ManuallyDrop::into_inner(join)
    }
}
