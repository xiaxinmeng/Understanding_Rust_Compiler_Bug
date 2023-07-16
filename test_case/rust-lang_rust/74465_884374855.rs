rust
> impl Spam {
>   fn get_eggs(&self) -> &Eggs {
>     self.eggs.get_with(|| Eggs::cook())
>   }
> }
> 