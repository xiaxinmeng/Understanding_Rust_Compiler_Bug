 rust
enum BL<'a> {
    Root,
    Link(&'a mut BLLike, &'a mut u32)
}

trait BLLike {
    fn val(&mut self) -> BL;
}

fn cast<'a : 'b, 'b>(foo: BL<'a>) -> BL<'b> {foo}
