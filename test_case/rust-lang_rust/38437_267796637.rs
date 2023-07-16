rust
struct Good(usize);
impl Drop for Good {
    #[inline(never)]
    fn drop(&mut self) {
        println!("dropping Good({})", self.0);
    }
}

struct Bad(usize);
impl Drop for Bad {
    #[inline(never)]
    fn drop(&mut self) {
        println!("dropping Bad({})", self.0);
    }
}

enum E { A(Bad), B(Good) }

fn main() {
    let mut go = true;
    
    loop {
        let next;
        match go {
            true => next = E::B(Good(123)),
            false => return,
        }

        match next {
            E::A(_) => return,
            E::B(_good) => go = false,
        }
    }
}
