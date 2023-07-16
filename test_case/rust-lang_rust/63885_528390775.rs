rust
extern crate serialize;
use serialize as rustc_serialize;

// uses `::rustc_serialize` internally,
// on 2018 edition it means *crate* rustc_serialize,
// not a name in root module.
#[derive(RustcEncodable)]
struct S;
