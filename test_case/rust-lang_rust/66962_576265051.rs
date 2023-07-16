rust
#![feature(const_generics)]
type CfgArray<T, const N: usize> = [T; {N}];
struct B<const N: usize>
    where CfgArray<u8, N>:,
    {
        arr: CfgArray<u8, N>,
    }
