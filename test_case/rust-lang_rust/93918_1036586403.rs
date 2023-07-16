plain
   Compiling toml v0.5.7
error: unnecessary parentheses around `if` condition
   --> src/bootstrap/native.rs:267:12
    |
267 |         if (builder.config.llvm_tools_enabled || builder.config.llvm_static_stdcpp) {
    |            ^                                                                      ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
    |
267 -         if (builder.config.llvm_tools_enabled || builder.config.llvm_static_stdcpp) {
267 +         if builder.config.llvm_tools_enabled || builder.config.llvm_static_stdcpp {

error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:01:21
