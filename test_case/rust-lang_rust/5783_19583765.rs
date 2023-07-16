
fn main() {
    let a: &fn(int) -> @fn(int) -> int = |x:int| |y:int| -> int x + y;
    let b = a(2);
    println(a(2)(3).to_str());  // 6
    println(b(3).to_str());  // completely random number
}
