rust
for<'late> fn f<'early1, 'early2>(a: &'late u8, b: &'early1 u8, c: &/*'elided*/u8) -> &'early2 u8 { ... }
