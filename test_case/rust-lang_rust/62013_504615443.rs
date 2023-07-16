rust
>pub fn foo2_desugered(mut a: &mut Demo) {
>   {
>        let mut _t1 = &mut a.foo;
>        // match _t1 {
>        //     Some(_1) => a = _1 ,
>        //     None => ()
>        // }
>    }
>    a.foo = None;
>}
>