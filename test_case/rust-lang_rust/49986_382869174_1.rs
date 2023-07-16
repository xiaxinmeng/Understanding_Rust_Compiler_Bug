rust
pub fn main() {
    ::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                &match (&1,) {
                                                     (__arg0,) =>
                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                  ::std::fmt::Display::fmt)],
                                                 }));
}
