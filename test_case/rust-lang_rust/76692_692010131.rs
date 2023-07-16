rust
mod inner {
    pub use u64;
    //^~ ERROR ambiguous
    
    pub mod u64 {}
    //^~ ERROR defined multiple times
}
