rust
struct MyStruct{
  c_allocated_object: Pin<Box<MaybeUninit<c_object_type>>>
}

impl MyStruct {
  fn build() -> MyStruct {
    let mut lazy_allocation = MaybeUninit::uninit();

    let mut boxed = Box::pin(lazy_allocation);

    my_unsafe_c_allocator(boxed.as_mut_ptr());

    MyStruct {
      c_allocated_object: boxed
    }
  }

  fn do_something_with_pointer(&mut self){
    unsafe{
        // send ptr to another c function
        do_something_with_pointer(self.c_allocated_object.as_mut_ptr())
    }
  }
}
