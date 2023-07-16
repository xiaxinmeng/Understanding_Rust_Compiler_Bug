
$ cargo run
   Compiling rust-dynamic-linker-error v0.0.0 (file:///Users/rodarmor/src/rust-dynamic-linker-error)
    Finished debug [unoptimized + debuginfo] target(s) in 0.17 secs
     Running `target/debug/rust-dynamic-linker-error`
dyld: Symbol not found: _iconv
  Referenced from: /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
  Expected in: /opt/local/lib/libiconv.2.dylib
 in /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
