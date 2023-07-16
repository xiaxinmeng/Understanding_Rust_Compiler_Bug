console
[misdreavus@tonberry allocator_api]$ cargo +stable test --doc
   Compiling semver-parser v0.7.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling allocator_api v0.5.0 (/home/misdreavus/clones/allocator_api)
    Finished dev [unoptimized + debuginfo] target(s) in 2.09s
   Doc-tests allocator_api

running 11 tests
test src/liballoc/boxed.rs - boxed::Box<T, A>::into_raw (line 186) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::from_raw_in (line 148) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::clone (line 320) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::clone_from (line 339) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::leak (line 224) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::new_in (line 47) ... ok
test src/liballoc/boxed.rs - boxed::Box<T, A>::leak (line 240) ... ok
test src/liballoc/boxed.rs - boxed::Box<T>::from_raw (line 119) ... ok
test src/liballoc/boxed.rs - boxed::Box<T>::new (line 87) ... ok
test src/liballoc/raw_vec.rs - raw_vec::RawVec<T, A>::double (line 278) ... ok
test src/liballoc/raw_vec.rs - raw_vec::RawVec<T, A>::reserve (line 484) ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[misdreavus@tonberry allocator_api]$ rustdoc +stable --version
rustdoc 1.31.1 (b6c32da9b 2018-12-18)
