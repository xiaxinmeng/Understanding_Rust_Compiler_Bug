
rustdoc src/libstd/std.rs
rustdoc src/libextra/extra.rs
rsync doc my.remote.server:doc
rm -rf doc
rustdoc foo.rs
rsync doc my.remote.server:doc
