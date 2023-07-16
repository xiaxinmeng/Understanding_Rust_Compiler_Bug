
fn foo<'a,'b>(x: &mut Vec<&'a u8>, y: &'b u8) where &'a (): Sized, &'b u32: Sized {
        x.push(y);
}
