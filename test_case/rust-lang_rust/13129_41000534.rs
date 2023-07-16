
trait GetRef<'a> {
  fn get(&self) -> &'a int;
  fn sum(&self) -> int;
}

struct Foo<'b> {
    data: &'b ~[int]
}

static SOME_INT: int = 3;

impl<'b> GetRef<'static> for Foo<'b> {
    fn get(&self) -> &'static int { &SOME_INT }
    fn sum(&self) -> int {
        let mut sum = 0;
        for &i in self.data.iter() {
            sum += i;
        }
        sum
    }
}
