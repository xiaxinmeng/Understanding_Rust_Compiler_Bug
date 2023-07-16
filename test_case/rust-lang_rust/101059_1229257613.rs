rust
#![feature(no_core)]
#![no_core]

pub mod http {
    pub struct Request;
}
pub mod pavex_runtime {
    pub use crate::http;
}
pub fn extract(_req: pavex_runtime::http::Request) {}
