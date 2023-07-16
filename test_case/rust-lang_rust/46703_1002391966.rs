rust
mod krate {
    mod internal {
        pub struct S;
    }
    
    pub use self::internal::S as T;
}

fn main() {
    krate::T
}
