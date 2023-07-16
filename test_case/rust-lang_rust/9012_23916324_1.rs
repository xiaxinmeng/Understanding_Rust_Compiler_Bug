 rust
priv use std::prelude::*;
priv extern mod std;

mod __std_macros {
    #[macro_escape];
    #[doc(hidden)];
    priv use std::prelude::*;
}
fn main() {
    {
        #[address_insignificant]
        pub static __static_fmtstr: [::std::fmt::rt::Piece<'static>, ..1u] =
            [::std::fmt::rt::Argument(::std::fmt::rt::Argument{position:
                                                                   ::std::fmt::rt::ArgumentNext,
                                                               format:
                                                                   ::std::fmt::rt::FormatSpec{fill:
                                                                                                  ' ',
                                                                                              align:
                                                                                                  ::std::fmt::parse::AlignUnknown,
                                                                                              flags:
                                                                                                  0u,
                                                                                              precision:
                                                                                                  ::std::fmt::parse::CountImplied,
                                                                                              width:
                                                                                                  ::std::fmt::parse::CountImplied,},
                                                               method:
                                                                   None,})];
        let __arg0 = &2;
        let args: &[::std::fmt::Argument] =
            &[::std::fmt::argument(::std::fmt::Default::fmt, __arg0)];
        let ret: ::std::fmt::Arguments =
            unsafe {
                ::std::cast::transmute((__static_fmtstr.as_slice(), args))
            };
        |test| { use_test(test) }(&ret)
    };
}

