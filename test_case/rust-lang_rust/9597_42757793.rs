
trait Test<'a> {
    fn method(&'a mut self);
}

struct RealTest<'a> {
    value: &'a mut int
}

impl<'a> RealTest<'a> {
    fn new<'a>(init: &'a mut int) -> RealTest<'a> {
        RealTest { value: init }
    }
}

impl<'a> Test<'a> for RealTest<'a> {
    fn method(&'a mut self) {
        *self.value += 1;
        println!("{}", *self.value);
    }
}

fn run_test<'a>(test: &'a mut Test<'a>) {
    test.method();
    //test.method();
}

fn main() {
    let mut cell = 0;
    {
        let mut test = RealTest::new(&mut cell);
        run_test(&mut test);
    }
    println!("{}", cell);
}
