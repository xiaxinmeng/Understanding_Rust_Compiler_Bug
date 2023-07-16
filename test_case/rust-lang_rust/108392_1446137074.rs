
echo "env RUSTFLAGS=$RUSTFLAGS"
env RUSTFLAGS=-Z share-generics -C linker-plugin-lto  -C link-arg=-Wl,--icf=safe
cargo-zigbuild build  --release --target armv7-unknown-linux-gnueabihf.2.17 -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --config='profile.release.split-debuginfo="packed"' --config=profile.release.debug=2 --features static,rustls,trust-dns,fancy-no-backtrace,zstd-thin,log_release_max_level_debug,cross-lang-fat-lto
   Compiling cargo-binstall v0.20.1 (/Users/nobodyxu/Dev/cargo-binstall/crates/bin)
error: linking with `/Users/nobodyxu/Library/Caches/cargo-zigbuild/0.16.2/zigcc-armv7-unknown-linux-gnueabihf.2.17.sh` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin:/Users/nobodyxu/.wasmer/bin:/Users/nobodyxu/.wasmtime/bin:/Users/nobodyxu/.local/bin:/Users/nobodyxu/Library/Python/3.10/bin:/Users/nobodyxu/Library/Python/3.9/bin:/opt/homebrew/opt/llvm/bin:/Users/nobodyxu/Library/Python/3.8/bin:/opt/homebrew/bin:/opt/homebrew/sbin:/usr/local/bin:/System/Cryptexes/App/usr/bin:/usr/bin:/bin:/usr/sbin:/sbin:/Library/TeX/texbin:/Users/nobodyxu/.cargo/bin:/Users/nobodyxu/.wasmer/globals/wapm_packages/.bin" VSLANG="1033" "/Users/nobodyxu/Library/Caches/cargo-zigbuild/0.16.2/zigcc-armv7-unknown-linux-gnueabihf.2.17.sh" "/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/symbols.o" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/deps/cargo_binstall-034005960a70ec2c.cargo_binstall.c1e0cf3f-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/deps" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/release/deps" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/build/bzip2-sys-e260e71e9683d6c2/out/lib" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/build/lzma-sys-7bf6d81e3ac313d8/out" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/build/zstd-sys-a94ed1b6652639e9/out" "-L" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/build/ring-4a2871aa10ce2276/out" "-L" "/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/armv7-unknown-linux-gnueabihf/lib" "-Wl,-Bstatic" "/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib" "/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/liblzma_sys-3f4791b9f78b1a3d.rlib" "/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libbzip2_sys-f100c6f9c714fbe5.rlib" "/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libring-92a2bff5da05ef49.rlib" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/deps/libcompiler_builtins-ea9bd470e6fc855d.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-Wl,-plugin-opt=O2,-plugin-opt=mcpu=generic" "-L" "/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/armv7-unknown-linux-gnueabihf/lib" "-o" "/Users/nobodyxu/Dev/cargo-binstall/target/armv7-unknown-linux-gnueabihf/release/deps/cargo_binstall-034005960a70ec2c" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,--strip-all" "-nodefaultlibs" "-Wl,--icf=safe"
  = note: warning: unsupported linker arg: -plugin-opt
          warning: unsupported linker arg: O2
          warning: unsupported linker arg: -plugin-opt
          warning: unsupported linker arg: mcpu=generic
          warning: unsupported linker arg: --icf
          warning: unsupported linker arg: safe
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(zstd_common.o at 119596)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(error_private.o at 47236)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(zstd_decompress.o at 2073264)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(zstd_ddict.o at 2035772)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(huf_decompress.o at 1966936)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(entropy_common.o at 12076)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(fse_decompress.o at 54884)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: warning: Linking two modules of different target triples: '/var/folders/4h/3pck4_r16tn6960znvv4w0nw0000gn/T/rustccZtFJh/libzstd_sys-64f575b0c38a9396.rlib(zstd_decompress_block.o at 2257684)' is 'armv6kz-unknown-linux-gnueabihf' whereas 'ld-temp.o' is 'armv7-unknown-linux-gnueabihf'
          
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h162454b1bee8ad3c
          >>> referenced by unix.rs:29 (/Users/nobodyxu/.cargo/registry/src/github.com-1ecc6299db9ec823/command-group-2.0.1/src/tokio/child/unix.rs:29)
          >>>               lto.tmp:(binstalk::ops::resolve::resolution::ResolutionSource::install::_$u7b$$u7b$closure$u7d$$u7d$::h0b4a5b4e0291ee15)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::hccc33318f84d4a0f
          >>> referenced by value.rs:476 (/Users/nobodyxu/.cargo/registry/src/github.com-1ecc6299db9ec823/toml-0.7.2/src/value.rs:476)
          >>>               lto.tmp:(_$LT$serde..__private..de..content..ContentRefDeserializer$LT$E$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_any::h1383e06dbfc9c9e5)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h89552025043e439b
          >>> referenced by uint_macros.rs:942 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:942)
          >>>               lto.tmp:(bytes::bytes_mut::BytesMut::reserve_inner::h02eb8cadf8c50b3e)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h55dd6fa0ceaae35e
          >>> referenced by unix.rs:31 (/Users/nobodyxu/.cargo/registry/src/github.com-1ecc6299db9ec823/terminal_size-0.1.17/src/unix.rs:31)
          >>>               lto.tmp:(miette::eyreish::get_default_printer::h279f3e817bba8be1)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h4eb04d05023f007e
          >>> referenced by pacing.rs:78 (/Users/nobodyxu/.cargo/registry/src/github.com-1ecc6299db9ec823/quinn-proto-0.8.4/src/connection/pacing.rs:78)
          >>>               lto.tmp:(_$LT$quinn..connection..ConnectionDriver$u20$as$u20$core..future..future..Future$GT$::poll::hcde9dd238955be6a)
          >>> referenced by varint.rs:95 (src/varint.rs:95)
          >>>               lto.tmp:(_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h47468ab1c2dfccac)
          >>> referenced by varint.rs:95 (src/varint.rs:95)
          >>>               lto.tmp:(_$LT$quinn_proto..config..TransportConfig$u20$as$u20$core..default..Default$GT$::default::h1dc2b3eaf7c62822)
          >>> referenced 8 more times
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h046479349e85827a
          >>> referenced by uint_macros.rs:942 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:942)
          >>>               lto.tmp:(quinn_proto::connection::Connection::handle_decode::hab52476dcdc20b11)
          >>> referenced by uint_macros.rs:942 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:942)
          >>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h48fbc6afd2817205)
          >>> referenced by uint_macros.rs:990 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:990)
          >>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h48fbc6afd2817205)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h10407e5d2dbaeec1
          >>> referenced by uint_macros.rs:942 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:942)
          >>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h48fbc6afd2817205)
          >>> referenced by uint_macros.rs:990 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:990)
          >>>               lto.tmp:(_$LT$futures_util..future..try_future..try_flatten..TryFlatten$LT$Fut$C$$LT$Fut$u20$as$u20$futures_core..future..TryFuture$GT$..Ok$GT$$u20$as$u20$core..future..future..Future$GT$::poll::h48fbc6afd2817205)
          >>> referenced by parser.rs:25 (/Users/nobodyxu/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/net/parser.rs:25)
          >>>               lto.tmp:(trust_dns_proto::quic::quic_client_stream::QuicClientStreamBuilder::connect::_$u7b$$u7b$closure$u7d$$u7d$::hf734dfd91c9bf3ac)
          >>> referenced 1 more times
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::h8018dd4223bc8a68
          >>> referenced by parser.rs:25 (src/net/parser.rs:25)
          >>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h602025f872c7eb38)
          >>> referenced by parser.rs:29 (src/net/parser.rs:29)
          >>>               lto.tmp:(std::net::parser::Parser::read_ipv4_addr::h602025f872c7eb38)
          
          ld.lld: error: undefined symbol: _$LT$T$u20$as$u20$core..convert..TryInto$LT$U$GT$$GT$::try_into::hf96b91c8ce55ce84
          >>> referenced by parser.rs:29 (src/net/parser.rs:29)
          >>>               lto.tmp:(std::net::parser::Parser::read_number::had37b5d438f58ade)
          >>> referenced by parser.rs:25 (src/net/parser.rs:25)
          >>>               lto.tmp:(std::net::parser::Parser::read_number::had37b5d438f58ade)
          

error: could not compile `cargo-binstall` due to previous error
error: Recipe `build` failed on line 158 with exit code 101
