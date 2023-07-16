 rust
enum Error {
    Error
}

struct ConnWrap(Conn);
impl ::std::ops::Deref for ConnWrap {
    type Target=Conn;
    fn deref(&self) -> &Conn { &self.0 }
}

struct App;
impl App {
    fn conn(&self) -> Result<ConnWrap, Error> { Ok(ConnWrap(Conn)) }
}

struct Conn;
impl Drop for  Conn {
    fn drop(&mut self) { println!("drop") }
}

fn accessible_sites(app: &App) -> Result<Vec<i32>, Error>
{
    let conn = &*try!(app.conn());
    return Ok(vec![]);
}

fn main() {
    accessible_sites(&App);
}
