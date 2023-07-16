
error[E0277]: the trait bound `std::borrow::Cow<'_, str>: std::convert::From<core::str::EscapeDefault<'_>>` is not satisfiedprobe, termcolor, rustc-serialize, rustc-ap-syntax(build.rs), same-file, num-derive(build.rs), rayon-core(build.rs), hex, ansi...
   --> /home/xanewok/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-graphviz-366.0.0/lib.rs:542:44
    |
542 |                     (&*s).escape_default().into()
    |                                            ^^^^ the trait `std::convert::From<core::str::EscapeDefault<'_>>` is not implemented for `std::borrow::Cow<'_, str>`
    |
    = help: the following implementations were found:
              <std::borrow::Cow<'a, [T]> as std::convert::From<&'a [T]>>
              <std::borrow::Cow<'a, [T]> as std::convert::From<&'a std::vec::Vec<T>>>
              <std::borrow::Cow<'a, [T]> as std::convert::From<std::vec::Vec<T>>>
              <std::borrow::Cow<'a, std::ffi::CStr> as std::convert::From<&'a std::ffi::CStr>>
            and 11 others
    = note: required because of the requirements on the impl of `std::convert::Into<std::borrow::Cow<'_, str>>` for `core::str::EscapeDefault<'_>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
