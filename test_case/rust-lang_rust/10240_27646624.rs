
trait Clone {
    ...
    fn clone_into(&self, other: &mut Self) {
        *other = self.clone();
    }
}
...
b.clone_into(a)
