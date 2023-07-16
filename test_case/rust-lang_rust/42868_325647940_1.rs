rust
type F<'early1, 'early2> = for<'late> fn(&'late u8, &'early1 u8, &/*'elided*/u8) -> &'early2 u8;
