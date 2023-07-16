rust
trait MyServer {
    async (: 'static) fn foo(&mut self) -> Result<(),String>;
    async (: 'static) fn bar(&mut self) -> Result<(),String>;
}
