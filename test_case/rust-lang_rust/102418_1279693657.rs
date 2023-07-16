
build% ~/.rustup/toolchains/nightly-x86_64-unknown-illumos/bin/rustfmt compiler/rustc_codegen_ssa/src/back/linker.rs
error: expected identifier, found keyword `let`
   --> /data/omnios-build/omniosorg/rust/compiler/rustc_codegen_ssa/src/back/linker.rs:676:17
    |
675 |             let res: io::Result<()> = try {
    |                                       --- while parsing this struct
676 |                 let mut f = BufWriter::new(File::create(&path)?);
    |                 ^^^ expected identifier, found keyword
