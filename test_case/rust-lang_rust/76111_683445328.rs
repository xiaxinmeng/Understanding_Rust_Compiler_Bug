rust
    pub fn do_fn<F>(otherwise: F,w:F) where F:FnOnce()  {
        otherwise();
    }
    fn main() {
        let arg = 0;
        do_fn(||{
            let result=arg+1;
        });
    }
