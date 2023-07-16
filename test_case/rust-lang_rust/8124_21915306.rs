
fn borrow_mut_to_immutable(stuff: &mut Stuff)
{ // lifetime 'a
    let a: &'a mut Foo = borrow_part(&'a mut *stuff);
    let c = copy_out_of_immutable(&'b stuff.y); // call has lifetime 'b
}
