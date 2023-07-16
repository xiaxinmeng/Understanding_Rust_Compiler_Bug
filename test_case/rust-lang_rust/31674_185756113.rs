
mod foo {
    mod bar {
        pub trait T {} // this is inaccessible outside foo
    }
    pub use bar::T; // unless it is used through this import
}
