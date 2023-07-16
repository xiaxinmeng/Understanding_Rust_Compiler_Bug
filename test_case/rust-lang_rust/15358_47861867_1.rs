 rust
struct Serie {
    metric: String,
    type_: String,
}
impl <__D: ::serialize::Decoder<__E>, __E> ::serialize::Decodable<__D, __E>
     for Serie {
    fn decode(__arg_0: &mut __D) -> ::std::result::Result<Serie, __E> {
        __arg_0.read_struct("Serie", 2u,
                            |_d|
                                ::std::result::Ok(Serie{metric:
                                                            match _d.read_struct_field("metric",
                                                                                       0u,
                                                                                       |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                => __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var)
                                                            },
                                                        type_:
                                                            match _d.read_struct_field("type_",
                                                                                       1u,
                                                                                       |_d|
                                                                                           ::serialize::Decodable::decode(_d))
                                                                {
                                                                Ok(__try_var)
                                                                => __try_var,
                                                                Err(__try_var)
                                                                =>
                                                                return Err(__try_var)
                                                            },}))
    }
}
impl <__S: ::serialize::Encoder<__E>, __E> ::serialize::Encodable<__S, __E>
     for Serie {
    fn encode(&self, __arg_0: &mut __S) -> ::std::result::Result<(), __E> {
        match *self {
            Serie{metric: ref __self_0_0, type_: ref __self_0_1} =>
            __arg_0.emit_struct("Serie", 2u, |_e| {
                                match _e.emit_struct_field("metric", 0u,
                                                           |_e|
                                                               (*__self_0_0).encode(_e))
                                    {
                                    Ok(__try_var) => __try_var,
                                    Err(__try_var) => return Err(__try_var)
                                };
                                return _e.emit_struct_field("type_", 1u,
                                                            |_e|
                                                                (*__self_0_1).encode(_e));
                            })
        }
    }
}
