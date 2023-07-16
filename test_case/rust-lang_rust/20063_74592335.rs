
rustc -Z unstable-options --pretty=expanded libcore/lib.rs | grep 'impl <.*PartialEq '
