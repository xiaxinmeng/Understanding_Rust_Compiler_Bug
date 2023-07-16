rust
pub struct Isa<const PREFIX: &'static str> {}

pub type Cargo = Isa<{ "riscv " }>;
pub type Gcc = Isa<{ "rv" }>;
