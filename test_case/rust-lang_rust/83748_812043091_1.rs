
error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
   --> src\dedup.rs:178:37
    |
178 |     fn dedup(self) -> DedupBy<Self, impl FnMut(&Self::Item, &Self::Item) -> bool, Self::Item>
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
