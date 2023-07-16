rust
fn bar() -> impl Send {
    async move
        {
            foo(

                ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["",
                                                                      ":"],
                                                                    &match (&1,
                                                                            &2)
                                                                         {
                                                                         (arg0,
                                                                          arg1)
                                                                         =>
                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                       ::core::fmt::Display::fmt),
                                                                          ::core::fmt::ArgumentV1::new(arg1,
                                                                                                       ::core::fmt::Display::fmt)],
                                                                     }))).await;
        }
}
