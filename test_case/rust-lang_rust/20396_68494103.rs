 rust
trait Foo<T> {}                           
enum Bar<T> {}                            
struct Baz<'a> {                          
    inner: for<'b> Foo<Bar<&'b ()>> + 'a,
}                                         
fn main() {}                              
