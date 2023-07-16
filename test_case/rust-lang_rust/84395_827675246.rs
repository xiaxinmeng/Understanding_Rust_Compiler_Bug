
$ cat src/main.rs 
mod other;

fn main() {
    println!("Hello, world!");
    other::f();
}
$ cat src/other.rs 
pub fn f() {
    println!("other module");
}
$ cargo clean
$ RUSTFLAGS="-Z print-link-args -C linker_plugin_lto -C linker=clang -C link_arg=-fuse-ld=lld -C link-arg=-Wl,--plugin-opt=-lto-embed-bitcode=optimized -C codegen-units=2" cargo build --release
... extract the modules into files...
 ag 'Hello' *.ll
world.1.ll
8:@anon.fad58de7366495db4650cfefac2fcd61.0 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"Hello, world!\0A" }>, align 1
$ ag 'other' *.ll
world.2.ll
11:@anon.6a27adbd289b8ad274e2ad25e8b2371f.1.llvm.8309556541991907897 = hidden unnamed_addr constant <{ [13 x i8] }> <{ [13 x i8] c"other module\0A" }>, align 1
61:define hidden void @_ZN5world5other1f17h4cc9722557ce956eE() unnamed_addr #1 {
