rust
write!(f, "{}{}", UnsafetySpace(decl.unsafety), AbiSpace(decl.abi))?;
primitive_link(f, PrimitiveType::Fn, "fn")?;
write!(f, "{}{}", decl.generics, decl.decl)
