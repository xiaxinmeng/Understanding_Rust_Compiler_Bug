 rust
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate std;
extern crate native;
extern crate serialize;
use std::prelude::*;
use serialize::json;

struct Foo {
    field1: String,
    field2: String,
}
#[automatically_derived]
impl <__D: ::serialize::Decoder<__E>, __E> ::serialize::Decodable<__D, __E>
     for Foo {
    fn decode(__arg_0: &mut __D) -> ::std::result::Result<Foo, __E> {
        __arg_0.read_struct("Foo", 2u,
                            |_d|
                                ::std::result::Ok(Foo{field1:
                                                          match _d.read_struct_field("field1",
                                                                                     0u,
                                                                                     |_d|
                                                                                         ::serialize::Decodable::decode(_d))
                                                              {
                                                              Ok(__try_var) =>
                                                              __try_var,
                                                              Err(__try_var)
                                                              =>
                                                              return Err(__try_var)
                                                          },
                                                      field2:
                                                          match _d.read_struct_field("field2",
                                                                                     1u,
                                                                                     |_d|
                                                                                         ::serialize::Decodable::decode(_d))
                                                              {
                                                              Ok(__try_var) =>
                                                              __try_var,
                                                              Err(__try_var)
                                                              =>
                                                              return Err(__try_var)
                                                          },}))
    }
}

fn main() {
    let data =
        r##"
    {
        "field1": "something",
        "non_existing_field": "something_else"
    }
    "##;
    let _ = json::decode::<Foo>(data.as_slice());
}
