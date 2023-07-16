
class Failer { let x: @int; new(x: @int) { self.x = x; } drop { fail; } }
fn main() {
     let f = Failer(@10);
     fail;
} 
