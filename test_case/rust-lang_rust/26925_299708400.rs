rust
#[derive(Copy, Clone)]
pub struct Ptr<Mark>(*mut u8, PhantomData<Mark>);

pub type Ref<'a, T: 'a> = Ptr<&'a T>;
pub type RefMut<'a, T: 'a> = Ptr<&'a mut T>;
