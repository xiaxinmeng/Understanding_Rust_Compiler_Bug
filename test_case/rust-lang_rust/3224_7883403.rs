
struct A { x: (); drop { error!("A"); } } 

fn main() {
    let bp = ~A { x: () };
    let ~_a <- bp; 
}
