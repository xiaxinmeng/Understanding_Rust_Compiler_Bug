rust
pub mod facade {
    pub use details1::ApiStruct;
    pub use details2::ApiTrait;
    pub use details3::api_fn;

    mod details1 {
        pub struct ApiStruct;
    }
    mod details2 {
        pub trait ApiTrait {}
    }
    mod details3 {
        pub fn api_fn() {}
    }
}
