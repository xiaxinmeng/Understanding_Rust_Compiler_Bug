
fn main() {
    let mut x = ~1;

    class r/& {
        let x: &~int;
        new(x: &~int) {
            self.x = x;
        }
        drop {
            log(error, **self.x);
        }
    }

    let r = r(&x);

    for int::range(0, 10) {|i|
        let z <- unsafe { let x <- *ptr::addr_of(x); x };
        fail;
        //x = ~(*z * i);
    }
}
