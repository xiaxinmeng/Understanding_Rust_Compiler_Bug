
[00:03:00] error[E0277]: the trait bound `std::string::String: std::convert::From<&&str>` is not satisfied

[00:03:00]     --> /checkout/src/libsyntax/parse/parser.rs:2711:21

[00:03:00]      |

[00:03:00] 2711 |                 err.span_label(span_of_tilde, &"did you mean `!`?");

[00:03:00]      |                     ^^^^^^^^^^ the trait `std::convert::From<&&str>` is not implemented for `std::string::String`

[00:03:00]      |

[00:03:00]      = help: the following implementations were found:

[00:03:00]                <std::string::String as std::convert::From<&'a str>>

[00:03:00]                <std::string::String as std::convert::From<std::boxed::Box<str>>>

[00:03:00]                <std::string::String as std::convert::From<std::borrow::Cow<'a, str>>>

[00:03:00]      = note: required because of the requirements on the impl of `std::convert::Into<std::string::String>` for `&&str`

[00:03:00] 
