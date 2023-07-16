
trait FromStructReader<'a> { }
trait ResponseHook {
     fn get<'a, T: FromStructReader<'a>>(&'a self);
}
fn foo(res : Box<ResponseHook>) { res.get }
fn main() {}
