rust
> pub struct SectionEntry<'a> {
>     data: Box<dyn SectionData + 'a>,
>     // all the private fields you want
> }
> 
> pub struct SectionMut<'a> {
>     entry: &'a mut SectionEntry<'a>,
> }
> 