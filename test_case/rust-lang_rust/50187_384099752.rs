
#![feature(proc_macro)]
use std::result;
mod module {
    pub fn result() {}
}
pub use self::module::result;
pub use self::result as rename;
