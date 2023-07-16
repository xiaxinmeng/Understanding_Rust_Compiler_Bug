
mod m {
    // Tuple struct with a private field.
    // Type S is pub(pub).
    // The field S::0 is pub(m).
    // Constructor S is min(pub(pub), pub(m)) -> pub(m).
    pub struct S(u8);
    
    fn f() {
        // Try to use S from the root module in value namespace.
        // No success, ::S exists only in type namespace.
        // How to fix: use S from this module instead of ::S from the root module.
        ::S;
    }
}

// This imports S only in type namespace, value S is too private.
// This is expected filtering behavior of imports described in RFC 1560.
use m::S;

fn main() {}
