`rust
mod auto_trait;
pub(crate) mod types;
use rustc_middle::ty::{self};
pub(crate) use self::types::*;
