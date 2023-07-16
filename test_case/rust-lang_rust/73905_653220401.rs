rust
trait Op where Self::Output: Copy {
    type Output;
}

// This now requires an explicit `where T::Output: Copy `
// like any other kind of where clause on the trait, except super traits.
fn f<T: Op>() {}
