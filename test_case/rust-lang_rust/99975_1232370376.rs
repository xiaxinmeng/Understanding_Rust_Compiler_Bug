rust
> #![feature(let_else)]
> 
> fn return_result() -> Option<String> {
>     Some("ok".to_string())
> }
> 
> fn start() -> String {
>     let Some(content) = return_result() else {
>         return "none".to_string()
>     };
> 
>     content
> }
> 
> fn main() {
>     start();
> }
> 