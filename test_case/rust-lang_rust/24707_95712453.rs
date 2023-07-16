 sh
echo '#![crate_type="lib"]; fn a(_: &[str]){}' | rustc - -g
