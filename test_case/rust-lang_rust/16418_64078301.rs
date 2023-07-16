 rust
struct Vec2<S>(S, S);

fn from_array<S>(arr: [S, ..2]) -> Vec2<S> {
    *unsafe { mem::transmute::<&[S, ..2], Box<Vec2<S>>>(&arr) }
}
