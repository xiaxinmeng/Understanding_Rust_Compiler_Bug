rust
> fn total_fp_ord(self) -> i64 {
>     let bi = self.to_bits() as i64;
>     bi^ (((bi >> 63) as u64) >> 1) as i64
> }
> 