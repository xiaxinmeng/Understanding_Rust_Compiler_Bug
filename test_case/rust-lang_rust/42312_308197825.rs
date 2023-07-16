
// Hack to avoid duplicate warnings for simple cases like `fn foo(x: Trait)`, where we would error
// once on the parameter as a whole, and once on the binding `x`.
if arg.pat.simple_name().is_none() { require_type_is_sized(..) }
