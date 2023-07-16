
trait T2: T1 {
    fn say_bye(&self);
}
trait T1 {
    fn say_hello(&self);
}
impl T1 for i32 {
    fn say_hello(&self) {
        println!("Hello {}!", self);
    }
}
impl T2 for i32 {
    fn say_bye(&self) {
        println!("Bye {}?", self);
    }
}
fn main() {
    let n = 16;
    n.say_hello();
    n.say_bye();
}
