 rust
// m.rs
extern crate n;

#[cfg(version1a)] use n::C;
#[cfg(version2a)] use n as local_n;
#[cfg(version3a)] use n;

#[cfg(version1a)] pub fn read() -> i8 { C }
#[cfg(version2a)] pub fn read() -> i8 { local_n::C }
#[cfg(version3a)] pub fn read() -> i8 { n::C }
#[cfg(version4a)] pub fn read() -> i8 { ::n::C }
#[cfg(version5a)] pub fn read() -> i8 { n::C }
#[cfg(version6a)] pub fn read() -> i8 { self::n::C }

#[cfg(any(version1b,
          version2b,
          version3b,
          version4b,
          version5b,))]
pub fn read() -> i8 { sub_m::read() }

mod sub_m {
    #[cfg(version1b)] use n::C;
    #[cfg(version2b)] use n as local_n;
    #[cfg(version3b)] use n;

    #[cfg(version1b)] pub fn read() -> i8 { C }
    #[cfg(version2b)] pub fn read() -> i8 { local_n::C }
    #[cfg(version3b)] pub fn read() -> i8 { n::C }
    #[cfg(version4b)] pub fn read() -> i8 { ::n::C }
    #[cfg(version5b)] pub fn read() -> i8 { super::n::C }
}

pub fn main() {
    println!("n::C is {}", read());
}
