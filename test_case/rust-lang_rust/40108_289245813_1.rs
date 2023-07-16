
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 1:34:34
[sudo] password for user: 
Sorry, try again.
[sudo] password for user: 
info: looks like you are running this command under `sudo`
      and so in order to preserve your $HOME this will now
      use vendored sources by default. Note that if this
      does not work you should run a normal build first
      before running a command like `sudo make install`
error: failed to load source for a dependency on `toml`

Caused by:
  Unable to update registry https://github.com/rust-lang/crates.io-index

Caused by:
  failed to update replaced source `registry https://github.com/rust-lang/crates.io-index`

Caused by:
  failed to read root of directory source: /home/user/Sources/rust/src/vendor

To learn more, run the command again with --verbose.
Build completed unsuccessfully in 0:00:00
