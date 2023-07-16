
error[E0277]: the trait bound `<I as combine::StreamOnce>::Error: combine::ParseError<char, <I as combine::StreamOnce>::Range, <I as combine::StreamOnce>::Position>` is not satisfied
Sep 21 16:12:45.476 INFO kablam!    --> src/lib.rs:163:1
Sep 21 16:12:45.476 INFO kablam!     |
Sep 21 16:12:45.476 INFO kablam! 163 | / pub struct Reserved<'a: 'b, 'b, I>
Sep 21 16:12:45.476 INFO kablam! 164 | | where
Sep 21 16:12:45.476 INFO kablam! 165 | |     I: Stream<Item = char> + 'b,
Sep 21 16:12:45.476 INFO kablam! 166 | |     I::Error: ParseError<I::Item, I::Range, I::Position>,
Sep 21 16:12:45.477 INFO kablam! 167 | | {
Sep 21 16:12:45.477 INFO kablam! 168 | |     parser: Lex<'a, 'b, Try<Skip<Str<I>, NotFollowedBy<LanguageParser<'a, 'b, I, char>>>>>,
Sep 21 16:12:45.477 INFO kablam! 169 | | }
Sep 21 16:12:45.477 INFO kablam!     | |_^ the trait `combine::ParseError<char, <I as combine::StreamOnce>::Range, <I as combine::StreamOnce>::Position>` is not implemented for `<I as combine::StreamOnce>::Error`
