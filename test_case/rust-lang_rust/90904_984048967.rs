rust
impl<const N: usize> Add<T> for Simd<T, N>
where
    T, N, Simd<T, N>, AndYourDog: ManySimdBounds, 
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        self.add(Simd::splat(rhs))
    }
}
