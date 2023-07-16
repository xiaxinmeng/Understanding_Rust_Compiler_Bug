
#[derive(Clone, Copy)]
struct Derived<T>(T);

#[derive(Clone)]
struct BoundOnStruct<T: Copy>(T);
impl<T: Copy> Copy for BoundOnStruct<T> {}

#[derive(Clone)]
struct BoundOnImpl<T>(T);
impl<T: Copy> Copy for BoundOnImpl<T> {}
