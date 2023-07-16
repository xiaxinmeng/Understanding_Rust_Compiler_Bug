rust
#[cfg(target_arch = "x86_64")]
type MyAlias: MyTrait = A;
#[cfg(target_arch = "riscv64")]
type MyAlias: MyTrait = B;
