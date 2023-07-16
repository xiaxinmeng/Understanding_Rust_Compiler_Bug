
=== Starting src_install
Found a makefile, using the install target
make -j13 -j1 DESTDIR=/var/tmp/paludis/build/dev-lang-rust-1.14.0/image/ install
"/usr/bin/python2.7" /var/tmp/paludis/build/dev-lang-rust-1.14.0/work/rustc-1.14.0/src/bootstrap/bootstrap.py dist --install

    Updating registry `https://github.com/rust-lang/crates.io-index`

[...] sandbox complaining[...]

warning: spurious network error (2 tries remaining): [12/-1] curl error: Could not resolve: github.com (Could not contact DNS servers)
warning: spurious network error (1 tries remaining): [12/-1] curl error: Could not resolve: github.com (Could not contact DNS servers)
error: failed to fetch `https://github.com/rust-lang/crates.io-index`
Caused by:
  [12/-1] curl error: Could not resolve: github.com (Could not contact DNS servers)
