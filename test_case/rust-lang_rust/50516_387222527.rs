
std::fs::DirBuilder::create (self=0xffffffffed38, path=0xaaaaaaac7c74 <str.0>) at /checkout/src/libstd/fs.rs:1976
(gdb) p *self
$11 = std::fs::DirBuilder {inner: std::sys::unix::fs::DirBuilder {mode: 511}, recursive: 254}
