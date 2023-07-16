plain
   Compiling basic-toml v0.1.2
   Compiling askama_derive v0.12.0
    Checking askama v0.12.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: can't compare `str` with `std::boxed::Box<str>`
    |
    |
410 |                         self.links.iter().find(|l| l.href == link.href && **text == l.original_text)
    |                                                                                  ^^ no implementation for `str == std::boxed::Box<str>`
    |
    = help: the trait `std::cmp::PartialEq<std::boxed::Box<str>>` is not implemented for `str`
    = help: the following other types implement trait `std::cmp::PartialEq<Rhs>`:
              <&'a str as std::cmp::PartialEq<OsString>>
              <&'a str as std::cmp::PartialEq<rustc_target::json::Json>>
              <&'a str as std::cmp::PartialEq<serde_json::Value>>
              <&'a str as std::cmp::PartialEq<std::string::String>>
              <&'a str as std::cmp::PartialEq<unicase::Ascii<S1>>>
              <&'b str as std::cmp::PartialEq<Cow<'a, str>>>
              <&str as std::cmp::PartialEq<windows::core::strings::bstr::BSTR>>
              <&str as std::cmp::PartialEq<windows::core::strings::hstring::HSTRING>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:25
