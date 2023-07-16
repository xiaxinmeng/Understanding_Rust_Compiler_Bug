
04:24 < reem> Really the issue is that impls for Box<X> do 
              not apply to Box<X + Y>, which really just 
              looks like a bug.
04:24 < reem> I think there is already agreement that that 
              is the desired semantics, though I could be 
              wrong.
