rs
trait TrustedRandomAccess {
    fn size(&self) -> usize;
    unsafe fn get_unchecked(&mut self, idx: usize) -> Self::Item;
    fn start_back_iteration_effect(&mut self);
    fn next_on_empty_effect(&mut self);
    
    const MAY_HAVE_SIDE_EFFECT: bool;
}
