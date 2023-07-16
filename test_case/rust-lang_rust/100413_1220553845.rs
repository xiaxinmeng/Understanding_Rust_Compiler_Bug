bash
$ CARGO_INCREMENTAL=0 RUSTFLAGS="-Cprofile-use=profile.prof" cargo build --release
# Change profile.prof
$ CARGO_INCREMENTAL=0 RUSTFLAGS="-Cprofile-use=profile.prof" cargo build --release
# In current rustc, the crate is not recompiled!
