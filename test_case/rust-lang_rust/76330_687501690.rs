
error: expected identifier, found keyword `pub`
   --> library/core/src/option.rs:391:11
    |
391 |     const pub fn unwrap_or(self, default: T) -> T {
    |           ^^^ expected identifier, found keyword

error: expected one of `:`, `;`, or `=`, found keyword `fn`
   --> library/core/src/option.rs:391:15
    |
160 | impl<T> Option<T> {
    |                   - while parsing this item list starting here
...
391 |     const pub fn unwrap_or(self, default: T) -> T {
    |               ^^ expected one of `:`, `;`, or `=`
...
940 | }
    | - the item list ends here

error: expected identifier, found keyword `pub`
   --> library/core/src/result.rs:801:11
    |
801 |     const pub fn unwrap_or(self, default: T) -> T {
    |           ^^^ expected identifier, found keyword

error: expected one of `:`, `;`, or `=`, found keyword `fn`
   --> library/core/src/result.rs:801:15
    |
257 | impl<T, E> Result<T, E> {
    |                         - while parsing this item list starting here
...
801 |     const pub fn unwrap_or(self, default: T) -> T {
    |               ^^ expected one of `:`, `;`, or `=`
...
829 | }
    | - the item list ends here

