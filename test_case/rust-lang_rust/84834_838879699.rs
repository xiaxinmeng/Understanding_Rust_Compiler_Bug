rust
pub mod sub1 {
    pub mod sub1sub1 {
        pub struct Sub1Sub1Struct;
    }

    pub mod sub1sub2 {
        pub struct Sub1Sub2Struct;
    }

    pub struct Sub1Struct;
}

pub mod sub2 {
    pub mod sub2sub1 {
        pub struct Sub2Sub1Struct;
    }

    pub struct Sub2Struct;
}

pub struct RootStruct;
