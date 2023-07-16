rust
pub fn clone(it: &mut ()) -> &mut ()
where
    for<'any> &'any mut (): Clone,
{
    it.clone()
}
