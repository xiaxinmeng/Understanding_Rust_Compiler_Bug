bash
==> Installing vala dependency: rust
==> ./configure --prefix=/usr/local/Cellar/rust/1.49.0_1 --release-channel=stable
==> make
==> make install
==> ./install.sh --prefix=/private/tmp/rust-20210201-21234-2auhx1/rustc-1.49.0-src/cargobootstrap
==> cargo install /usr/local/Cellar/rust/1.49.0_1 --features curl-sys/force-system-lib-on-osx
Last 15 lines from /Users/frak/Library/Logs/Homebrew/rust/05.cargo:
    |                                                           this method call resolves to `&T`
    |
    = note: cannot satisfy `std::string::String: AsRef<_>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
error: could not compile `cargo`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: failed to compile `cargo v0.50.0 (/private/tmp/rust--cargo-20210201-21234-fgm2gx)`, intermediate artifacts can be found at `/private/tmp/rust--cargo-20210201-21234-fgm2gx/target`

Caused by:
  build failed

Do not report this issue to Homebrew/brew or Homebrew/core!

These open issues may also help:
Rust-dependent formulae on Apple Silicon - upstream issue tracker https://github.com/Homebrew/homebrew-core/issues/68301

Error: You are using macOS 10.13.
We (and Apple) do not provide support for this old version.
You will encounter build failures with some formulae.
Please create pull requests instead of asking for help on Homebrew's GitHub,
Twitter or any other official channels. You are responsible for resolving
any issues you experience while you are running this
old version.
