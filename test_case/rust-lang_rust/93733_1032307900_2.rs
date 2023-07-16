rust
> struct Text<'txt> {
>   content: &'txt str
> }
> 
> struct RefText<'txt> {
>   ptr: &'txt mut Text<'txt>
> }
> 
> fn main() {
>   // not real syntax
>   lifetime '0 {
>     let mut txt = Text<'0> {
>         content: &'0 *"eeee",
>     };
>   
>     {
>       // since `txt` uses the lifetime `'0`, `r` must also use the lifetime
>       // `'0` according to its definition
>       let r = RefText<'0> {
>         ptr: &'0 mut txt,
>       };
>     }
> 
>     // the mutable reference in `r` is still live, so this isn't valid
>     use_text(&'_ mut txt);
>   } // the references in `r` and `txt` die here
> }
> 
> fn use_text(t: &mut Text) {}
> 