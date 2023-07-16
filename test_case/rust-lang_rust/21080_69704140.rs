 rust
extern crate test;                                                      

struct Future<T>;                                                       

fn join<J: ToJoin<T>, T: Send>(_: J) -> Future<T> { }                   

trait ToJoin<T> { }                                                     
impl <A1, T1, A2, T2, A3, T3> ToJoin<(T1, T2, T3)> for (A1, A2, A3) { } 

fn main() {                                                             
    let _: Future<(i32, i32, i32)> = join((0, 0, 0));                   
}                                                                       
