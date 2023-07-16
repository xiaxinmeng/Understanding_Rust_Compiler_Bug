
me@myplace rust $ sudo ./x.py install
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
  failed to read root of directory source: /home/andrew/Projects/software-projects/systems/rust/src/vendor

Caused by:
  No such file or directory (os error 2)
failed to run: /home/andrew/Projects/software-projects/systems/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/andrew/Projects/software-projects/systems/rust/src/bootstrap/Cargo.toml --frozen
Build completed unsuccessfully in 0:00:01
