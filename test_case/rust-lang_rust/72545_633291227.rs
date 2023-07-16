rust
pub(crate) struct Time;

use proc_macro_hack::proc_macro_hack;

macro_rules! impl_macros {
    ($($name:ident : $type:ty),* $(,)?) => {
        $(
            #[proc_macro_hack]
            #[allow(clippy::unimplemented)]
            pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                panic!()
            }
        )*
    };
}

impl_macros! {
    time: Time,
}
