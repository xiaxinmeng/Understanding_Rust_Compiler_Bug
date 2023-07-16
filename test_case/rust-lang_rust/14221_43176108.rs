 rust
pub mod a {
    pub enum En { A,
                  B,
    #[cfg(field)] C(int) }

    pub fn key(e: En) -> StrBuf {
        match e { A => "A",
                  B => "B",
    #[cfg(field)] C(_) => "C",
        }.to_strbuf()
    }

    #[cfg(inner)]
    pub mod b {
        use a::En;
        pub fn key(e: En) -> StrBuf {
            match e { A => "A",
                      B => "B",
        #[cfg(field)] C(_) => "C",
            }.to_strbuf()
        }
    }
}

#[cfg(inner)]
fn main() {
    println!("Hello World {}", a::b::key(a::A));
}

#[cfg(not(inner))]
fn main() {
    println!("Hello World {}", a::key(a::A));
}
