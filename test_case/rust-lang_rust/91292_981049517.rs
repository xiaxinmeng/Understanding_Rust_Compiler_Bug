rust
> pub struct SectionMut<'a> {
>     data: &'a mut Option<Box<dyn SectionData + 'a>>,
> }
> 