console
      $ mkdir toolchain-123 # maybe put the git hash in the name?
      $ DESTDIR=toolchain-123 ./x.py install --stage 1 compiler/rustc library/std
      $ # `/usr/local` is the default, can override with `[install] prefix = "..."` in `config.toml`
      $ rustup toolchain link 123 toolchain-123/usr/local
      $ rustc +123 -V
      rustc 1.64.0-dev
      