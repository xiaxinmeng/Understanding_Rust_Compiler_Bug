
pub struct SomeType<T> {
    // Some fields that are intentionally private...
}

impl<T> SomeType<T> {
    pub const DEFAULT: Self = Self {
        // Initialize fields...
    };
}

// Now we can create new `SomeType<T>` values generic over T without a function call...
let value: SomeType<i32> = SomeType::DEFAULT;

// Better example of what I actually want to allow...
pub static VALUE: SomeType<i32> = SomeType::DEFAULT;
