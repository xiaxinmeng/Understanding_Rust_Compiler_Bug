
 warning: spurious network error (1 tries remaining): error inflating zlib stream; class=Zlib (5)
error: failed to get `cc` as a dependency of package `bootstrap v0.0.0 (/checkout/src/bootstrap)`

Caused by:
  failed to fetch `https://github.com/rust-lang/crates.io-index`

Caused by:
  error inflating zlib stream; class=Zlib (5)
failed to run: /checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
