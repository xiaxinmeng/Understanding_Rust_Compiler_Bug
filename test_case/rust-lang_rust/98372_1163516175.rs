
#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

#[lang = "owned_box"]
pub struct Box<T: ?Sized, A: Allocator = Global>(PhantomData<T>, A);
