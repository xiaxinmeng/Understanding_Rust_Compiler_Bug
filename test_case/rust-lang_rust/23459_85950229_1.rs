
<std macros>:2:20: 2:66 error: type `&mut std::fs::File` does not implement any method in scope named `write_fmt`
<std macros>:2 ( & mut * $ dst ) . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:2:66: 2:66 help: methods from traits can only be called if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
<std macros>:2:66: 2:66 help: candidate #1: use `std::io::Write`
