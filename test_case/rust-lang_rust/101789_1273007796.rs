plain
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0277]: the trait bound `std::string::String: From<rustc_span::Span>` is not satisfied
    --> compiler/rustc_parse/src/parser/item.rs:1801:30
     |
1801 |                     err.note(span, "the `let` keyword is not allowed in `struct` fields");
     |                         ---- ^^^^ the trait `From<rustc_span::Span>` is not implemented for `std::string::String`
     |                         required by a bound introduced by this call
     |
     = help: the following other types implement trait `From<T>`:
     = help: the following other types implement trait `From<T>`:
               <&'l str as From<&'l unic_langid_impl::subtags::region::Region>>
               <&'l str as From<&'l unic_langid_impl::subtags::script::Script>>
               <std::string::String as From<&mut str>>
               <std::string::String as From<&std::string::String>>
               <std::string::String as From<&str>>
               <std::string::String as From<Cow<'a, str>>>
               <std::string::String as From<char>>
               <std::string::String as From<std::boxed::Box<str>>>
     = note: required for `rustc_span::Span` to implement `Into<std::string::String>`
     = note: required for `SubdiagnosticMessage` to implement `From<rustc_span::Span>`
     = note: 1 redundant requirement hidden
     = note: required for `rustc_span::Span` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `DiagnosticBuilder::<'a, G>::note`
     |
     |
488  |     forward!(pub fn note(&mut self, msg: impl Into<SubdiagnosticMessage>) -> &mut Self);
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::note`
error[E0061]: this function takes 1 argument but 2 arguments were supplied
    --> compiler/rustc_parse/src/parser/item.rs:1801:25
     |
     |
1801 |                     err.note(span, "the `let` keyword is not allowed in `struct` fields");
     |                         ^^^^       ----------------------------------------------------- argument of type `&'static str` unexpected
note: associated function defined here
    --> /checkout/compiler/rustc_errors/src/diagnostic_builder.rs:488:21
     |
     |
488  |     forward!(pub fn note(&mut self, msg: impl Into<SubdiagnosticMessage>) -> &mut Self);
help: remove the extra argument
     |
     |
1801 |                     err.note(span);

   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0277]: the trait bound `std::string::String: From<rustc_span::Span>` is not satisfied
    --> compiler/rustc_parse/src/parser/item.rs:1802:30
    --> compiler/rustc_parse/src/parser/item.rs:1802:30
     |
1802 |                     err.note(span, "see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information");
     |                         ---- ^^^^ the trait `From<rustc_span::Span>` is not implemented for `std::string::String`
     |                         required by a bound introduced by this call
     |
     = help: the following other types implement trait `From<T>`:
     = help: the following other types implement trait `From<T>`:
               <&'l str as From<&'l unic_langid_impl::subtags::region::Region>>
               <&'l str as From<&'l unic_langid_impl::subtags::script::Script>>
               <std::string::String as From<&mut str>>
               <std::string::String as From<&std::string::String>>
               <std::string::String as From<&str>>
               <std::string::String as From<Cow<'a, str>>>
               <std::string::String as From<char>>
               <std::string::String as From<std::boxed::Box<str>>>
     = note: required for `rustc_span::Span` to implement `Into<std::string::String>`
     = note: required for `SubdiagnosticMessage` to implement `From<rustc_span::Span>`
     = note: 1 redundant requirement hidden
     = note: required for `rustc_span::Span` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `DiagnosticBuilder::<'a, G>::note`
     |
     |
488  |     forward!(pub fn note(&mut self, msg: impl Into<SubdiagnosticMessage>) -> &mut Self);
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::note`
error[E0061]: this function takes 1 argument but 2 arguments were supplied
    --> compiler/rustc_parse/src/parser/item.rs:1802:25
     |
     |
1802 |                     err.note(span, "see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information");
     |                         ^^^^       ----------------------------------------------------------------------------------------- argument of type `&'static str` unexpected
note: associated function defined here
    --> /checkout/compiler/rustc_errors/src/diagnostic_builder.rs:488:21
     |
     |
488  |     forward!(pub fn note(&mut self, msg: impl Into<SubdiagnosticMessage>) -> &mut Self);
help: remove the extra argument
     |
     |
1802 |                     err.note(span);

Some errors have detailed explanations: E0061, E0277.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_parse` due to 4 previous errors
