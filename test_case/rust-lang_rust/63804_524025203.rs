rust
> // server connects and is told counter starts at 1
> server calls arm(TokenStream(1), TokenStream(2))
>   TokenStream::new() returns TokenStream(3)
>   TokenStream(3) ends up in TLS (leaking it)
>   TokenStream(2) is returned
> // server still knows about TokenStream(3)
> // server disconnects (taking TokenStream(3)'s actual data with it)
> 