 rust
(self.as_ref():&[u8]).to_rmo_with(pool)
(self.as_ref(): &[u8]).to_rmo_with(pool) // current rustfmt output
self.as_ref():&[u8].to_rmo_with(pool)
self.as_ref::<[u8]>().to_rmo_with(pool)
self.as_ref<[u8]>().to_rmo_with(pool) // what we wanted, but can't get
self.as_bytes().to_rmo_with(pool)
