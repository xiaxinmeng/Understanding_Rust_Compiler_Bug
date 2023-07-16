 rust
// Not breaking, as far as I can tell (unless we define integer fallback to have higher precedence)

// From:
fn foo(x: u32) { }
// To:
fn foo<T=u32>(x: T) { }

// Breaking (but that's okay IMO)

// From:
fn foo<T>(x: T) { }
// To:
fn foo<T=u32>(x: T) { }
