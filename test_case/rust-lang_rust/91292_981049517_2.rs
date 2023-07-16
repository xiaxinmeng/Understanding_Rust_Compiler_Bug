rust
pub struct SectionEntry
{
    data: Box<dyn SectionData>,
    /* some other private fields in here */
}
pub struct SectionMut<'a>
{
    entry: &'a mut SectionEntry
}
