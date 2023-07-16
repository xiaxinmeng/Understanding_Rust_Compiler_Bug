rust
struct Foo;

impl FnOnce<()> for Foo {
    type Output = i32;
    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        1
    }
}

impl FnOnce<()> for Foo {
    type Output = Result<i32, bool>;
    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        Ok(1)
    }
}

// error[E0119]: conflicting implementations of trait `std::ops::FnOnce<()>` for type `Foo`
