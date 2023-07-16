 rust
// Note that we don't use pnacl-ld here. We want to avoid the costly post-link
// simplification passes pnacl-ld runs (it runs -O *post link*). Instead,
// since pnacl-ld's output is just bitcode, what we do here is pull all of
// our 'native' dependencies into a (very large) module and, if requested,
// run the LTO passes on it.
