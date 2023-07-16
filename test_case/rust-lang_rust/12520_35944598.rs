
trait Ord {
    fn lt -> bool { self.cmp(other) == Less }
    fn le -> bool { self.cmp(other) != Greater }
    fn gt -> bool { self.cmp(other) == Greater }
    fn ge -> bool { self.cmp(other) != Less }
    fn cmp -> Ordering;
}

trait Eq {
    fn eq -> bool { self.equals(other) }
    fn ne -> bool { !self.eq(other) }
    fn equals -> bool;
}

impl Ord for f32 {
    fn lt -> bool { *self < *other }
    ...
    fn cmp -> Ordering { transmute::<&f32,&i32>(self).cmp(transmute(other)) }
}

impl Eq for f32 {
    fn eq -> bool { *self == *other }
    fn ne -> bool { *self == *other }
    fn equals -> bool { self.cmp(other) == Equal }
}
