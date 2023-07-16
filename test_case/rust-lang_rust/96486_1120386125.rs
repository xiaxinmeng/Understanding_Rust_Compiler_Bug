
> strip = false  # DON'T strip the symbols. This will be done right before burning the patch.
> opt-level = "z"  # Optimize for size.
> panic = "abort"
> lto = "fat"  # (same behaviour with on)
> overflow-checks = false  # Bug: https://github.com/rust-lang/compiler-builtins/issues/347
> 