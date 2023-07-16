rust
// The known type of the expression
let expr: &mut Foo = &mut f;

// Placeholder for the function argument
let arg: &mut ?0 = expr; // (1)
let func = foo::<?0>;
func(arg)
