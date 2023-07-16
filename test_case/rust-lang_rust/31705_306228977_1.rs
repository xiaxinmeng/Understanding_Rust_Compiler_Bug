rust
macro_rules! network_onefunc {
    ( $name: ident -> $type_: ty ) => {
        pub fn $name() -> $type_ {
            println!("test");
        }
    };
}

macro_rules! network_impl {
    ( $( $name: ident -> $type_: ty ),+ ) => {
        $(
            network_onefunc!( $name -> $type_ )
        )*
    };
}

network_impl!(hi -> ());
