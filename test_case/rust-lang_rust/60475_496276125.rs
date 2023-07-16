
$ rustup update 1.30.0
$ rustup run 1.30.0 cargo install thecrate
...
this Cargo does not support nightly features, but if you
switch to nightly channel you can add
`cargo-features = ["rename-dependency"]` to enable this feature
