rust
fn explode<V, F, V2>(mut node_map: F) -> Option<()>
where
    F: FnMut(V) -> Option<V2>,
{
    let key = conjure();
    node_map(key)?;
    None
}

pub struct Layout<const N: usize>;
impl<const N: usize> Layout<N> {
    pub fn new() {
        for i in 0..N {
            let node_map = |key: [(); N]| -> Option<[f32; N]> {
                key[i];
                None
            };
            explode(node_map);
        }
    }
}

fn conjure<T>() -> T {
    unimplemented!()
}

pub fn main() {
    Layout::<0>::new();
}
