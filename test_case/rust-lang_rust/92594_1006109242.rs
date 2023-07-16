rust
#[derive(RustcDecodable, RustcEncodable)]
struct Bar {
    f: u32,
}

fn main() {
    println!("{}", rustc_serialize::json::encode(&Bar { f: 3 }).unwrap());
}
