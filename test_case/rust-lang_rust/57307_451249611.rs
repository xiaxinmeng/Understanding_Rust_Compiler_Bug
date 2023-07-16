rust
// This would work for any closure.
impl<'a, F> Pattern<'a> for F where F: FnMut(char) -> bool { ... }

// This would only work for path expressions to fn items
//    and type-annotated locals.
// (even annotated closures like `|c: &char| true` will select the `for F` impl
//  during type inference and cause type-checking to fail, as matching this
//  second impl would require a coercion)
impl<'a, F> Pattern<'a> for fn(&char) -> bool { ... }
