
fn main() {
    // using `.borrow()`:
    let _: &i32 = 0i32.borrow();
    let _: &i32 = Cow::<i32>::Owned(0i32).borrow();
    let _: &i32 = Cow::Borrowed(&0i32).borrow();
    /* … */
    // using deref-coercion:
    let _: &i32 = &0i32;
    let _: &i32 = &Cow::<i32>::Owned(0i32);
    let _: &i32 = &Cow::Borrowed(&0i32);
    /* … */
    // using `.as_ref()`:
    //let _: &i32 = 0i32.as_ref(); // Won't work
    let _: &i32 = Cow::<i32>::Owned(0i32).as_ref();
    let _: &i32 = Cow::Borrowed(&0i32).as_ref();
    /* … */
}
