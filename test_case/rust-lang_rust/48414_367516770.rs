
PM 052135 <misdreavus> oh, i see
PM 052143 <misdreavus> the docs are referring to a trait that hasn't been loaded yet
PM 052320 <misdreavus> when the trait is local, this is fine, because all that info is still in quick access
PM 052332 <misdreavus> but because the trait is remote, we need to pull it in
PM 052356 <misdreavus> but! we were already pulling in a remote trait
PM 052501 <misdreavus> clean::inline::record_extern_trait, farther up the stack, is waiting to insert a clean::Trait into external_traits
PM 052552 <misdreavus> so now i need to manually release that borrow once i know that i need to calculate it
