 rust
#[deriving(Eq)]
struct Vector1 {
    x: int,
}

impl Add<Vector1, Vector1> for Vector1 {
    fn add(&self, rhs: &Vector1) -> Vector1 {
        Vector1 {
            x: self.x + rhs.x,
        }
    }
}

impl AddAssign<Vector1> for Vector1 {
    fn add_assign(&mut self, rhs: &Vector1) {
        self.x = self.x + rhs.x;
    }
}

fn main() {
    let v1 = Vector1 { x: 1 };
    let v2 = Vector1 { x: 4 };
    let _v3 = v1.add(&v2);
    let v3 = v1 + v2;
    if v3.x != 5 {
        println("+ :-(");
    }

    let mut v1 = Vector1 { x: 1 };
    let v2 = Vector1 { x: 4 };
    v1.add_assign(&v2);
    v1 += v2;
    if v1.x != 9 {
        println("+= :-(");
    }
}
