
trait SomeInteger         { fn bleep(&self);   }
impl  SomeInteger for i8  { fn bleep(&self) {} }
impl  SomeInteger for i32 { fn bleep(&self) {} }

fn bleep_it(a: Box<dyn SomeInteger>) {
    a.bleep();
}

fn main() {
    let number_of_yaks = 3;

    bleep_it(Box::new(number_of_yaks));
    print!("we have run out of {yaks} to shave");
}
