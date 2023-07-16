rust
> impl<'a> SectionMut<'a> {
>     pub fn open(&mut self) -> Option<&mut (dyn SectionData + 'a)> {
> 