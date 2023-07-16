rust
// the proto is just: struct Whoops { boing @0 :Bool; }

pub mod schemas_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas_capnp.rs"));
}

fn fancy_serialize<
    Own: for<'a> capnp::traits::Owned<'a>,
    F: Fn(&mut <Own as capnp::traits::Owned<'_>>::Builder),
>(
    cb: F,
) -> Vec<u8> {
    let mut builder = capnp::message::Builder::new_default();
    {
        let mut root = builder.init_root();
        cb(&mut root);
    }
    capnp::serialize::write_message_to_words(&builder)
}

fn main() {
    fancy_serialize::<schemas_capnp::whoops::Owned, _>(|root| root.set_boing(true));
}
