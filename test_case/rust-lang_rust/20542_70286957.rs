 rust
trait Vec3d {
    fn cross<V>(&self, rhs: &V)  
        where V: Vec3d,
              Mul<<Self as Index<usize>>::Output>::Output: Float { } 
}
