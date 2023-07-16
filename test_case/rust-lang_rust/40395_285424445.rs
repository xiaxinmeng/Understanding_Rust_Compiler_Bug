rust
pub type Float = f64;
pub type Point2f = Point2<Float>;
pub type Point3f = Point3<Float>;
type Point4f = Point4<Float>;

#[derive(Default)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

// do *not* derive Default                                                                                                    
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Default for Point3f {
    /// will *not* show up in docs                                                                                            
    fn default() -> Point3f {
        Point3f {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        }
    }
}

// do *not* derive Default                                                                                                    
pub struct Point4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl Default for Point4f {
    /// will show up in docs                                                                                                  
    fn default() -> Point4f {
        Point4f {
            x: 0.0,
            y: 1.0,
            z: 2.0,
            w: 1.0,
	}
    }
}

#[cfg(test)]
mod tests {
    use super::{Point2f, Point3f, Point4f};
    #[test]
    fn p2_works() {
        let p2: Point2f = Point2f::default();
        assert_eq!(p2.x, 0.0);
        assert_eq!(p2.y, 0.0);
    }
    #[test]
    fn p3_works() {
        let p3: Point3f = Point3f::default();
	assert_eq!(p3.x, 0.0);
        assert_eq!(p3.y, 1.0);
        assert_eq!(p3.z, 2.0);
    }
    #[test]
    fn p4_works() {
        let p4: Point4f = Point4f::default();
        assert_eq!(p4.x, 0.0);
        assert_eq!(p4.y, 1.0);
        assert_eq!(p4.z, 2.0);
        assert_eq!(p4.w, 1.0);
    }
}
