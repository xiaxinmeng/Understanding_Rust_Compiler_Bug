rust
> for value in 0..=u8::MAX {
>     let uppercase = (b'A'..=b'Z').contains(&value);
>     let lowercase = (b'a'..=b'z').contains(&value);
>     let numeric = (b'0'..=b'9').contains(&value);
>     let control = (0..=0x1f).contains(&value) || value == 0x7f;
>     let graphic = (b'!'..=b'~').contains(&value);
>     let hexdigit = numeric | (b'A'..=b'F').contains(&value) | (b'a'..=b'f').contains(&value);
>                                                                                               
>     assert_eq!(value.is_ascii(), (0..128).contains(&value));
>     assert_eq!(value.is_ascii_alphabetic(), uppercase | lowercase);
>     assert_eq!(value.is_ascii_alphanumeric(), uppercase | lowercase | numeric);
>     assert_eq!(value.is_ascii_control(), control);
>     assert_eq!(value.is_ascii_digit(), numeric);
>     assert_eq!(value.is_ascii_graphic(), graphic);
>     assert_eq!(value.is_ascii_hexdigit(), hexdigit);
>     assert_eq!(value.is_ascii_lowercase(), lowercase);
>     assert_eq!(value.is_ascii_uppercase(), uppercase);
> }
> 