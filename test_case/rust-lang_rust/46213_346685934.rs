rust
// Use the Opacity.0.tag niche
pub enum FilterOp {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(PropertyBinding<f32>, f32),
    Saturate(f32),
    Sepia(f32),
}

pub enum PropertyBinding<T> {
    Value(T),
    Binding(PropertyBindingKey<T>),
}

pub struct PropertyBindingKey<T> {
    pub id: PropertyBindingId,
    _phantom: PhantomData<T>,
}

pub struct PropertyBindingId {
    namespace: IdNamespace,
    uid: u32,
}

pub struct IdNamespace(pub u32);
