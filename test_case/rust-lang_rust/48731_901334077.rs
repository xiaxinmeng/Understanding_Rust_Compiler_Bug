rust
> fn slice_shift_char(a: &str) -> Option<(char, &str)> {
>     let mut chars = a.chars();
>     chars.next().map(|c| (c, chars.as_str()))
> }
> 
> fn main() {
>     assert_eq!(slice_shift_char("hello"), Some(('h', "ello")));
>     assert_eq!(slice_shift_char("ĺḿńóṕ"), Some(('ĺ', "ḿńóṕ")));
>     assert_eq!(slice_shift_char(""), None);
> }
> 