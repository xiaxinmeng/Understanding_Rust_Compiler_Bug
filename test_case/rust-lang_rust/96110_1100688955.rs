rust
> const fn to_ascii_digit(self, radix: u8) -> Option<Self>;
> // or 
> const fn from_ascii_digit(self, radix: u8) -> Option<Self>;
> // I prefer this one cause self is admited to be a ascii_digit we when to convert to u8 (as the issue is looking for)
> // to_ascii_digit look more like the reverse of what the issue is looking for
> 