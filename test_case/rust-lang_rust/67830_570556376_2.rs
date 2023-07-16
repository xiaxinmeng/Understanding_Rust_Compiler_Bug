
fn test(_: impl for<'a> MyFn<&'a A, Output=impl Iterator + 'a>) {
                                                            ^^ undeclared lifetime
