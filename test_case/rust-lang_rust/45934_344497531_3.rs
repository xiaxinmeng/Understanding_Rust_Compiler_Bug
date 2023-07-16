rust
pub fn g() {} // (1)
macro m() {
    pub fn g() {} // (2)

    mod bar {
        pub fn g() {} // (3)
        use self::g as g3; // Resolves to (3) today
        use super::g as g1; // Resolves to (1) today
        use macro::g as g2; // Would resolve to (2)
    }
}

// ...

m!(); //^ The above statements are true wherever `m!()` is invoked.
