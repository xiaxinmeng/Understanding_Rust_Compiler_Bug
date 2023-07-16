
 ❯ env RUSTFLAGS="-L ../rust-sdl" RUST_LOG=rustc=1 make                                                                              [rust-sdl_ttf/master=]
rustc -L ../rust-sdl src/./ttf.rs -o libsdl_ttf.dummy
warning: ignoring specified output filename for library.
task <unnamed> failed at 'lookup_item: id not found: 89886', /build/rust-git/src/rust/src/librustc/metadata/decoder.rs:92
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1 to get further details and report the results to github.com/mozilla/rust/issues
task <unnamed> failed at 'explicit failure', /build/rust-git/src/rust/src/librustc/rustc.rs:376
