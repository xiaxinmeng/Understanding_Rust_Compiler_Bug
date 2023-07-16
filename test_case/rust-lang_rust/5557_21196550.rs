
#[link(name = "fivefivefiveseven",
       vers = "0.1")];
#[crate_type = "lib"];

pub enum Event {
    KeyEvent { foo: u8 }
}
