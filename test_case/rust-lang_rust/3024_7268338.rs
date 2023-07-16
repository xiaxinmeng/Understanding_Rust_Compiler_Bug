
class dummy { let x: int; new(x: int) { self.x = x; } drop { #error["Destructor %d", self.x]; } }

fn welp<T>(+a: ~T) -> T { 
    let ~x <- a;
    x   
}

fn main() {
    let o = ~dummy(5);
    welp(o);
}   
