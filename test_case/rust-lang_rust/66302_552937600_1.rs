rust
         const fn foo<T: Trait>() -> usize {
             T::ASSOC_CONST.fetch_add(1, Ordering::Relaxed)
         }
         