rs
struct Box<T> {
    ptr: *const T,
    // … (e.g., should we `PhantomData<T>` or not)
}

impl<T> Drop for Box<T> { /* or:
unsafe impl<#[may_dangle] T> Drop for Box<T> { */
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place::<T>(self.ptr); } // no-op if `T` has no drop glue (so `T` could dangle).
        unsafe { alloc::dealloc(self.ptr, …); } // does not use values of type `T`.
    }
}

/* e.g.
impl<'dangling_in_drop> for Box<&'dangling_in_drop str> {
// or
unsafe impl<#[may_dangle] 'dangling_in_drop> for Box<&'dangling_in_drop str> {

// as well as:
impl<'dangling_in_drop> for Box<DisplayOnDrop<&'dangling_in_drop str>> {
// or
unsafe impl<#[may_dangle] 'dangling_in_drop> for Box<DisplayOnDrop<&'dangling_in_drop str>> {
*/

// where
struct DisplayOnDrop<T : Display>(T);
impl<T : Display> Drop for DisplayOnDrop<T> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let (b1, b2);
    {
        let dropped_before_the_boxes = String::from("…");
        let thus_dangling_in_drop = &*dropped_before_the_boxes;

        b1 = Box::new(thus_dangling_in_drop);
        b2 = Box::new(DisplayOnDrop(thus_dangling_in_drop));
     } // `drop(dropped_before_the_boxes)`
     /* `thus_dangling_in_drop` is dangling here! */
} // `drop((b1, b2))`
