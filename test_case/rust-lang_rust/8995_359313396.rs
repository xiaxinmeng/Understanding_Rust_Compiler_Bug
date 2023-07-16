
#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    const ZERO: Self = Vector { x: 0.0, y: 0.0, z: 0.0 };
    
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x, y, z
        }
    }
}
