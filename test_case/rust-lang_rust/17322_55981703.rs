
trait Test {
    fn test(&mut self) -> int;
}

impl Test for Box<Test+'static> {
    fn test(&mut self) -> int {
        self.test()
    }
}

impl Test for int {
    fn test(&mut self) -> int {
        *self
    }
}

fn main() {
    let mut b: Box<Test+'static> = box 3i;
    let mut c: &mut Box<Test+'static> = &mut b;
    let mut d: &mut Test = c;
}

