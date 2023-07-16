
ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h585cb2f6c97a5620
>>> referenced by unix.rs:29 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/command-group-2.0.1/src/tokio/child/unix.rs:29)
>>>               lto.tmp:(binstalk::ops::resolve::resolution::ResolutionSource::install::_$u7b$$u7b$closure$u7d$$u7d$::hde7913c78b76c629)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::hb63aaaa3987f28d3
>>> referenced by value.rs:476 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/toml-0.7.2/src/value.rs:476)
>>>               lto.tmp:(_$LT$serde..__private..de..content..ContentRefDeserializer$LT$E$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_any::haf92b445cd7a9ccc)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h19d0644e34ac2110
>>> referenced by function.rs:250 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250)
>>>               lto.tmp:(binstalk_downloader::download::Download::and_extract::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h5e1bd7c12bd7ce45)
>>> referenced by pacing.rs:78 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/quinn-proto-0.8.4/src/connection/pacing.rs:78)
>>>               lto.tmp:(_$LT$quinn..connection..ConnectionDriver$u20$as$u20$core..future..future..Future$GT$::poll::hd2f51d8b8689d4b3)
>>> referenced by varint.rs:95 (src/varint.rs:95)
>>>               lto.tmp:(_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::he2a62cd8086b8b6f)
>>> referenced 9 more times

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::hbeb38da9a5e1d98f
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(bytes::bytes_mut::BytesMut::reserve_inner::hfc095dc3131116b8)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h508a70ae588c6414
>>> referenced by unix.rs:31 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/terminal_size-0.1.17/src/unix.rs:31)
>>>               lto.tmp:(miette::eyreish::get_default_printer::h5cae39ea52af6318)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h2fae84facad90144
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(quinn_proto::connection::Connection::handle_decode::h0f1d941094d3525d)
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h9538f22fa632aa41)
>>> referenced by uint_macros.rs:972 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:972)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h9538f22fa632aa41)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h864aa1d5a15a27f2
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h9538f22fa632aa41)
>>> referenced by uint_macros.rs:972 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:972)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h9538f22fa632aa41)
>>> referenced by parser.rs:25 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/net/parser.rs:25)
>>>               lto.tmp:(trust_dns_proto::quic::quic_client_stream::QuicClientStreamBuilder::connect::_$u7b$$u7b$closure$u7d$$u7d$::hb0cf20d9b5d134f4)
>>> referenced 1 more times

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h300b9e324d3f533c
>>> referenced by parser.rs:25 (src/net/parser.rs:25)
>>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h26d2880a9c00b2a9)
>>> referenced by parser.rs:29 (src/net/parser.rs:29)
>>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h26d2880a9c00b2a9)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h851daca5434a2fc4
>>> referenced by parser.rs:29 (src/net/parser.rs:29)
>>>               lto.tmp:(std::net::parser::Parser::read_number::h3c4f9197e7f16e19)
>>> referenced by parser.rs:25 (src/net/parser.rs:25)
>>>               lto.tmp:(std::net::parser::Parser::read_number::h3c4f9197e7f16e19)
