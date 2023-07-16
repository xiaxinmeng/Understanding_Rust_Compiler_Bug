rust
> trait Helper: Base<Output = <Self as Helper>::Target> + Base<Output = u32> { ... }
> 