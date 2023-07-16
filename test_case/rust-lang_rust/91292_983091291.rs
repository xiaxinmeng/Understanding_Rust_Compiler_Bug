rust
> struct SectionDataWithReference<'a>(&'a u8);
> impl SectionData for SectionDataWithReference<'_> { ... }
> 
> let num = 5;
> let mut entry = SectionEntry {
>     data: Box::new(SectionDataWithReference(&num)),
> };
> let sm = SectionMut {
>     entry: &mut entry,
> };
> 