 rust
trait Bar{} 
impl Bar for usize {}
impl Bar for i32 {}

fn main() {
    let (a, b) = (&5u as &Bar, &9u as &Bar); 
    let (c, d): (&Bar, &Bar) = (a, b);


    let (a, b) = (Box::new(5u) as Box<Bar>, Box::new(9u) as Box<Bar>); 
    let (c, d): (&Bar, &Bar) = (&*a, &*b);


    let (c, d): (&Bar, &Bar) = (&5, &9);
}
