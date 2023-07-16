
error[E0631]: type mismatch in function arguments
   --> web_scraper/src/endpoint_docs/bodies.rs:31:14
    |
31  |         .map(get_rest_call_parameters)
    |          --- ^^^^^^^^^^^^^^^^^^^^^^^^ expected due to this
    |          |
    |          required by a bound introduced by this call
...
43  | pub(crate) fn get_rest_call_parameters(body: ElementRef) -> Result<Vec<RestCallParameter>> {
    | ------------------------------------------------------------------------------------------ found signature defined here
    |
    = note: expected function signature `fn(&ElementRef<'_>) -> _`
               found function signature `for<'a> fn(ElementRef<'a>) -> _`
note: required by a bound in `map`
   --> /home/matt/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:803:12
    |
803 |         F: FnMut(Self::Item) -> B,
    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::map`
help: consider borrowing the argument
    |
43  | pub(crate) fn get_rest_call_parameters(body: &ElementRef) -> Result<Vec<RestCallParameter>> {
    |                                              +

error[E0599]: the method `collect` exists for struct `Map<Iter<'_, ElementRef<'_>>, fn(ElementRef<'a>) -> Result<Vec<RestCallParameter>, Report<Error>> {get_rest_call_parameters}>`, but its trait bounds were not satisfied
  --> web_scraper/src/endpoint_docs/bodies.rs:32:10
   |
29 |       let all_parameters = bodies
   |  __________________________-
30 | |         .iter()
31 | |         .map(get_rest_call_parameters)
32 | |         .collect::<Result<Vec<Vec<RestCallParameter>>>>()?;
   | |         -^^^^^^^ method cannot be called due to unsatisfied trait bounds
   | |_________|
   |
   |
  ::: /home/matt/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:61:1
   |
61 |   pub struct Map<I, F> {
   |   -------------------- doesn't satisfy `_: Iterator`
   |
   = note: the full type name has been written to '/home/matt/projects/oanda_generator/target/debug/deps/web_scraper-64fa5fcf61b24b32.long-type-12607946688287355720.txt'
   = note: the following trait bounds were not satisfied:
           `<for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters} as FnOnce<(&ElementRef<'_>,)>>::Output = _`
           which is required by `Map<std::slice::Iter<'_, ElementRef<'_>>, for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters}>: Iterator`
           `for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters}: FnMut<(&ElementRef<'_>,)>`
           which is required by `Map<std::slice::Iter<'_, ElementRef<'_>>, for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters}>: Iterator`
           `Map<std::slice::Iter<'_, ElementRef<'_>>, for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters}>: Iterator`
           which is required by `&mut Map<std::slice::Iter<'_, ElementRef<'_>>, for<'a> fn(ElementRef<'a>) -> std::result::Result<Vec<RestCallParameter>, error_stack::Report<error::Error>> {get_rest_call_parameters}>: Iterator`

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `web_scraper` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
