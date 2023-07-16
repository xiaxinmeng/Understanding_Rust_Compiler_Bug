
struct A; struct B;

fn main() {
    let id: &Fn(B)-> B = &|x| { x }; 
    let di: &Fn(A)-> &Fn(B) -> B = &|_| { id };
}
