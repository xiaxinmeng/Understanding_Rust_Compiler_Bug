 rs
use std::io::stdio::println;

struct Person{
  first_name: ~str,
  last_name: ~str
}

impl<'self> Person {
  pub fn new(first: ~str, last: ~str) -> ~Person {
    ~Person{ first_name: first, last_name: last }
  }

  pub fn full_name(&self) -> ~str {
    self.first_name + " " + self.last_name
  }

  pub fn get_first_name(&'self self) -> &'self str {
    self.first_name
  }
}

fn main() {
  let person = Person::new(~"Tom", ~"Dale");
  println(person.get_first_name());
  println(person.full_name());
}
