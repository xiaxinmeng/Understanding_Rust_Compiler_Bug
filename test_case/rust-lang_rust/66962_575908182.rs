rust
#![feature(const_generics)]

#[derive(PartialEq, Eq)]
struct Config {
    arr_size: usize
}

type CfgArray<T, const CFG: Config> = [T; {CFG.arr_size}];

struct B<const CFG: Config>
    where CfgArray<u8, CFG>:,
{
    arr: CfgArray<u8, CFG>,
}
