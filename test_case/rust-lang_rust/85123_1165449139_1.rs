
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
   --> C:\Users\Alexandre\devel\kyute\kyute\src\widget\grid\mod.rs:271:27
    |
271 |         let first = input.next()?;
    |                           ^^^^
    |
note: first, the lifetime cannot outlive the lifetime `'b` as defined here...
   --> C:\Users\Alexandre\devel\kyute\kyute\src\widget\grid\mod.rs:270:29
    |
270 |     pub(crate) fn parse_css<'b>(input: &mut Parser<'a, 'b>) -> Result<Line<'a>, ParseError<'a, ()>> {
    |                             ^^
note: ...so that the type `cssparser::Parser<'a, 'b>` is not borrowed for too long
   --> C:\Users\Alexandre\devel\kyute\kyute\src\widget\grid\mod.rs:271:21
    |
271 |         let first = input.next()?;
    |                     ^^^^^
note: but, the lifetime must be valid for the lifetime `'a` as defined here...
   --> C:\Users\Alexandre\devel\kyute\kyute\src\widget\grid\mod.rs:268:6
    |
268 | impl<'a> Line<'a> {
    |      ^^
note: ...so that the types are compatible
   --> C:\Users\Alexandre\devel\kyute\kyute\src\widget\grid\mod.rs:273:9
    |
273 | /         match (first, second) {
274 | |             // auto
275 | |             (Token::Ident(id), Err(_)) if &**id == "auto" => Ok(Line::Auto),
276 | |             // span N
...   |
308 | |             _ => Err(input.new_custom_error(())),
309 | |         }
    | |_________^
    = note: expected `std::result::Result<grid::Line<'a>, ParseError<'a, _>>`
               found `std::result::Result<grid::Line<'_>, ParseError<'_, _>>`
