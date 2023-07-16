rust
fn late_explicit<'early, 'late>(_: &'late u8) -> &'early u8 { loop {} }
fn late_implicit<'early       >(_: &      u8) -> &'early u8 { loop {} }
