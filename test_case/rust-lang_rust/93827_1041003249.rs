rust
struct MyMutRef<'a, T>(&'a mut T);

const fn accept_my_mut_ref(x: MyMutRef<'_, usize>) {}
