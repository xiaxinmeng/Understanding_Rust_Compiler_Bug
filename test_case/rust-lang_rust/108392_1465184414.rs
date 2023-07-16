
ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h2302cd0aa19169af
>>> referenced by unix.rs:29 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/command-group-2.0.1/src/tokio/child/unix.rs:29)
>>>               lto.tmp:(binstalk::ops::resolve::resolution::ResolutionSource::install::_$u7b$$u7b$closure$u7d$$u7d$::hd3abe8150b67ddfc)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::hcd19764587633cc5
>>> referenced by value.rs:476 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/toml-0.7.2/src/value.rs:476)
>>>               lto.tmp:(_$LT$serde..__private..de..content..ContentRefDeserializer$LT$E$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_any::had41023df13da389)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h788589be0451f4f8
>>> referenced by function.rs:250 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250)
>>>               lto.tmp:(binstalk_downloader::download::Download::and_extract::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h885e894613e6bcb4)
>>> referenced by pacing.rs:78 (/home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/quinn-proto-0.8.4/src/connection/pacing.rs:78)
>>>               lto.tmp:(_$LT$quinn..connection..ConnectionDriver$u20$as$u20$core..future..future..Future$GT$::poll::h97a973e09a5ac617)
>>> referenced by varint.rs:95 (src/varint.rs:95)
>>>               lto.tmp:(_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h08b2eb84e2485aee)
>>> referenced 9 more times

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h7bb25eab4ae51928
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(bytes::bytes_mut::BytesMut::reserve_inner::h898abdfc47020faa)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h28d21b2a2cd4b287
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(quinn_proto::connection::Connection::handle_decode::he269f07f6892704e)
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h1ff3b36b788fe9dc)
>>> referenced by uint_macros.rs:972 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:972)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h1ff3b36b788fe9dc)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h0b5e19f1f414a624
>>> referenced by uint_macros.rs:924 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:924)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h1ff3b36b788fe9dc)
>>> referenced by uint_macros.rs:972 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:972)
>>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h1ff3b36b788fe9dc)
>>> referenced by parser.rs:25 (/home/runner/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/net/parser.rs:25)
>>>               lto.tmp:(trust_dns_proto::quic::quic_client_stream::QuicClientStreamBuilder::connect::_$u7b$$u7b$closure$u7d$$u7d$::hc5134720629b4c19)
>>> referenced 1 more times

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h3ad5fd32bc67a7a0
>>> referenced by parser.rs:25 (src/net/parser.rs:25)
>>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h437c3dea9a4c7949)
>>> referenced by parser.rs:29 (src/net/parser.rs:29)
>>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h437c3dea9a4c7949)

ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h88a59245c56a1cc1
>>> referenced by parser.rs:29 (src/net/parser.rs:29)
>>>               lto.tmp:(std::net::parser::Parser::read_number::h32d3af77ef448570)
>>> referenced by parser.rs:25 (src/net/parser.rs:25)
>>>               lto.tmp:(std::net::parser::Parser::read_number::h32d3af77ef448570)
