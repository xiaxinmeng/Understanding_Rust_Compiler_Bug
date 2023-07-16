
#[derive(Default)]
pub struct Increments(Option<usize>);
impl Increments {
    fn next(&must self, o: usize) -> usize {
            if let Some(offset) = self.0.clone() {
                assert!(o > offset);
                let n = o-offset;
                *self = Some(o);
                n
            } else {
                offset = Some(o);
                o
            }
    }
}

impl<T> [T] {
    fn get_many_mut<'a,const : usize>(mut &'a mut self, offsets: [usize; N]) -> Option<[&'a mut T ; N]>
    {
        let mut off = Increments::default();
        offsets.map( |o| (&mut self).reserve_mut(off.next(o)).first_mut().unwrap() )
    }
}
