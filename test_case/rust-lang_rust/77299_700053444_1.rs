
error: proc macro panicked
   --> /Users/vasilipascal/.cargo/registry/src/github.com-1ecc6299db9ec823/rocket_http-0.4.4/src/parse/uri/parser.rs:119:34
    |
119 |             let path_and_query = pear_try!(path_and_query(is_pchar));
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: message: called `Option::unwrap()` on a `None` value
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

error: could not compile `rocket_http`

Caused by:
  process didn't exit successfully: `rustc --crate-name rocket_http /Users/vasilipascal/.cargo/registry/src/github.com-1ecc6299db9ec823/rocket_http-0.4.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="private-cookies"' -C metadata=8e117303e9a53d85 -C extra-filename=-8e117303e9a53d85 --out-dir /Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps -L dependency=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps --extern cookie=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libcookie-d9ba2c04fc8dd53b.rmeta --extern hyper=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libhyper-badc683c900ce9dc.rmeta --extern indexmap=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libindexmap-5197918e16e6ae9a.rmeta --extern pear=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libpear-2f6d117e45a52960.rmeta --extern percent_encoding=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libpercent_encoding-32c178760959eb29.rmeta --extern smallvec=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libsmallvec-0c3d025db82f3c0f.rmeta --extern state=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libstate-8302fd253462dbb4.rmeta --extern time=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libtime-75fb422068042b64.rmeta --extern unicode_xid=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/deps/libunicode_xid-f4ef1d6919cd36ed.rmeta --cap-lints allow -L native=/Users/vasilipascal/work/rust/rocket/hello-rocket/target/debug/build/ring-3b4d75d0ec10c019/out` (exit code: 1)
