bash
STD_DATE=$(rustc -Vv | grep commit-date | awk '{print $2}')
export RUSTDOCFLAGS="--document-private-items --crate-version \"${STD_DATE}\" --html-in-header ${PWD}/in-head.html"
cargo doc --manifest-path library/std/Cargo.toml
