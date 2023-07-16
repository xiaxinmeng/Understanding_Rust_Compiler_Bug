rust
> trait Lerp {
>     fn lerp32(t: f32, v0: Self, v1: Self) -> Self { Self::lerp64(t, v0, v1) };
>     fn lerp64(t: f64, v0: Self, v1: Self) -> Self;
> }
> 
> impl f32 {
>     fn lerp<L: Lerp>(self, v0: L, v1: L) -> L { L::lerp32(self, v0, v1) }
> }
> 
> impl f64 {
>     fn lerp<L: Lerp>(self, v0: L, v1: L) -> L { L::lerp64(self, v0, v1) }
> }
> 