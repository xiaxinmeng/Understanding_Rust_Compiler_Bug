rs
#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]

trait Iter: Copy + Sized {
    type Item: Copy;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    #[default_method_body_is_const]
    fn count(mut self) -> usize {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }
}

#[derive(Copy, Clone)]
struct ConstIter {
    next: Option<u8>,
}

impl const Iter for ConstIter {
    type Item = u8;
    
    fn next(&mut self) -> Option<u8> {
        match self.next {
            Some(val) => {
                self.next = val.checked_add(1);
                Some(val)
            }
            None => None,
        }
    }
}

const _: () = {
    let c = ConstIter { next: Some(0) }.count();
    if c != 256 {
        [()][4];
    }
};
