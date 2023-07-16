rust
pub mod foo {
    pub mod bar {
        #[restrict(impl(super))]
        pub(crate) trait Foo {}
    }

    // can impl Foo here
}
// but not here

pub struct Time {
    #[restrict(mut(crate))]
    pub hour: u8,
    #[restrict(mut(crate))]
    pub minute: u8,
    #[restrict(mut(crate))]
    pub second: u8,
    #[restrict(mut(crate))]
    pub nanosecond: u32,
}
