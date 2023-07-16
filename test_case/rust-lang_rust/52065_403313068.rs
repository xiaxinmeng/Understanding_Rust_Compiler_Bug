rust
fn advance_n_and_return_first<I>(iter: &mut I, total_step: usize) -> Option<I::Item>
where
    I: Iterator,
{
    let next = iter.next();
    if total_step > 1 {
        iter.nth(step-2);
    }
    next
}
