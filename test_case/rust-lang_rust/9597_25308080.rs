
trait Test {
    fn method(&mut self);
}

struct RealTest {
    priv value: @mut int
}

impl RealTest{
    fn new(init: @mut int) -> RealTest {
        RealTest { value: init }
    }
}

impl Test for RealTest {
    fn method(&mut self) {
        *self.value += 1;
        println(fmt!("%?", self.value));
    }
}

fn run_test(test: &mut Test) {
    test.method();
    test.method();
}

fn main() {
    let cell = @mut 0;
    let test = &mut RealTest::new(cell) as &mut Test;

    run_test(test);
    println(fmt!("%?", cell));
}
