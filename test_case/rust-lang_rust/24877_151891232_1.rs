
<std macros>:2:8: 2:54 error: no method named `write_fmt` found for type `&mut Box<Foo>` in the current scope
<std macros>:2 $ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:2:1: 2:46 note: in this expansion of write! (defined in <std macros>)
<anon>:20:3: 20:38 note: in this expansion of writeln! (defined in <std macros>)
<std macros>:2:8: 2:54 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
<std macros>:2:8: 2:54 help: candidate #1: use `std::io::Write`
error: aborting due to previous error
