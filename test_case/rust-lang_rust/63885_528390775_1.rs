
mod rustc_serialize {
    pub use serialize::*;
}

// `::rustc_serialize` no longer resolves to the module in the root on 2018 edition
#[derive(RustcEncodable)]
struct S;
