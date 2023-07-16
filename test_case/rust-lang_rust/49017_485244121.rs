
error[E0275]: overflow evaluating the requirement `&tokio::reactor::PollEvented2<_>: std::io::Read`
   --> src/xmpp/xmpp_connection.rs:257:48
    |
257 |             stanzas::make_iq_unsupported_error(id, conn.state.client.jid.clone(), from)
    |                                                ^^
    |
    = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
    = note: required because of the requirements on the impl of `std::io::Read` for `&tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<_>>`
    = note: required because of the requirements on the impl of `std::io::Read` for `&tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<_>>>`
    = note: required because of the requirements on the impl of `std::io::Read` for `&tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<tokio::reactor::PollEvented2<_>>>>`
...
