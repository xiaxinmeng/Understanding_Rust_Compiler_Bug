zsh
al@eggy rust % rustc +nightly ./src/test/codegen/simd-wide-sum.rs --edition=2021 -Zmir-opt-level=3 -Zvalidate-mir -Zfuel=simd_wide_sum=46
warning: optimization-fuel-exhausted: Inline Instance { def: Item(WithOptConstParam { did: DefId(2:23390 ~ core[a873]::core_simd::vector::{impl#0}::to_array), const_param_did: None }), substs: [u8, Const { ty: usize, val: Value(Scalar(0x0000000000000008)) }] } into MirSource { instance: Item(WithOptConstParam { did: DefId(0:10 ~ simd_wide_sum[31d5]::wider_reduce_into_iter), const_param_did: None }), promoted: None }

warning: 1 warning emitted

al@eggy rust % rustc +nightly ./src/test/codegen/simd-wide-sum.rs --edition=2021 -Zmir-opt-level=3 -Zvalidate-mir -Zfuel=simd_wide_sum=47
warning: optimization-fuel-exhausted: Inline Instance { def: Item(WithOptConstParam { did: DefId(2:8403 ~ core[a873]::iter::traits::iterator::Iterator::map), const_param_did: None }), substs: [std::array::IntoIter<u8, 8_usize>, u16, fn(u8) -> u16 {<u16 as std::convert::From<u8>>::from}] } into MirSource { instance: Item(WithOptConstParam { did: DefId(0:10 ~ simd_wide_sum[31d5]::wider_reduce_into_iter), const_param_did: None }), promoted: None }

Invalid bitcast
  %6 = bitcast <8 x i8> %5 to [8 x i8]
in function wider_reduce_into_iter
LLVM ERROR: Broken function found, compilation aborted!

al@eggy rust % rustc +nightly --version --verbose
rustc 1.63.0-nightly (99930ac7f 2022-06-11)
binary: rustc
commit-hash: 99930ac7f8cbb5d9b319b2e2e92794fd6f24f556
commit-date: 2022-06-11
host: x86_64-apple-darwin
release: 1.63.0-nightly
LLVM version: 14.0.5
