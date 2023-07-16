
trait A     { fn a(&self); }
trait B : A { fn b(&self); }

impl<T:B> A for T { fn a(&self) { self.b(); } }

impl B for int {
        fn b(&self) {
                    std::io::println(&*format!("b({:?})", *self));
                        }
}

fn main() {
}
