 rust
trait Borrow<T: ?Sized> {}                                       
impl<T: ?Sized> Borrow<T> for T {}                               
impl<'a, T: ?Sized> Borrow<T> for &'a T {}                       

struct Map<K: 'static>(K);                                       
static MAP: Map<&'static str> = Map("bar");                      
impl<K> Map<K> {                                                 
    fn get<T>(&self, _key: &T) -> Option<&K> where K: Borrow<T> {
        loop {}                                                  
    }                                                            
}                                                                

fn _foo<T: FnMut(&Captures)>(_: T) {}                            

struct Captures<'t>(&'t str);                                    
impl<'t> Captures<'t> {                                          
    fn at(&self) -> &'t str { loop {} }                          
}                                                                

pub fn foo() {                                                   
    _foo(|caps| {                                                
        MAP.get(&caps.at());                                     
    })                                                           
}                                                                
fn main() {}
