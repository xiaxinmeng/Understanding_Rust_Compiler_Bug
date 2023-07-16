
#[derive(Clone, Copy, Debug)]
struct S(u8);

impl S {
    fn f(mut self) {
        println!("arg {:?}", self);
        self.0 += 1;
        println!("arg {:?}", self);
        (move || {
            println!("capture level 1 {:?}", self);
            self.0 += 1;
            println!("capture level 1 {:?}", self);
            (move || {
                println!("capture level 2 {:?}", self);
                self.0 += 1;
                println!("capture level 2 {:?}", self);
                (move || { // (Need to go deeper)
                    println!("capture level 3 {:?}", self);
                    self.0 += 1;
                    // For a minute there, I lost my `self`, I lost my `seeelf`
                    println!("capture level 3 {:?}", self);
                })();
                println!("capture level 2 {:?}", self);
            })();
            println!("capture level 1 {:?}", self);
        })();
        println!("arg {:?}", self);
    }
}

fn main() {
    S(0).f();
}

--------
Prints:

arg S(0)
arg S(1)
capture level 1 S(1)
capture level 1 S(2)
capture level 2 S(2)
capture level 2 S(3)
capture level 3 S(3)
capture level 3 S(4)
capture level 2 S(3)
capture level 1 S(2)
arg S(1)
