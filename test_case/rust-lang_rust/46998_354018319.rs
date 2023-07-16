
info: looks like you are running this command under `sudo`
      and so in order to preserve your $HOME this will now
      use vendored sources by default. Note that if this
      does not work you should run a normal build first
      before running a command like `sudo make install`
Updating submodules
error: failed to load source for a dependency on `cc`

Caused by:
  Unable to update registry `https://github.com/rust-lang/crates.io-index`

Caused by:
  failed to update replaced source registry `https://github.com/rust-lang/crates.io-index`

Caused by:
  failed to read root of directory source: ~/rust/src/vendor

Caused by:
  No such file or directory (os error 2)

