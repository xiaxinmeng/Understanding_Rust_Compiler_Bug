rust
fn split(&self) -> (&[u8], &[u8]);
fn before(&self) -> &[u8];
fn after(&self) -> &[u8];
fn split_mut(&mut self) -> (&mut [u8], &mut [u8]);
fn before_mut(&mut self) -> &mut [u8];
fn after_mut(&mut self) -> &mut [u8];
