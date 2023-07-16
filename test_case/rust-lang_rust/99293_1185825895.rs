rust
builder.ensure(crate::compile::Std::new(compiler, compiler.host));
builder.ensure(crate::compile::Std::new(compiler, target));
cargo.arg("--all-targets");
