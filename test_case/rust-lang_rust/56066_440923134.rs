plain
travis_time:end:35d1fefe:start=1542866082473491941,finish=1542866137407218559,duration=54933726618
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/616fe4172b688393aeee5f34935cc25733c9c062.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/2ce92beabb912d417a7314d6da83ac9b50dc2afb.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lldb' (https://github.com/rust-lang-nursery/lldb.git) registered for path 'src/tools/lldb'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/rust-sgx'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
---
[00:00:51] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
[00:00:54] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:00:55] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:00:55] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:00:55] Submodule path 'src/liblibc': checked out '1eeba38558f5f83cd62901923f4bea8eea90bf82'
[00:00:55] Submodule path 'src/rust-sgx': checked out '11f01b5487aa6ddcd9829b1cfbe44be1a29517dc'
[00:00:55] Submodule path 'src/stdsimd': checked out '2f5e78c0b8bce40e2154764d63930741d7a65ca1'
[00:00:56] Submodule path 'src/tools/clang': checked out 'd0fc1788123de9844c8088b977cd142021cea1f2'
[00:00:56] Submodule path 'src/tools/clippy': checked out 'f5d868c9edfc6c2a9310d564a2f738bec67dfd6b'
[00:00:56] Submodule path 'src/tools/lld': checked out '2a9b88b8b419d094fb2185c0ca31c28d31bdca00'
[00:00:57] Submodule path 'src/tools/lldb': checked out 'fdea743be550ed8d7b61b2c908944cdd1290a6ad'
---

[00:03:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:13: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:14: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:15: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:16: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:17: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:18: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:19: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:20: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:21: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:22: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:23: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:24: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:25: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:26: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:27: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:28: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:29: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:30: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:31: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:32: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:33: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:37: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:38: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:39: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:40: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:41: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:42: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:43: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:44: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:45: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:49: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:50: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:51: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:52: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:53: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:54: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:55: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:56: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:57: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:58: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:59: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:60: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:61: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:62: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:63: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:64: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:65: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:66: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:67: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:68: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:69: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:70: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:71: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:72: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:73: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:74: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:75: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:79: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:80: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:81: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:82: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:83: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:84: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:85: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:86: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:87: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:88: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:89: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:90: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:91: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:92: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:93: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:94: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:95: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:96: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:97: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:98: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:99: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:100: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:101: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:102: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:103: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:104: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:105: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:109: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:110: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:111: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:112: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:113: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:114: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:115: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:116: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:117: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:118: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:122: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:123: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:124: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:125: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:126: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:127: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:128: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:129: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:130: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:131: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:132: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:133: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:134: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:135: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:136: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:140: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:141: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:142: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:143: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:144: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:145: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:146: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:147: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:148: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:149: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:150: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:151: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:152: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:153: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:154: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:155: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:156: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:157: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:158: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:159: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:160: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:161: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:162: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:163: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:164: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:165: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:166: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:167: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:168: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:169: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:170: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:171: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:172: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:173: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:174: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:175: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:176: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:177: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:178: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:179: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:180: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:181: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:182: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:186: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:187: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:188: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:189: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:190: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:191: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:192: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:193: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:194: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:195: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:196: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:197: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:198: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:199: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:200: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:201: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:202: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:203: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:204: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:205: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:206: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:207: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:208: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:209: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:210: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:211: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:212: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:213: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:214: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:215: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:216: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:220: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:221: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:222: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:223: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:224: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:225: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:226: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:227: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:228: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:229: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:230: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:231: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:232: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:233: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:234: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:235: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:236: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:237: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:238: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:239: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:240: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:241: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:242: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:243: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:244: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:245: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:246: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:247: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:248: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:252: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:253: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:254: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:255: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:256: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:257: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:258: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:259: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:260: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:261: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:262: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:263: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:264: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:265: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:266: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:270: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:271: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:272: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:273: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:274: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:275: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:276: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:277: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:278: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:279: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:280: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:281: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:282: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:283: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:284: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:285: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:286: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:287: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:288: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:289: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:290: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:291: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/large_array_impl.rs:292: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:22: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:23: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:24: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:25: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:26: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:27: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:28: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:29: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:31: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:32: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:32: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:33: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:35: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:37: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:38: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:39: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:40: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:41: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:42: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:43: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:44: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:45: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:46: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:47: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:49: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:50: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:51: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:52: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:53: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:54: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:55: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:59: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:60: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:61: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:62: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:63: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:64: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:65: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:66: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:67: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:68: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:72: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:73: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:74: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:75: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:76: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:77: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:78: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:79: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:80: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:81: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:82: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:83: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:85: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:86: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:87: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:88: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:89: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:90: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:91: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:92: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:93: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:94: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:96: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:97: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:98: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:99: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:100: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:101: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:102: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:103: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:104: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:105: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:112: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:113: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:114: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:115: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:116: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:117: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:118: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:119: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:120: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:121: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:122: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:123: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:124: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:125: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:126: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:127: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:135: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:136: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:137: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:138: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:139: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:140: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:141: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:142: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:150: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:151: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:152: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:153: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:154: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:155: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:156: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:157: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:158: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:159: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:160: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:161: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:162: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:163: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:164: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:165: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:166: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:167: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:168: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:169: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:170: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:171: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:172: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:173: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:174: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:175: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:183: line longer than 100 chars
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:184: line longer than 100 chars
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:190: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:191: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:192: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:193: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:194: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:202: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:203: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:204: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:205: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:206: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:213: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:214: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:215: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:216: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:217: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:218: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:219: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:220: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:221: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:222: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:223: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:224: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:225: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:231: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:232: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:236: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:237: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:238: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:239: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:240: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:241: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:242: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:246: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:250: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:251: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:252: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:256: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:262: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:263: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:264: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:265: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:266: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:267: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:268: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:269: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:270: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:271: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:272: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:273: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:277: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:278: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:279: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:283: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:289: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:290: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:291: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:292: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:298: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:299: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:303: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:304: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:305: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:306: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:307: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:308: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:309: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:310: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:311: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:312: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:313: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:314: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:315: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:316: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:317: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:318: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:319: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:323: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:327: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:328: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:329: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:331: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:332: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:333: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:334: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:335: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:336: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:337: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:341: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:342: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:343: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:349: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:350: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:351: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:352: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:358: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:359: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:360: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:361: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:362: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:363: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:364: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:365: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:366: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:367: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:368: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:369: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:370: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:371: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:372: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:373: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:374: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:375: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:376: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:377: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:378: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:384: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:385: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:386: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:387: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:388: tab character
[00:03:46] tidy error: /checkout/src/rust-sgx/sgx-isa/src/lib.rs:389: tab character
---
travis_time:end:18eb2e14:start=1542866374810764215,finish=1542866374815638301,duration=4874086
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04075d60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:167c9cd4
travis_time:start:167c9cd4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15443071
$ dmesg | grep -i kill
