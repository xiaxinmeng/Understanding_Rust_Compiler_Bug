rust
struct X {}
impl SomeTrait for X { ... }

struct Y { x: X }
impl SomeTrait for Y { ... }

let y = Y { x: X {} }

let a = &y as *const SomeTrait;
let b = &y.x as *const SomeTrait;

println!("{:?} == {:?} : {}", a, b, a == b);
