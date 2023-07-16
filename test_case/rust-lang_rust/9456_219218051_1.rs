 rust
// prelude omitted
fn main() {
    ::std::io::_print(::std::fmt::Arguments::new_v1_formatted({
            static __STATIC_FMTSTR:
                   &'static [&'static str]
                   =
                &["",
                  " is ",
                  "; ",
                  " is ",
                  "\n"];
            __STATIC_FMTSTR
        },
        &match (&"\u{6d4b}\u{8bd5}\u{5b57}\u{7b26}\u{4e32}",
                &"foo\u{2014}bar")
             {
             (__arg0,
              __arga) =>
             [::std::fmt::ArgumentV1::new(__arg0,
                                          ::std::fmt::Debug::fmt),
              ::std::fmt::ArgumentV1::new(__arg0,
                                          ::std::fmt::Display::fmt),
              ::std::fmt::ArgumentV1::new(__arga,
                                          ::std::fmt::Debug::fmt),
              ::std::fmt::ArgumentV1::new(__arga,
                                          ::std::fmt::Display::fmt)],
         },
        {
            static __STATIC_FMTARGS:
                   &'static [::std::fmt::rt::v1::Argument]
                   =
                &[::std::fmt::rt::v1::Argument{position:
                                                   ::std::fmt::rt::v1::Position::At(2usize),
                                               format: (omitted),},
                  ::std::fmt::rt::v1::Argument{position:
                                                   ::std::fmt::rt::v1::Position::At(3usize),
                                               format: (omitted),},
                  ::std::fmt::rt::v1::Argument{position:
                                                   ::std::fmt::rt::v1::Position::At(0usize),
                                               format: (omitted),},
                  ::std::fmt::rt::v1::Argument{position:
                                                   ::std::fmt::rt::v1::Position::At(1usize),
                                               format: (omitted),}];
            __STATIC_FMTARGS
        }));
}
