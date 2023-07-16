 rust
#[feature(macro_rules)];

macro_rules! struct_fields {
    ($Struct:ident $($field:ident),+ )
        => { $Struct { $($field: $field),+ } } ;
}

struct SchemeRelativeUrl {
    userinfo: ~str,
    host: [u8, ..4],
    port: u32,
    path: ~str
}

fn main() {
    let userinfo = ~"hi";
    let host = [1,2,3,4];
    let port = 56;
    let path = ~"/root/";
    let s = struct_fields!( SchemeRelativeUrl userinfo, host, port, path );
    println!("s: {:?}", s);
}
