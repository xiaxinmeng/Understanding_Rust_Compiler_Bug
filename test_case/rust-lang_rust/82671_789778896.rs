
   Compiling xml5ever v0.16.1
error: field is never read: `ns`
   --> tokio-xmpp/src/xmpp_codec.rs:183:5
    |
183 |     ns: Option<String>,
    |     ^^^^^^^^^^^^^^^^^^
    |
note: the lint level is defined here
   --> tokio-xmpp/src/lib.rs:3:22
    |
3   | #![deny(unsafe_code, unused, missing_docs, bare_trait_objects)]
    |                      ^^^^^^
    = note: `#[deny(dead_code)]` implied by `#[deny(unused)]`

error: aborting due to previous error

error: could not compile `tokio-xmpp`
