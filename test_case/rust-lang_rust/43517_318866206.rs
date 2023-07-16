
pub mod slow_iters {

    pub fn iter_any() -> bool {
        [1].iter().any(|&x| x == 1)
    }

    pub fn for_loop() -> bool {
        for &x in [1].into_iter() {
            if x == 1 { return true; }
        }
        return false;
    }

}
