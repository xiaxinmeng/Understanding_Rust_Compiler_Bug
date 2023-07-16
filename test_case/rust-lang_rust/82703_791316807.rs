rust
unsafe { NonZeroU8::unchecked_new(nz.get().saturating_add(nz.get())) }
