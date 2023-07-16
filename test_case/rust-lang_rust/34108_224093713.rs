
// Builtin non-local prelude from std
extern crate std; // <- injected automatically into the crate root
#[prelude_import] use std::prelude::v1::*; // <- injected automatically into the crate root

// Custom local prelude
#[prelude_import] use ::whatever::module::i::want; // <- written manually at the crate root, overwrites previous #[prelude_import] imports, module name is not hardcoded
