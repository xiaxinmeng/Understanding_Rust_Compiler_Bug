rust
macro_rules! foo(( $ t : ident ) => {
                 macro_rules ! $ t {
                 (  ) => { foo ! ( moo ) ; println ! ( stringify ! ( $ t ) ) ;
                 } } });

fn main() {
    macro_rules! bar((  ) => {
                     foo ! ( moo ) ; println ! ( stringify ! ( bar ) ) ; });
    macro_rules! moo((  ) => {
                     foo ! ( moo ) ; println ! ( stringify ! ( moo ) ) ; });
    ::io::_print(::std::fmt::Arguments::new_v1(&["bar\n"],
                                               &match () { () => [], }));
    macro_rules! moo((  ) => {
                     foo ! ( moo ) ; println ! ( stringify ! ( moo ) ) ; });
    ::io::_print(::std::fmt::Arguments::new_v1(&["moo\n"],
                                               &match () { () => [], }));
}
