rust
#![feature(return_position_impl_trait_in_trait)]
#![feature(adt_const_params)]

pub trait Bar<const BAR:&'static str>{}

impl Bar<"asdf"> for () {}

pub trait Foo<const FOO:&'static str>{
    fn foo()->impl Bar<"asdf">{
        ()
    }
}
