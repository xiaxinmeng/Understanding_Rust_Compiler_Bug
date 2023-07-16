rust
> pub fn is_char_boundary(s: &str, index: usize) -> bool {
>     match s.as_bytes().get(index) {
>         None => false,
>         Some(_) if index == 0 || index == s.len() => true,
>         Some(&b) => (b as i8) >= -0x40,
>     }
> }
> 