rust
#![feature(nll)]
#![allow(unused_variables, unused_mut)]

use std::fmt;

fn main() {
    let mut v = vec![1, 2, 3, 4];
    {
        let x = good(v.iter().cloned());
        let mut y = bad(v.iter().cloned());
        
        // even trying to exhaust `y` will not suffice unless we actually
        // *consume* it (and thus run its destructor):
        for (i, elem) in y.by_ref().enumerate() {
            println!("processing y[{}]: {:?}", i, elem);
        }
        println!("`y` is \"exhausted\" here, but not dead.");

        println!("this is where you wanted to push");
        // v.push(5);
        
        println!("end of block");
    }
}

fn good<I: Iterator<Item = usize>>(x: I) -> std::vec::IntoIter<usize> {
    x.collect::<Vec<usize>>().into_iter()
}


fn bad<I: Iterator<Item = usize>>(x: I) -> impl Iterator<Item = usize> {
    // x.collect::<Vec<usize>>().into_iter()
    return DeadHands { iter: x, limit: 2 };
    
    /// An iterator that yields at most `self.limit` items from given `iter`.
    struct DeadHands<I: Iterator> where I::Item: fmt::Debug {
        iter: I,
        limit: usize 
    }
    impl<I: Iterator> Drop for DeadHands<I> where I::Item: fmt::Debug {
        fn drop(&mut self) {
            for elem in self.iter.by_ref() {
                println!("accessing elem {:?} from *original* iterator", elem)
            }
        }
    }
    impl<I: Iterator> Iterator for DeadHands<I> where I::Item: fmt::Debug {
        type Item = I::Item;
        fn next(&mut self) -> Option<I::Item> {
            if self.limit > 0 {
                self.limit -= 1;
                self.iter.next()
            } else {
                None
            }
        }
    }

}
