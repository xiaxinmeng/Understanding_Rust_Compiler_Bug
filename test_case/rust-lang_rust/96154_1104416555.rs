rust
enum Mascot {
    Ferris,
    Corro {
        unsafe_permit: String,
    },
}

impl Mascot {
    unsafe fn get_unsafe_permit_unchecked(&self) -> &str {
        match self {
            Mascot::Corro { unsafe_permit } => {
                unsafe_permit
            }
            Mascot::Ferris => {
                std::hint::unreachable_unchecked()
            }
        }
    }
}
