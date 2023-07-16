
$ RUST_BACKTRACE=1 cargo +i686 test
    Finished test [unoptimized + debuginfo] target(s) in 0.36s
     Running target\debug\deps\shadowsocks-3162014de34ee4a7.exe

running 14 tests
test crypto::cipher::test_cipher::classic_bytes_to_key ... ok
test crypto::cipher::test_cipher::rc4_md5_key_iv ... ok
test crypto::sodium::test::test_rust_crypto_cipher_chacha20 ... ok
test crypto::sodium::test::test_rust_crypto_cipher_chacha20_ietf ... ok
test crypto::sodium::test::test_rust_crypto_cipher_xsalsa20 ... ok
test crypto::sodium::test::test_rust_crypto_cipher_salsa20 ... ok
test crypto::ring::test::test_ring_aes256gcm ... ok
test crypto::ring::test::test_ring_aes128gcm ... ok
test crypto::ring::test::test_ring_chacha20poly1305 ... ok
test crypto::sodium::test::test_rust_crypto_cipher_xchacha20_ietf_poly1305 ... ok
test plugin::test::generate_random_port ... ok
test crypto::rc4_md5::test::test_rc4_md5_cipher ... ok
test crypto::cipher::test_cipher::get_cipher ... ok
test crypto::table::test_table_cipher ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\sslocal-2908c456d2376e86.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\ssmanager-166ba062fe2993b3.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\ssserver-af5b63b47c70cf38.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\ssurl-7df5a221484249cc.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\socks5-10b5f04632b1e656.exe

running 2 tests
test socks5_relay_aead ... ok
test socks5_relay_stream ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\tunnel-b4f92e553d83dadc.exe

running 2 tests
test udp_tunnel ... ok
test tcp_tunnel ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\udp-d924342571b907bc.exe

running 1 test
test udp_relay ... FAILED

failures:

---- udp_relay stdout ----
thread 'udp_relay' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 10013, kind: PermissionDenied, message: "An attempt was made to access a socket in a way forbidden by its access permissions." }', tests\udp.rs:74:65
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\std\src/panicking.rs:475
   1: core::panicking::panic_fmt
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\core\src/panicking.rs:85
   2: core::option::expect_none_failed
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\core\src/option.rs:1274
   3: core::result::Result<T,E>::unwrap
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src/result.rs:1005
   4: udp::start_udp_echo_server::{{closure}}
             at .\tests/udp.rs:74
   5: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\future/mod.rs:78
   6: tokio::runtime::task::core::Core<T,S>::poll::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/core.rs:173
   7: tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\loom\std/unsafe_cell.rs:14
   8: tokio::runtime::task::core::Core<T,S>::poll
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/core.rs:158
   9: tokio::runtime::task::harness::Harness<T,S>::poll::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/harness.rs:107
  10: core::ops::function::FnOnce::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\ops/function.rs:233
  11: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src/panic.rs:318
  12: std::panicking::try::do_call
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src/panicking.rs:373
  13: ___rust_try
  14: std::panicking::try
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src/panicking.rs:337
  15: std::panic::catch_unwind
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src/panic.rs:394
  16: tokio::runtime::task::harness::Harness<T,S>::poll
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/harness.rs:89
  17: tokio::runtime::task::raw::poll
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/raw.rs:104
  18: tokio::runtime::task::raw::RawTask::poll
             at C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22/C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/raw.rs:66
  19: tokio::runtime::task::Notified<S>::run
             at C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\trust-dns-resolver-0.19.5/C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime\task/mod.rs:169
  20: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:155
  21: tokio::coop::with_budget::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:127
  22: std::thread::local::LocalKey<T>::try_with
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src\thread/local.rs:267
  23: std::thread::local::LocalKey<T>::with
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src\thread/local.rs:243
  24: tokio::coop::with_budget
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:120
  25: tokio::coop::budget
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:96
  26: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:155
  27: tokio::runtime::basic_scheduler::enter::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:213
  28: tokio::macros::scoped_tls::ScopedKey<T>::set
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\macros/scoped_tls.rs:63
  29: tokio::runtime::basic_scheduler::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:213
  30: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:123
  31: tokio::runtime::Runtime::block_on::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/mod.rs:444
  32: tokio::runtime::context::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/context.rs:72
  33: tokio::runtime::handle::Handle::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/handle.rs:76
  34: tokio::runtime::Runtime::block_on
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/mod.rs:441
  35: udp::udp_relay
             at .\tests/udp.rs:104
  36: udp::udp_relay::{{closure}}
             at .\tests/udp.rs:104
  37: core::ops::function::FnOnce::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\ops/function.rs:233
  38: core::ops::function::FnOnce::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\ops/function.rs:233
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'udp_relay' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', tests\udp.rs:139:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\std\src/panicking.rs:475
   1: core::panicking::panic_fmt
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\core\src/panicking.rs:85
   2: core::option::expect_none_failed
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\/library\core\src/option.rs:1274
   3: core::result::Result<T,E>::unwrap
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src/result.rs:1005
   4: udp::udp_relay::{{closure}}
             at .\tests/udp.rs:137
   5: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\future/mod.rs:78
   6: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:131
   7: tokio::coop::with_budget::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:127
   8: std::thread::local::LocalKey<T>::try_with
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src\thread/local.rs:267
   9: std::thread::local::LocalKey<T>::with
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\std\src\thread/local.rs:243
  10: tokio::coop::with_budget
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:120
  11: tokio::coop::budget
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src/coop.rs:96
  12: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:131
  13: tokio::runtime::basic_scheduler::enter::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:213
  14: tokio::macros::scoped_tls::ScopedKey<T>::set
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\macros/scoped_tls.rs:63
  15: tokio::runtime::basic_scheduler::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:213
  16: tokio::runtime::basic_scheduler::BasicScheduler<P>::block_on
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/basic_scheduler.rs:123
  17: tokio::runtime::Runtime::block_on::{{closure}}
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/mod.rs:444
  18: tokio::runtime::context::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/context.rs:72
  19: tokio::runtime::handle::Handle::enter
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/handle.rs:76
  20: tokio::runtime::Runtime::block_on
             at .\C:\Users\mateusz\.cargo\registry\src\github.com-1ecc6299db9ec823\tokio-0.2.22\src\runtime/mod.rs:441
  21: udp::udp_relay
             at .\tests/udp.rs:104
  22: udp::udp_relay::{{closure}}
             at .\tests/udp.rs:104
  23: core::ops::function::FnOnce::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\ops/function.rs:233
  24: core::ops::function::FnOnce::call_once
             at /rustc/5989bf48724031b72326a5b731a15fca101339e2\library\core\src\ops/function.rs:233
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


failures:
    udp_relay

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test udp'
