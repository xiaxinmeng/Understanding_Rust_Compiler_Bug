console
  // first, enabled building the linker by setting `lld = true` in the [rust] section; then:
  $ echo '[target.wasm32-wasi]' >> config.toml
  $ echo 'wasi-root = "/.../wasi-libc/sysroot"' >> config.toml
  