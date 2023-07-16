
struct Vec2 {
    x: float,
    y: float
}

// methods we want to export as methods as well as operators
impl Vec2 {
#[inline(always)]
    pure fn vmul(other: float) -> Vec2 {
        Vec2 { x: self.x * other, y: self.y * other }
    }
}

// Right-hand-side operator visitor pattern
trait RhsOfVec2Mul<Result> { pure fn mul_vec2_by(lhs: &Vec2) -> Result; }

// Vec2's implementation of Mul "from the other side" using the above trait
impl<Res, Rhs: RhsOfVec2Mul<Res>> Vec2: Mul<Rhs,Res> {
    pure fn mul(rhs: &Rhs) -> Res { rhs.mul_vec2_by(&self) }
}

// Implementation of 'float as right-hand-side of Vec2::Mul'
impl float: RhsOfVec2Mul<Vec2> {
    pure fn mul_vec2_by(lhs: &Vec2) -> Vec2 { lhs.vmul(self) }
}

// Usage with failing inference
fn main() {
    let a = Vec2 { x: 3f, y: 4f };

    // the following compiles and works properly
    let v1: Vec2 = a * 3f;
    io::println(fmt!("%f %f", v1.x, v1.y));

    // the following compiles but v2 will not be Vec2 yet and
    // using it later will cause an error that the type of v2
    // must be known
    let v2 = a * 3f;
    io::println(fmt!("%f %f", v2.x, v2.y)); // error regarding v2's type
}
