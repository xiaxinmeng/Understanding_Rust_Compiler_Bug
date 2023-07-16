rust
fn difference_iter<'a, 'ref_, T>(
    set_a: &'ref_ HashSet<&'a T>,
    set_b: &'ref_ HashSet<&T>,
) -> impl Iterator<Item = &'a T> + 'ref_
where
    T: std::cmp::Eq + std::hash::Hash + ?Sized,
    'a: 'ref_,
{
    set_a.iter().filter(move |a| !set_b.contains(*a)).copied()
}
