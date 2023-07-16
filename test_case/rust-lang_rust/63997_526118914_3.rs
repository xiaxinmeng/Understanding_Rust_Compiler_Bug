rust
trait Bar { const fn map(self) -> Self; } 
impl Bar for i32 { ... }
impl Bar for u32 { ... }
// or const impl Bar for i32 { ... {
