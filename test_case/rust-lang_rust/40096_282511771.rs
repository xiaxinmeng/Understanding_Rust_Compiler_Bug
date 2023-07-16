rust
> // Here, the String created will be dropped immediately, as it’s not bound:
> 
> let _ = String::from("  hello  ").trim();
> 