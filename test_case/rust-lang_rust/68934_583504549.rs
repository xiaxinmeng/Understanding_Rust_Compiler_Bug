
struct UBranch<'a>(&'a mut UBranch<'a>, u32);

impl<'a> UBranch<'a> {
    fn deep_fetch(&mut self, v2: u32) -> &mut Self {
        match self {
            UBranch(ref mut a, ref v) if v == &v2 => a.deep_fetch(v2),
            x => x
        }
    }
}

