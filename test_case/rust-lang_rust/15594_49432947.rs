
>     pub fn baz(x: ::Option<uint>) -> ::Option<uint> {
>         use None;
>         let z = match x {
>             ::Some(y) if y > 10 => create_exprs!(y; 2 16 128),
>             ::Some(z) => create_exprs!(z; 2 16 128),
>             None => create_exprs!(0u; 2 16 128)
>         };
>         bar::baz(bar::baz(foo::baz(foo::baz(::Some(z)))))
>     }
> }}
> 