
main.rs
pub mod foreign1;
pub mod foreign2;
mod foreign3;

foreign1.rs
extern "C" foreignabi1() -> int {
     5
}

foreign2.rs
pub extern "C" foreignabi2() -> int {
     5
}

foreign3.rs
pub extern "C" foreignabi3() -> int {
     5
}
