
     Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-4730807b7cd5cc26.wasm
     Running build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/corebenches-363dc139d3325bd1.wasm
RuntimeError: unreachable
    at __rust_start_panic (wasm-function[1168]:1)
    at rust_panic (wasm-function[1156]:39)
    at _ZN3std9panicking20rust_panic_with_hook17h4caac20c72d75dc1E (wasm-function[1151]:279)
    at _ZN3std9panicking11begin_panic17h0bae38d50d36d9eeE (wasm-function[1034]:55)
    at _ZN3std3sys4wasm4time7Instant3now17he17d4f43932bec50E (wasm-function[1132]:15)
    at _ZN3std4time7Instant3now17h3e49cc4c147b1168E (wasm-function[1126]:1)
    at _ZN4test5bench13ns_iter_inner17hd2aa5a154314b7aaE (wasm-function[126]:26)
    at _ZN4test5bench7Bencher4iter17hd48f4aeafd101cecE (wasm-function[333]:622)
    at _ZN4core3ops8function6FnOnce9call_once17hbb5613d41d821ee3E (wasm-function[593]:23)
    at _ZN4test28__rust_begin_short_backtrace17hcf6eae0a01d6856fE (wasm-function[791]:11)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1299187062a5e60cE (wasm-function[790]:76)
    at _ZN4test28__rust_begin_short_backtrace17heeb3fc1ca1e28307E (wasm-function[795]:10)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h989be548adadda66E (wasm-function[794]:11)
    at _ZN4test8run_test14run_test_inner17h8475abccc200e35fE (wasm-function[911]:2059)
    at _ZN4test8run_test17h8f7c713e96afa866E (wasm-function[867]:4672)
    at _ZN4test7console17run_tests_console17h4fdd5ee55c71f0b9E (wasm-function[860]:13898)
    at _ZN4test9test_main17hc4961888722dea66E (wasm-function[909]:332)
    at _ZN4test16test_main_static17h8622e732a3fec2c9E (wasm-function[910]:356)
    at _ZN11corebenches4main17h83609ba82c495beaE (wasm-function[670]:10)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h797ebef26866c867E (wasm-function[688]:25)
error: test failed, to rerun pass '-p core --bench corebenches'
