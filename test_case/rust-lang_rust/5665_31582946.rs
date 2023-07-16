
trait U {}
trait V : U {}
fn foo<'a>(a : &'a V)-> &'a U   {
    a as &U
}
