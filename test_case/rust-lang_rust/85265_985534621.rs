Rust
/// Ineffective workaround.
pub fn use_ref_add(a: Vec4, b: Vec4) -> Vec4 {
    let mut c = Vec4::default();
    Vec4::ref_add(&a, &b, &mut c);
    c
}

#[derive(Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Ineffective workaround.
    pub fn ref_add(lhs: &Self, rhs: &Self, out: &mut Self) {
        out.x = lhs.x + rhs.x;
        out.y = lhs.y + rhs.y;
        out.z = lhs.z + rhs.z;
        out.w = lhs.w + rhs.w;
    }
}

impl std::ops::Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w }
    }
}


impl std::ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
