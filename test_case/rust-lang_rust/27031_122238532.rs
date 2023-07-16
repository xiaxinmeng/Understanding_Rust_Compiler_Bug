 Rust
enum BL<'a> {
    Root,
    Link(&'a mut BLLike, &'a mut u32)
}

trait BLLike {
    fn val(&mut self) -> BL;
}

impl<'a> BLLike for BL<'a> {
    fn val(&mut self) -> BL {
        match self {
            &mut BL::Root => BL::Root,
            &mut BL::Link(ref mut cdr, ref mut car) => BL::Link(*cdr, car)
        }
    }

}

fn traverse<'a>(mut u: u32, mut l: BL) {
    if u > 0 {
        traverse(u-1, BL::Link(&mut l, &mut u))
    } else {
        while let BL::Link(cdr, car) = l {
            println!("u={:?}", car);
            l = cdr.val();
        }
    }
}

fn main() {
    traverse(14, BL::Root);
}
