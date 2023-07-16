
extern crate serialize;
extern crate test;

#[deriving(Encodable)]
pub struct JsonSerialized  {
    really_long_field_name: uint,
}

pub struct MyServer;

pub const ALT_BEHAVIOUR: bool = false;

impl MyServer {
    fn get_json_str(&self) -> &str {
        if ! ALT_BEHAVIOUR {
            serialize::json::encode(&JsonSerialized {
                really_long_field_name: 0,}).as_slice().clone()
        } else {
            let s0 = serialize::json::encode(&JsonSerialized { really_long_field_name: 0, });
            println!("s0: `{}`", s0);
            let s1 = s0.as_slice();
            println!("s1: `{}`", s1);
            let s2 = s1.clone();
            println!("s2: `{}`", s2);
            s2
        }
    }
}

fn mess_the_heap() {
    let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into_string();
    test::black_box(s);
}

fn main() {
    let srv = MyServer;
    let s = srv.get_json_str();
    mess_the_heap();
    println!("s: `{}`", s);
}
