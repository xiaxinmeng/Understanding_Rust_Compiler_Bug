rust
#![feature(unboxed_closures)]

trait MyTrait { }
impl<'a> MyTrait for &'a i32 { }

trait MyTraitOutput<Args>: Fn<Args> { }
impl<Args, F> MyTraitOutput<Args> for F
where
    F: Fn<Args>,
    <F as FnOnce<Args>>::Output: MyTrait
{ }

fn example<F: for<'a> Fn<(&'a i32,)>>(_f: F)
where
    F: for<'a> MyTraitOutput<(&'a i32,)>
{
    //call the closure and store its result. then, pass the result to methods on 
    //MyTraitOutput which forward to methods on MyTrait
}
