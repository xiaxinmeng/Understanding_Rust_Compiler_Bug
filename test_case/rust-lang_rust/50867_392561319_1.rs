
$ cargo +nightly run --manifest-path cargo-bisect-rustc/Cargo.toml -- \
  --start 79252ff4e25d82f9fe856cb66f127b79cdace163 \
  --end 8a37c75a3a661385cc607d934c70e86a9eaf5fd7 \
  --test-dir issue50867/ -- test
[…]
searched toolchains 79252ff4e25d82f9fe856cb66f127b79cdace163 through 8a37c75a3a661385cc607d934c70e86a9eaf5fd7
regression in 5f3994f58f41818bf31efeda6af41e8060978ca5
